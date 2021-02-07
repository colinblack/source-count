use crate::file::Node;
use std::fs::File;
use std::path::Path;

pub type Task = Node;
type ReadCB = fn(&mut [u8]);

pub const BUFFER_SIZE: usize = 50 * 1024;
/*pub struct Task {
    task_id: u64,
    path: & 'static str,  //这里引用的生命周期必须要是static, 才能传入线程池 参考 https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541/2
    file: & 'static str,
}*/

pub trait TaskBase {
    fn print_info(&self);
    fn do_count(&self);
}

pub struct Rio {}

impl Rio {
    pub fn read_buffer(path: &str, cb: ReadCB) {
        let ring = rio::new().expect("create uring");
        let file = File::open(Path::new(path)).expect("openat");
        let mut data: &mut [u8] = &mut [0; BUFFER_SIZE];
        let mut pos : usize = 0;
        loop{
            let completion = ring.read_at(&file, &mut data, pos as u64);
            let size =  match completion.wait(){
                Err(e) => {panic!("read file error:{}", e)},
                Ok(s) => s
            };
            cb(data);
            pos += size;
            if size < BUFFER_SIZE{
                data = &mut data[..size];
                cb(data);
                break;
            }
        }
    }
}

pub struct TaskCPP {
    task: Task,
}

impl TaskCPP {
    pub fn count(data :&mut [u8]){

    }
    pub fn new(t: Task) -> TaskCPP {
        TaskCPP { task: t }
    }
}
impl TaskBase for TaskCPP {
    fn print_info(&self) {
        println!("{:?}", self.task);
    }
    fn do_count(&self) {
        Rio::read_buffer(&self.task.path, TaskCPP::count);
    }
}

pub struct TaskShell {
    task: Task,
}

impl TaskShell {
    pub fn new(t: Task) -> TaskShell {
        TaskShell { task: t }
    }
    pub fn count(data :&mut [u8]){

    }
}

impl TaskBase for TaskShell {
    fn print_info(&self) {
        println!("{:?}", self.task);
    }

    fn do_count(&self) {
        Rio::read_buffer(&self.task.path, TaskShell::count);
    }
}

pub struct TaskNone {}
impl TaskNone {
    pub fn new() -> TaskNone {
        TaskNone {}
    }
}

impl TaskBase for TaskNone {
    fn print_info(&self) {}
    fn do_count(&self) {}
}
