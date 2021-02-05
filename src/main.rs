/*use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);


}*/
#![allow(unused)]
mod file;
mod scheduler;
mod task;
mod worker;

use file::File;
use scheduler::Scheduler;
use std::io;

#[macro_use]

// one possible implementation of walking a directory only visiting files

/*fn print(entry: &DirEntry) {
    //    println!("{:?}", entry.path());
}*/

fn main() -> io::Result<()> {
    //TODO:使用函数式风格实现visit_dirs
    /*
        let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    for val in &mut entries{
        if let Some(res) = val.to_str(){
            println!("{}", res);
        }
    }
    */
    let mut scheduler: Scheduler = Scheduler::new(4);
    scheduler.initial();
    scheduler.run();
    scheduler.event_loop();

    Ok(())
}

#[cfg(test)]
mod tests {
    use mio::{Events, Poll};
    use std::io;
    use std::time::Duration;

    #[test]
    fn do_work() -> io::Result<()> {
        return Ok(());
    }
}
