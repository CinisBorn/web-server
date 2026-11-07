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
use http_core::response::{Response, ResponseData, StatusCode};

use http_parser::error::ParserErr;
use http_parser::response::SerializeResponse;
use http_parser::{ParserState, parse_req};

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
    pub(crate) fn new(size: usize, router: Arc<Router>) -> Self {
        assert!(size > 0);

        let (sender, receiver) = sync_channel::<Message<TcpStream>>(size);
        let receiver = Arc::new(Mutex::new(receiver));

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
    fn spawn_thread(rx: SharedReceiver, router: Arc<Router>) -> JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                let request = rx.lock().unwrap().recv().unwrap();

                match request {
                    Message::NewJob(mut stream) => {
                        Self::handle_connection(&mut stream, &router);
                    }
                    Message::Shutdown => break,
                }
            }
        })
    }

    fn finish_worker_job(
        stream: &mut TcpStream,
        res: &mut Response,
        closure: Option<&str>,
        method: &Method,
    ) -> bool {
        if let Some(value) = closure {
            if value == "close" {
                let mut headers = HashMap::new();

                headers.insert("Connection".to_string(), "close".to_string());

                res.set_headers(headers);

                _ = stream.write_all(&res.serialize(&method));
                stream.shutdown(std::net::Shutdown::Write).unwrap();

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_connection(stream: &mut TcpStream, router: &Arc<Router>) {
        loop {
            let req = match parse_req(stream, None) {
                Ok(r) => r,
                Err(err) => {
                    Self::send_error_and_close(stream, &err);
                    break
                },
            };

            let ParserState::Complete(req) = req else {
                break;
            };

            let req_method = req.start_line().method();
            let closure = req.connection();
            let router = router;

            let mut res = router.match_route(&req);

            let finish_work = Self::finish_worker_job(stream, &mut res, closure, req_method);

            if finish_work {
                break;
            }

            _ = stream.write_all(&res.serialize(req_method));
        }
    }
}

impl ThreadPool {
    /// Sends an HTTP response with information about the error cause, then
    /// closes the connection.
    fn send_error_and_close(
        stream: &mut TcpStream,
        err: &ParserErr
    ) {
        use std::io::ErrorKind;
        use http_core::response::StatusCode as Code;
        
        let mut res = Response::new();

        match err {
            ParserErr::ConflictingBodyFraming => res.set_status(Code::BadRequest),
            ParserErr::ContentTooLarge(_) => res.set_status(Code::ContentTooLarge),
            ParserErr::InvalidInput(_) => res.set_status(Code::BadRequest),
            ParserErr::InvalidTransferEncoding(_) => res.set_status(Code::NotImplemented),
            ParserErr::UnsupportedHttpVersion(_) => res.set_status(Code::HttpVersionNotSupported),
            ParserErr::Io(err) => match err.kind() {
                ErrorKind::WouldBlock => res.set_status(Code::RequestTimeout),
                ErrorKind::TimedOut => res.set_status(Code::RequestTimeout),
                _ => res.set_status(Code::BadRequest),
            },
            ParserErr::TooLong(code) => match code {
                431 => res.set_status(Code::RequestHeaderFieldsTooLarge),
                414 => res.set_status(Code::UriTooLong),
                _ => res.set_status(Code::BadRequest),
            },
            ParserErr::TooManyItems(code) => match code {
                431 => res.set_status(Code::RequestHeaderFieldsTooLarge),
                _ => res.set_status(Code::BadRequest)
            }
        }

        Self::finish_worker_job(
            stream, 
            &mut res, 
            Some("close"), 
            &Method::GET
        );
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
