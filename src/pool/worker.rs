use crate::handler::Handler;
use crate::Config;

use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

enum Message {
    Stream(TcpStream),
    Stop,
}

pub struct Worker {
    tx: Sender<Message>,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(config: Arc<Config>, id: usize, ready_tx: &Sender<usize>) -> Worker {
        let ready_tx = ready_tx.clone();
        let (tx, rx) = mpsc::channel();

        let thread = Some(thread::spawn(move || {
            println!("worker {}: start", id);
            loop {
                ready_tx.send(id).unwrap();
                match rx.recv().unwrap() {
                    Message::Stream(s) => Handler::new(config.as_ref(), s, id).handle(),
                    Message::Stop => break,
                }
            }
            println!("worker {}: stop", id);
        }));

        Worker { tx, thread }
    }

    pub fn handle(&self, s: TcpStream) {
        self.tx.send(Message::Stream(s)).unwrap();
    }

    pub fn stop(&self) {
        self.tx.send(Message::Stop).unwrap();
    }

    pub fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
