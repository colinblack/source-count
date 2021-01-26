/*use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);


}*/
#![allow(unused)]
mod file;
mod task;
mod worker;
mod scheduler;

use file::File;
use std::io;
use nix::sys::epoll::{epoll_create1, epoll_ctl, epoll_create};

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
    let mut f = File::new();
    match f.get_counter_files(){
        Ok(_) => {}
        Err(_) => {panic!("get counter files fail")}
    }
    f.print_counter_files();

    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn do_work(){

    }

}
