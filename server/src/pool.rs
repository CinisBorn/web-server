use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use http_parser::response::SerializeResponse;
use connection::ConnectionStream;
use http_parser::parse_req;
use routing::Router;

/// Protocol commucation among workers
pub enum Message {
    /// Handle requests
    NewJob(ConnectionStream),
}

/// A thread pool for workers management. 
pub struct ThreadPool {
    workers: Vec<JoinHandle<ConnectionStream>>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Sends a message to an available worker for executing a job.
    pub fn execute(&self, message: Message) {
        self.sender.send(message).unwrap();
    }

    /// Creates a new thread pool.
    pub fn new(size: usize, router: Arc<Mutex<Router>>) -> Self {
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
                let request  = receiver.lock().unwrap().recv().unwrap();
                
                match request {
                    Message::NewJob(mut stream) => {
                        let req = parse_req(stream.stream_mut()).unwrap();
                        let router = router.lock().unwrap();
                        let mut res = router.match_route(&req);
                        
                        stream.send_response(
                            &mut res.serialize(&req.start_line().method())
                        );
                    },
                }
            });

            workers.push(handle);
        }

        Self { workers, sender }
    }
}