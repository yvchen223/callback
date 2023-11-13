use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Runtime {
    callbacks: RefCell<HashMap<usize, Box<dyn FnOnce() -> ()>>>,
    next_id: RefCell<usize>,
    event_sender: Sender<usize>,
    event_receiver: Receiver<usize>,
}

impl Runtime {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Runtime {
            callbacks: RefCell::new(HashMap::new()),
            next_id: RefCell::new(1),
            event_sender: tx,
            event_receiver: rx,
        }
    }

    pub fn run(&self, f: fn()) {
        f();
        for event_id in &self.event_receiver {
            let cb = self.callbacks.borrow_mut().remove(&event_id).unwrap();
            cb();
            if self.callbacks.borrow().is_empty() {
                break;
            }
        }
    }

    fn next_id(&self) -> usize {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    pub fn set_cb(&self, cb: impl FnOnce() + 'static) -> Result<usize, String> {
        let id = self.next_id();
        self.callbacks.borrow_mut().insert(id, Box::new(cb));
        Ok(id)
    }

    pub fn event_sender(&self) -> Sender<usize> {
        self.event_sender.clone()
    }
}
