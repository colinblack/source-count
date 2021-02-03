use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
extern crate getopts;
use crate::scheduler::DISPATCH_SIZE;
use getopts::Options;

#[derive(Debug)]
pub enum FileType {
    CPP,
    SHELL,
    None,
}
#[derive(Debug)]
pub struct Node {
    pub path: String,
    pub name: String,
    pub t: FileType,
}

pub struct File {
    pub nodes: Vec<Node>,
}

impl Node {
    fn new(p: String, n: String, t: FileType) -> Node {
        Node {
            path: p,
            name: n,
            t: t,
        }
    }
}

impl File {
    pub fn new() -> File {
        File { nodes: Vec::new() }
    }

    //   fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> { //入参是函数指针
    fn visit_dirs<F: FnMut(&DirEntry, &mut Vec<Node>)>(
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
                    cb(&entry, &mut self.nodes);
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
        /*     for v in &self.files {
            println!("{}", v);
        }*/
    }

    fn node_info(v: &String) -> (String, FileType) {
        let p = Path::new(&v);

        let f = p.file_name().unwrap().to_str().unwrap().to_string();
        let t = p.extension().unwrap().to_str().unwrap().to_string();

        let t_c = || {
            let r = match t.as_str() {
                "cpp" => FileType::CPP,
                "cc" => FileType::CPP,
                "c" => FileType::CPP,
                "h" => FileType::CPP,
                "hpp" => FileType::CPP,
                "sh" => FileType::SHELL,
                _ => FileType::None,
            };
            r
        };

        (f, t_c())
    }

    /*    fn insert_node(&mut self, path : Vec<String>){
        for v in path {
            //    let t = p.file_name().unwrap();  //这样能直接取出$OsStr
            //下面这样写主要目的是v被借用，再push(v)会有“cannot move out of borrowing“ error
            //参考 https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html
            let (f, t) = {
                let p = Path::new(&v);

                 let f =  p.file_name().unwrap().to_str().unwrap().to_string();
                 let t =   p.extension().unwrap().to_str().unwrap().to_string();

                let t_c = || {
                    let r = match t.as_str() {
                        "cpp" => FileType::CPP,
                        "cc" => FileType::CPP,
                        "c" =>FileType::CPP,
                        "h" => FileType::CPP,
                        "hpp" => FileType::CPP,
                        "sh" => FileType::SHELL,
                        _ => FileType::None
                    };
                    r
                };

                (f,t)
            };
            /*  let p = Path::new(&v);
                if let Some(file) = p.file_name(){
                if let Some(file_type) = p.extension() {
                  self.nodes.push( Node::new(v, file.to_str().unwrap().to_string(), FileType::CPP));  //p借用了v，p生命周期结束了才能move v, 所以这种写法error
                }
            }*/

            self.nodes.push(Node::new(v, f, t_c()));
        }

    }*/

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
            let path = matches.opt_strs("f");
            for v in path {
                let (f, t) = File::node_info(&v);
                self.nodes.push(Node::new(v, f, t));
            }

            return Ok(());
        }

        if matches.opt_present("p") {
            let path = match matches.opt_str("p") {
                None => "".to_string(),
                Some(p) => p,
            };

            self.visit_dirs(Path::new(&path), &mut |entry: &DirEntry, v| {
                let mut p = entry.path().into_os_string().into_string().unwrap();
                let (f, t) = File::node_info(&p);
                v.push(Node::new(p, f, t)); //这里如果调用self.v.push报错两次借用
            });

            return Ok(());
        }

        Err(())
    }

    pub fn get_file_info(&'static self, index: usize) -> (&[Node]) {
        //返回固定大小数组
        let nodes = self.nodes.get(index..(index + DISPATCH_SIZE)).unwrap();
        nodes
    }
}
