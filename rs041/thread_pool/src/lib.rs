#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

#[allow(dead_code)]
pub struct ThreadPool {
    // threads: Vec<std::thread::JoinHandle<()>>
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
struct Worker {
    id: usize, 
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move|| loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        // let thread = std::thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() { // while let (and if let and match) does not drop temporary (LockResult<MutexGuard<T>> to unlock) values until the end of the associated block.
        //         println!("Worker {} got a job; executing.", id);
        //         job();
        //     }
        // });
        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool. 
    /// The size is the number of threads in the pool. 
    /// # Panics
    /// The `new` function will panic if the size is zero. 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}