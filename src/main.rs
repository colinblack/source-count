/*use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);


}*/
#![allow(unused)]
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn print(entry : &DirEntry){
    println!("{:?}", entry.path());
}

fn main() -> io::Result<()> {

    //TODO:使用函数式风格实现visit_dirs
/*    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for val in &mut entries{
        if let Some(res) = val.to_str(){
            println!("{}", res);
        }
    } */

    visit_dirs(Path::new("./"), &print);


    Ok(())
}
