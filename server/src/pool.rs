use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use http_core::response::Response;
use http_parser::response::SerializeResponse;
use connection::ConnectionStream;
use http_parser::parse_req;
use routing::Router;

/// Protocol commucation among workers
pub(crate) enum Message {
    /// Handle requests
    NewJob(ConnectionStream),
    /// Shutdown a worker
    Shutdown,
}

/// A thread pool for workers management. 
pub(crate) struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Sends a message to an available worker for executing a job.
    pub(crate) fn execute(&self, message: Message) {
        self.sender.send(message).unwrap();
    }

    /// Creates a new thread pool.
    pub(crate) fn new(size: usize, router: Arc<Mutex<Router>>) -> Self {
        if size <= 0 {
            panic!("It's possible create zero or less threads");
        }
        
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let router = Arc::new(router);
        let mut workers = Vec::new();
        
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            let router = Arc::clone(&router);
            
            let handle = std::thread::spawn(move || loop {
                let request = receiver.lock().unwrap().recv().unwrap();

                match request {
                    Message::NewJob(mut stream) => {
                        let router = router.lock();
                        
                        let req = parse_req(stream.stream_mut()).unwrap();
                        let req_method = req.start_line().method();

                        if let Ok(router) = router {
                            let mut res = router.match_route(&req);
                            let mut res = res.serialize(req_method);

                            stream.send_response(&mut res); 
                        } else {
                            let mut res = Response::internal_error(Some(b"Internal Server Error"));
                            stream.send_response(&res.serialize(req_method));
                        }  
                    },
                    Message::Shutdown => break
                }
            });

            workers.push(handle);
        }

        Self { workers, sender }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let workers: Vec<JoinHandle<()>> = self.workers.drain(..).collect();

        for _ in &self.workers {
            self.sender.send(Message::Shutdown).unwrap();
        }
        
        for worker in workers {
            worker.join().unwrap();
        }
    }
}