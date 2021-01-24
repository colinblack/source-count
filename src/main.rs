/*use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);


}*/
#![allow(unused)]
pub mod file;
use file::File;
use std::io;

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

//    visit_dirs(Path::new("./"), &print);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
