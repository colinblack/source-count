use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
extern crate getopts;
use getopts::Options;

pub struct File {
    files: Vec<String>,
}

impl File {
    pub fn new() -> File {
        File { files: Vec::new() }
    }

    //   fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> { //入参是函数指针
    fn visit_dirs<F: FnMut(&DirEntry, &mut Vec<String>)>(
        &mut self,
        dir: &Path,
        cb: &mut F,
    ) -> io::Result<()> {
        //F必须为&mut
        //入参是闭包
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(&path, cb)?; //方法前要用self
                } else {
                    cb(&entry, &mut self.files);
                }
            }
        }
        Ok(())
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("sage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }
    pub fn print_counter_files(&self) {
        for v in &self.files {
            println!("{}", v);
        }
    }

    pub fn get_counter_files(&mut self) -> Result<(), ()> {
        //mut self 结构体字段才能变为可变
        let args: Vec<String> = env::args().collect();
        let mut opts = Options::new();
        opts.optopt("p", "", "set count path name", "NAME");
        opts.optopt("f", "", "set count file name", "NAME");
        opts.optflag("h", "help", "print this help menu");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                panic!(f.to_string())
            }
        };

        if matches.opt_present("h") {
            File::print_usage(&args[0], opts); //关联函数要使用::
            return Err(());
        }

        if matches.opt_present("f") {
            self.files = matches.opt_strs("f");
            return Ok(());
        }

        if matches.opt_present("p") {
            let path = match matches.opt_str("p") {
                None => "".to_string(),
                Some(p) => p,
            };

            self.visit_dirs(Path::new(&path), &mut |entry: &DirEntry, v| {
                let mut p = entry.path().into_os_string().into_string().unwrap();
                v.push(p); //这里如果调用self.fils.push报错两次借用
            });

            return Ok(());
        }

        Err(())
    }
}
