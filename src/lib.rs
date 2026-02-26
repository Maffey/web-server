use std::thread;

#[derive(Debug, Clone)]
pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        Self::build(size).expect("ThreadPool size must be greater than zero!")
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let mut workers = Vec::with_capacity(size);

        for worker_id in 0..size {
            workers.push(Worker::new(worker_id));
        }

        Ok(ThreadPool { workers })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    // In a production thread pool implementation,
    // you’d likely want to use std::thread::Builder
    // and its spawn method that returns Result instead,
    // to avoid panics if system can't spawn a thread.
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {
            // do some work here
        });

        Worker { id, thread }
    }
}

