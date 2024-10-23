use std::os::unix::thread;

pub struct ThreadPool {
    worekrs: Vec<Worker>,
    
};

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
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    
}