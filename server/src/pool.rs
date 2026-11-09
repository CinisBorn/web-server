//! A thread pool for processing HTTP connections concurrently.
//!
//! The pool starts a fixed number of worker threads. Incoming connections
//! are submitted as jobs through a message channel, and workers process
//! those jobs using the shared router.
//!
//! The pool owns the worker threads and coordinates their shutdown when
//! it is dropped.

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use http_core::Method;
use http_core::response::{Response, ResponseData};

use http_parser::response::SerializeResponse;
use http_parser::{ParserState, parse_req};

use crate::RequestConfig;

use routing::Router;

type Workers = Vec<JoinHandle<()>>;
/// Shared receiver used by workers to retrieve pool messages.
type SharedReceiver = Arc<Mutex<Receiver<Message<TcpStream>>>>;

/// Protocol commucation among workers
pub(crate) enum Message<T> {
    /// Handle requests
    NewJob(T),
    /// Shutdown a worker
    Shutdown,
}

/// A thread pool for workers management.
pub(crate) struct ThreadPool {
    workers: Workers,
    sender: SyncSender<Message<TcpStream>>,
}

impl ThreadPool {
    /// Submits a message to the worker pool.
    ///
    /// # Panics
    ///
    /// Panics if all receivers have been dropped and the message cannot
    /// be sent.
    pub(crate) fn execute(&self, message: Message<TcpStream>) {
        use http_core::response::StatusCode::ServiceUnavailable;

        match self.sender.try_send(message) {
            Err(TrySendError::Full(Message::NewJob(mut stream))) => {
                let mut response = Response::new();
                response.set_status(ServiceUnavailable);
                _ = stream.write_all(&response.service_unavailable());
            }

            Err(TrySendError::Disconnected(_)) => {
                // The receiver has disconnected.
                // Log the failure or handle it appropriately.
            }

            _ => {}
        }
    }

    /// Creates a thread pool with the specified number of workers.
    ///
    /// All workers share the provided router.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of worker threads to create. Must be greater
    ///   than zero.
    /// * `router` - Router shared by all workers.
    ///
    /// # Panics
    ///
    /// Panics if `size` is zero.
    pub(crate) fn new(
        size: usize, 
        router: Arc<Router>,
        req_config: Arc<Option<RequestConfig>>
    ) -> Self {
        assert!(size > 0);

        let (sender, receiver) = sync_channel::<Message<TcpStream>>(size);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::new();

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            let router = Arc::clone(&router);
            let req_config = Arc::clone(&req_config);

            let handle = Self::spawn_thread(
                receiver, 
                router, 
                req_config
            );

            workers.push(handle);
        }

        Self { workers, sender }
    }

    /// Starts a worker that receives and processes pool messages.
    fn spawn_thread(
        rx: SharedReceiver, 
        router: Arc<Router>,
        req_config: Arc<Option<RequestConfig>>
    ) -> JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                let request = rx.lock().unwrap().recv().unwrap();

                match request {
                    Message::NewJob(mut stream) => {
                        Self::handle_connection(&mut stream, &router, &req_config);
                    }
                    Message::Shutdown => break,
                }
            }
        })
    }

    fn should_finish_worker(value: Option<&str>) -> bool {
        if let Some(value) = value {
            if value.to_lowercase() == "close" {
                return true 
            }

            return false
        } 

        false
    }
    
    fn finish_worker(stream: &mut TcpStream, res: &mut Response,method: &Method) {
        let mut headers = HashMap::new();
        
        headers.insert("Connection".to_string(), "close".to_string());  
        res.set_headers(headers);   
        
        _ = stream.write_all(&res.serialize(&method));
        stream.shutdown(std::net::Shutdown::Write).unwrap();
    }

    fn handle_connection(
        stream: &mut TcpStream, 
        router: &Arc<Router>,
        req_config: &Arc<Option<RequestConfig>>
    ) {
        loop {
            // The first as_ref returns a referento to Request 
            // Config (without the Arc wrapper). 
            // 
            // The second as_ref change &Option<T> to Option<&T>
            let config = req_config.as_ref().as_ref();
            let req = match parse_req(stream, config) {
                Ok(r) => r,
                Err(err) => {
                    let mut res = err.new_response_err();
                    
                    Self::finish_worker(
                        stream, 
                        &mut res,  
                        &Method::GET
                    );
                    
                    break
                },
            };

            let ParserState::Complete(req) = req else {
                break;
            };

            let req_method = req.start_line().method();
            
            let finish_work = req.connection();
            let finish_work = Self::should_finish_worker(finish_work);
            
            let router = router;
            let mut res = router.match_route(&req);

            if finish_work {
                Self::finish_worker(stream, &mut res, req_method);
                break;
            }

            _ = stream.write_all(&res.serialize(req_method));
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Shutdown).unwrap();
        }

        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}
