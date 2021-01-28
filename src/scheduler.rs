use crate::file::File;
use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::borrow::BorrowMut;
use std::io::Result;
use threadpool::ThreadPool;

struct Udp(&'static str, &'static str);
const IP_PORT: Udp = Udp("127.0.0.1", "12865");
//const IP : &str = "127.0.0.1";  rust中定义字符串常量，不能使用String
const UDP_SOCKET: Token = Token(0);

pub struct Scheduler {
    task_count: usize,
    files: File,
    thread_size: usize,
    thread_pool: ThreadPool,
    poll: Option<Poll>,
    events: Option<Events>,
    listen_fd: Option<UdpSocket>,
}

impl Scheduler {
    pub fn new(t_n: usize) -> Scheduler {
        Scheduler {
            task_count: 0,
            files: File::new(),
            thread_size: t_n,
            thread_pool: ThreadPool::new(t_n),
            poll: None,
            events: None,
            listen_fd: None,
        }
    }

    pub fn initial(&mut self) {
        match self.files.get_counter_files() {
            Ok(_) => {}
            Err(_) => {
                panic!("get counter files fail")
            }
        }

        self.poll = match Poll::new() {
            Ok(poll) => Some(poll),
            Err(e) => {
                panic!("new poll fail:{}", e)
            }
        };

        self.events = Some(Events::with_capacity(self.thread_size + 1));

        let ip_port = format!("{}:{}", IP_PORT.0, IP_PORT.1).parse().unwrap();
        self.listen_fd = match UdpSocket::bind(ip_port) {
            Ok(socket) => Some(socket),
            Err(e) => {
                panic!("create udp fail:{}", e)
            }
        };

        self.poll.as_ref().unwrap().registry().register(
            self.listen_fd.as_mut().unwrap(),
            UDP_SOCKET,
            Interest::READABLE,
        );
    }

    pub fn event_loop(&mut self) -> Result<()> {
        loop {
            self.poll
                .as_mut()
                .unwrap()
                .poll(self.events.as_mut().unwrap(), None)?;

            for event in self.events.as_ref().unwrap().iter(){
                match  event.token(){
                    UDP_SOCKET =>{
                        match self.listen_fd.as_ref().unwrap().recv(){
                            Ok(_) => {

                            }
                            Err(_) =>{

                            }
                        }
                    }
                    _ =>{

                    }
                }
            }


        }
    }
}
