use crate::file::File;
use crate::file::FileType;
use crate::task::{TaskBase, TaskCPP, TaskNone, TaskShell};
use crate::worker::Worker;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::Result;
use std::io::{self, Read, Write};
use std::sync::atomic::Ordering::AcqRel;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct Tcp(pub(crate) &'static str, pub(crate) &'static str);
pub const IP_PORT: Tcp = Tcp("127.0.0.1", "12865");
//const IP : &str = "127.0.0.1";  rust中定义字符串常量，不能使用String
const SERVER: Token = Token(0);
pub const DISPATCH_SIZE: usize = 5;

pub struct Scheduler {
    files: File,
    thread_size: usize,
    thread_pool: ThreadPool,
    poll: Option<Poll>,
    events: Option<Events>,
    listen_fd: Option<TcpListener>,
    txs: Vec<Sender<Box<dyn TaskBase + Send>>>,
    rxs: Vec<Receiver<Box<dyn TaskBase + Send>>>,
    connections: HashMap<Token, TcpStream>,
}

impl Scheduler {
    pub fn new(t_n: usize) -> Scheduler {
        Scheduler {
            files: File::new(),
            thread_size: t_n,
            thread_pool: ThreadPool::new(t_n),
            poll: None,
            events: None,
            listen_fd: None,
            txs: Vec::with_capacity(t_n),
            rxs: Vec::with_capacity(t_n),
            connections: HashMap::new(),
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

        for i in 0..self.thread_size {
            let (tx, rx) = channel();
            self.txs.push(tx);
            self.rxs.push(rx);
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
        for i in 0..self.thread_size {
            let mut rx = self.rxs.remove(0);
            self.thread_pool.execute(move || {
                Worker::do_work(&mut rx);
            });
        }

        return Ok(());
    }

    pub fn dispatch(&mut self) {}

    pub fn event_loop(&mut self) -> Result<()> {
        let mut buffer = Vec::with_capacity(128);
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

                        self.connections.insert(token, connection);
                    },
                    token => {
                        //派发任务
                        if let Some(connection) = self.connections.get_mut(&token) {
                            connection.read_to_end(&mut buffer);
                            let nodes = self.files.get_nodes();
                            if nodes.is_empty() {
                                return Ok(());
                            }
                            // https://www.reddit.com/r/rust/comments/bv90s7/temporary_value_dropped_while_borrowed/
                            //let mut worker = self.workers.lock().unwrap().get_mut(token.0).unwrap(); error

                            for _ in 0..DISPATCH_SIZE {
                                // https://stackoverflow.com/questions/51429501/how-do-i-conditionally-check-if-an-enum-is-one-variant-or-another
                                /*      if let FileType::CPP = v.t {
                                } else if let FileType::SHELL = v.t {
                                }*/
                                let node = nodes.remove(0);
                                let task_new = || -> Box<dyn TaskBase + Send + 'static> {
                                    if let FileType::CPP = node.t {
                                        Box::new(TaskCPP::new(node))
                                    } else if let FileType::SHELL = node.t {
                                        Box::new(TaskShell::new(node))
                                    } else {
                                        Box::new(TaskNone::new())
                                    }
                                };

                                //TODO match返回trait类型
                                /* let task = match v.t {
                                  FileType::CPP => Box::new(TaskCPP::new(v)),
                                  FileType::SHELL => Box::new(TaskShell::new(v)),
                                  _ => {}
                                };*/
                                match self.txs.get_mut(token.0 - 1).unwrap().send(task_new()){
                                    Ok(()) => {},
                                    Err(e) => {panic!("send task failed:{}", e)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
