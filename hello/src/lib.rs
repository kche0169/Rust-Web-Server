
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// pub struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);
        let mut workers = Vec::with_capacity(num);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..num {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
   fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker id: {id} got a job. Executing");
            job();
        });
        Worker {id, thread}
    }
}