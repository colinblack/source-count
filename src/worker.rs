use crate::scheduler::IP_PORT;
use crate::task::{Task, TaskBase};
use mio::net::TcpStream;
use std::io;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::Deref;

pub struct Worker {
}

impl Worker {
    pub fn do_work(rx :&mut Receiver<Box<dyn TaskBase + Send>>) {
        let addr = format!("{}:{}", IP_PORT.0, IP_PORT.1).parse().unwrap();
        let mut client = match TcpStream::connect(addr) {
            Err(e) => panic!("connect fail: {}", e),
            Ok(client) => client,
        };

        loop {
            //  let left_size = 0;
            // match client.write(&[left_size]){
            match client.write(b"Req") {
                Ok(_) => {}
                Err(e) => println!("worker req fail:{}", e),
            }
            for task in rx.deref() {
                task.print_info();
            }
        }
    }
}
