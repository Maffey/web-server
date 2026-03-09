use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for worker_id in 0..size {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, closure: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);

        self.sender.send(job).expect("Failed to send job to worker threads!");
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
    fn new(id: usize, receiver:  Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver
                    .lock()
                    .expect("Mutex is in poisoned state!")
                    .recv()
                    .expect("Failed to receive job!");

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}

