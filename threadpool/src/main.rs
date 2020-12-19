use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

type Thunk = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    jobs: Sender<Thunk>
}

impl ThreadPool {
    fn new(threads: usize) -> Self {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..threads {
            spawn_in_pool(rx.clone());
        }

        ThreadPool { jobs: tx }
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        self.jobs.send(Box::new(job)).unwrap();
    }
}

fn spawn_in_pool(jobs: Arc<Mutex<Receiver<Thunk>>>) {
    thread::spawn(move || {
        loop {
            let message = {
                let lock = jobs.lock().unwrap();
                lock.recv()
            };

            match message {
                Ok(job) => job(),
                Err(..) => break
            }
        }
    });
}

fn main() {
    let threads: usize = 3;
    let pool = ThreadPool::new(threads);

    let (tx, rx) = channel();
    for _ in 0..threads {
        let tx = tx.clone();
        pool.spawn(move || {
            tx.send(1).unwrap()
        });
    }

    println!("{}", rx.iter().take(threads).fold(0, |a, b| a + b));
}