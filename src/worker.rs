use std::sync::mpsc::{Sender, Receiver};

pub struct Woker{
    worker_id : i32,
    worker_name : String,
    task_queue : (Sender<String>, Receiver<String>)
}