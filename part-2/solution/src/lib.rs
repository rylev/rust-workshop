use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(worker_count);

        for n in 0..worker_count {
            workers.push(Worker::new(n, Arc::clone(&rx)))
        }

        ThreadPool {
            _workers: workers,
            tx,
        }
    }

    pub fn execute<F>(&self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(callback))
            .expect("Thread shut down too early");
    }
}

struct Worker {
    _id: usize,
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let result = rx.lock().unwrap().recv();
            match result {
                Ok(rx) => {
                    println!("Worker {} got a job; executing.", id);
                    rx.call()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _handle: handle,
        }
    }
}

trait BoxedFn {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> BoxedFn for F {
    fn call(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn BoxedFn + Send + 'static>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threadpool() {
        let pool = ThreadPool::new(4);

        pool.execute(|| {
            println!("Number 1");
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        pool.execute(|| {
            println!("Number 2");
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        pool.execute(|| {
            println!("Number 3");
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        pool.execute(|| {
            println!("Number 4");
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
