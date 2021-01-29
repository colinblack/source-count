use crate::task::Task;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Worker {
    worker_id: i32,
    task_queue: (Sender<Task>, Receiver<Task>),
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            worker_id: -1,
            task_queue: channel(),
        }
    }

    pub fn set_worker_id(&mut self, id: i32) {
        self.worker_id = id;
    }

    pub fn do_work(&mut self){

    }


}
