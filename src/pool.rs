use crate::Config;

mod worker;
use worker::Worker;

use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};

pub struct Pool {
    workers: Vec<Worker>,
    rx: Receiver<usize>,
}

impl Pool {
    pub fn init(config: Config) -> Self {
        let config = Arc::new(config);
        let mut workers = Vec::with_capacity(config.worker_number);
        let (tx, rx) = mpsc::channel();

        for i in 0..config.worker_number {
            let config = config.clone();
            let w = Worker::new(config, i, &tx);
            workers.push(w);
        }

        Self { workers, rx }
    }

    pub fn handle(&self, stream: TcpStream) {
        let i = self.rx.recv().unwrap();
        println!("worker {}: ready", i);
        self.workers[i].handle(stream);
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for worker in &self.workers {
            worker.stop();
        }
        for worker in &mut self.workers {
            worker.join();
        }
    }
}
