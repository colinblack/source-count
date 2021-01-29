use crate::file::File;
use crate::worker::Worker;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::borrow::BorrowMut;
use std::io::Result;
use std::io::{self, Read, Write};
use threadpool::ThreadPool;

struct Udp(&'static str, &'static str);
const IP_PORT: Udp = Udp("127.0.0.1", "12865");
//const IP : &str = "127.0.0.1";  rust中定义字符串常量，不能使用String
const SERVER: Token = Token(0);

pub struct Scheduler {
    task_count: usize,
    files: File,
    thread_size: usize,
    thread_pool: ThreadPool,
    poll: Option<Poll>,
    events: Option<Events>,
    listen_fd: Option<TcpListener>,
    workers: Vec<Worker>,
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
            workers: Vec::with_capacity(t_n + 2),
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

        for v in 2..(self.thread_size + 2) {
            self.workers.push(Worker::new());
        }

        self.events = Some(Events::with_capacity(self.thread_size + 1));

        let ip_port = format!("{}:{}", IP_PORT.0, IP_PORT.1).parse().unwrap();
        self.listen_fd = match TcpListener::bind(ip_port) {
            Ok(socket) => Some(socket),
            Err(e) => {
                panic!("create udp fail:{}", e)
            }
        };

        self.poll.as_ref().unwrap().registry().register(
            self.listen_fd.as_mut().unwrap(),
            SERVER,
            Interest::READABLE,
        );
    }

    fn next(current: &mut Token) -> Token {
        let next = current.0;
        current.0 += 1;
        Token(next)
    }

    pub fn run(&mut self) -> Result<()> {

        for _ in 0..self.thread_size {
            self.thread_pool.execute(|| {
                
            });
        }

        return Ok(());
    }

    pub fn event_loop(&mut self) -> Result<()> {
        let mut unique_token = Token(SERVER.0 + 1);
        loop {
            self.poll
                .as_mut()
                .unwrap()
                .poll(self.events.as_mut().unwrap(), None)?;

            for event in self.events.as_ref().unwrap().iter() {
                match event.token() {
                    SERVER => loop {
                        let (mut connection, address) =
                            match self.listen_fd.as_ref().unwrap().accept() {
                                Ok((connection, address)) => (connection, address),
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            };

                        let token = Scheduler::next(&mut unique_token);
                        self.poll.as_ref().unwrap().registry().register(
                            //将TCP连接加入poll
                            &mut connection,
                            token,
                            Interest::READABLE,
                        )?;
                        if let Some(worker) = self.workers.get_mut(token.0) {
                            worker.set_worker_id(token.0 as i32);
                        }
                    },
                    token => {
                        //派发任务
                    }
                }
            }
        }
    }
}
