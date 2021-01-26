use crate::file::File;
use std::os::unix::io::RawFd;
use threadpool::ThreadPool;

pub struct  Scheduler{
    task_count : u64,
    files : File,
    e_fd : RawFd,
    thread_pool : ThreadPool
}

impl Scheduler{
    pub fn new() ->Scheduler{
        Scheduler{
            

        }
    }

}
