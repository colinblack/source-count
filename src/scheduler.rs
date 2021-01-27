use crate::file::File;
use mio::{Events, Poll, Interest, Token};
use threadpool::ThreadPool;

pub struct  Scheduler{
    task_count : u64,
    files : File,
    thread_pool : ThreadPool,
    poll : Poll
}

impl Scheduler{
    pub fn new(t_n : usize) ->Scheduler{
        Scheduler{
          task_count : 0,
          files : File::new(),
          thread_pool : ThreadPool::new(t_n),
          poll : 
        }
    }

    pub fn initail(&mut self) -> Result<(),()>{
  /*      match self.files.get_counter_files(){
            Ok(_) => {}
            Err(_) => {panic!("get counter files fail")}
        }
*/
        let mut poll = Poll::new();


        Ok(())
    }
}
