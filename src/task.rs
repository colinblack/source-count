use crate::file::Node;

pub type Task=Node;

/*pub struct Task {
    task_id: u64,
    path: & 'static str,  //这里引用的生命周期必须要是static, 才能传入线程池 参考 https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541/2
    file: & 'static str,
}*/

pub trait  TaskBase{
    fn print_info(&self);
}

pub struct TaskCPP {
    task : & 'static Task
}

impl TaskCPP{
   pub fn new(t : &'static Task) -> TaskCPP{
        TaskCPP {
            task : t
        }
    }
}
impl TaskBase for TaskCPP{
    fn print_info(&self){
        println!("{:?}", self.task);
    }
}

pub struct TaskShell {
    task : & 'static Task

}

impl TaskShell{
    pub fn new(t : &'static Task) -> TaskShell{
        TaskShell{
            task : t
        }
    }
}

impl TaskBase for TaskShell{
    fn print_info(&self){
        println!("{:?}", self.task);
    }
}


pub struct TaskNone{

}
impl TaskNone{
    pub fn new() -> TaskNone{
        TaskNone{
        }
    }
}

impl TaskBase for TaskNone{
    fn print_info(&self){
    }
}



