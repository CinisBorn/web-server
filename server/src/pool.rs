//! A thread pool for processing HTTP connections concurrently.
//!
//! The pool starts a fixed number of worker threads. Incoming connections
//! are submitted as jobs through a message channel, and workers process
//! those jobs using the shared router.
//!
//! The pool owns the worker threads and coordinates their shutdown when
//! it is dropped.

use std::sync::mpsc::{self, Receiver, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use connection::ConnectionStream;
use http_core::response::{Response, ResponseData};
use http_parser::parse_req;
use http_parser::response::SerializeResponse;
use routing::Router;

type Workers = Vec<JoinHandle<()>>;
/// Shared receiver used by workers to retrieve pool messages.
type SharedReceiver = Arc<Mutex<Receiver<Message>>>;

/// Protocol commucation among workers
pub(crate) enum Message {
    /// Handle requests
    NewJob(ConnectionStream),
    /// Shutdown a worker
    Shutdown,
}

/// A thread pool for workers management.
pub(crate) struct ThreadPool {
    workers: Workers,
    sender: mpsc::SyncSender<Message>,
}

impl ThreadPool {
    /// Submits a message to the worker pool.
    ///
    /// # Panics
    ///
    /// Panics if all receivers have been dropped and the message cannot
    /// be sent.
    pub(crate) fn execute(&self, message: Message) {
        use http_core::response::StatusCode::ServiceUnavailable;

        match self.sender.try_send(message) {
            Ok(_) => {},
            Err(TrySendError::Full(Message::NewJob(mut stream))) => {
                let mut response = Response::new();
                
                response.set_status(ServiceUnavailable);
                stream.send_response(&response.service_unavailable());
            },
            _ => {
                /*
                 * I will not handle it why the "sender" or "receiver" is 
                 * dropped when the server shutdowns. 
                 * 
                 * It's unlike a TrySendError::Disconnected is returned in the 
                 * current code design. 
                 */
            }
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
    pub(crate) fn new(size: usize, router: Arc<Mutex<Router>>) -> Self {
        if size <= 0 {
            panic!("It's possible create zero or less threads");
        }

        let (sender, receiver) = mpsc::sync_channel::<Message>(size);
        let receiver = Arc::new(Mutex::new(receiver));
        let router = router;
        let mut workers = Vec::new();

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            let router = Arc::clone(&router);

            let handle = Self::spawn_thread(receiver, router);

            workers.push(handle);
        }

        Self { workers, sender }
    }
    
    /// Starts a worker that receives and processes pool messages.
    fn spawn_thread(rx: SharedReceiver, router: Arc<Mutex<Router>>) -> JoinHandle<()> {
        let handle = std::thread::spawn(move || {
            loop {
                let request = rx.lock().unwrap().recv().unwrap();

                match request {
                    Message::NewJob(mut stream) => {
                        let router = router.lock();

                        let req = parse_req(stream.stream_mut()).unwrap();
                        let req_method = req.start_line().method();

                        if let Ok(router) = router {
                            let mut res = router.match_route(&req).serialize(req_method);

                            stream.send_response(&mut res);
                        } else {
                            let res: Option<&[u8]> = Some(b"Internal Server Error");
                            let res = Response::internal_error(res).serialize(req_method);

                            stream.send_response(&res);
                        }
                    }
                    Message::Shutdown => break,
                }
            }
        });

        handle
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
