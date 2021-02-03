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
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct Tcp(pub(crate) &'static str, pub(crate) &'static str);
pub const IP_PORT: Tcp = Tcp("127.0.0.1", "12865");
//const IP : &str = "127.0.0.1";  rust中定义字符串常量，不能使用String
const SERVER: Token = Token(0);
pub const DISPATCH_SIZE: usize = 5;

pub struct Scheduler {
    task_finish: usize, //已派发数量
    files: File,
    thread_size: usize,
    thread_pool: ThreadPool,
    poll: Option<Poll>,
    events: Option<Events>,
    listen_fd: Option<TcpListener>,
    workers: Arc<Mutex<Vec<Worker>>>, //Arc智能指针，用于多线程间共享变量, Arc不可变，需使用Mutex才能可变
    connections: HashMap<Token, TcpStream>,
}

impl Scheduler {
    pub fn new(t_n: usize) -> Scheduler {
        Scheduler {
            task_finish: 0,
            files: File::new(),
            thread_size: t_n,
            thread_pool: ThreadPool::new(t_n),
            poll: None,
            events: None,
            listen_fd: None,
            workers: Arc::new(Mutex::new(Vec::with_capacity(t_n + 2))),
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

        for v in 2..(self.thread_size + 2) {
            self.workers.lock().unwrap().push(Worker::new());
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
        for i in 2..(self.thread_size + 2) {
            let workers = Arc::clone(&self.workers);
            let ccc = 10;

            self.thread_pool.execute(move || {
                if let Some(worker) = workers.lock().unwrap().get_mut(i) {
                    worker.do_work();
                }
            });
        }

        return Ok(());
    }

    pub fn dispatch(&mut self) {}

    pub fn event_loop(&'static mut self) -> Result<()> {
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

                        /*if let Some(worker) = self.workers.lock().unwrap().get_mut(token.0) {
                            worker.set_worker_id(token.0 as i32);
                        }*/

                        self.connections.insert(token, connection);
                    },
                    token => {
                        //派发任务
                        if let Some(connection) = self.connections.get_mut(&token) {
                            connection.read_to_end(&mut buffer);
                            let nodes = self.files.get_file_info(self.task_finish);
                            // https://www.reddit.com/r/rust/comments/bv90s7/temporary_value_dropped_while_borrowed/
                            //let mut worker = self.workers.lock().unwrap().get_mut(token.0).unwrap(); error

                            let mut worker = self.workers.lock().unwrap();
                            let worker = worker.get_mut(token.0).unwrap();
                            for v in nodes {
                                // https://stackoverflow.com/questions/51429501/how-do-i-conditionally-check-if-an-enum-is-one-variant-or-another
                                /*      if let FileType::CPP = v.t {
                                } else if let FileType::SHELL = v.t {
                                }*/

                                let task_new = || -> Box<dyn TaskBase + Send + 'static> {
                                    if let FileType::CPP = v.t {
                                        Box::new(TaskCPP::new(v))
                                    } else if let FileType::SHELL = v.t {
                                        Box::new(TaskShell::new(v))
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

                                worker.task_queue.0.send(task_new()).unwrap();
                            }

                            self.task_finish += DISPATCH_SIZE;
                        }
                    }
                }
            }
        }
    }
}
