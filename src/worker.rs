use crate::task::Task;
use std::sync::mpsc::{channel, Receiver, Sender};
use mio::net::TcpStream;
use crate::scheduler::IP_PORT;
use std::io;
use std::io::Write;

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
        let addr = format!("{}:{}", IP_PORT.0, IP_PORT.1).parse().unwrap();
        let mut client = match TcpStream::connect(addr){
            Err(e) => panic!("connect fail: {}", e),
            Ok(client) => client
        };

        loop{
          //  let left_size = 0;
            // match client.write(&[left_size]){
             match client.write(b"Req"){
                Ok(_) =>{}
                Err(e) => println!("worker req fail:{}", e)
            }
            for task in &self.task_queue.1{


            }

        }

    }


}
