pub(crate) mod file;
pub(crate) mod timer;

use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use colored::{ColoredString, Colorize};
use file::create_file;

extern crate chrono;
extern crate colored;
extern crate lazy_static;
extern crate time;

const FILE_SUF: &str = ".log";

lazy_static::lazy_static! {
    pub static ref LOG_FILE: Arc<Mutex<File>> = Arc::new(Mutex::new(File::create("flog.lock").unwrap()));
}

#[derive(Debug)]
pub struct Log {
    file_path: String,
    out_display: bool,
    log_file: bool,
}

impl Log {
    pub fn new(file_path: String, out_display: bool) -> Log {
        match create_file(file_path.clone()) {
            Some(temp) => {
                
                let arc_temp = Arc::clone(&LOG_FILE);
                let mut mut_temp = arc_temp.lock().unwrap();
                *mut_temp = temp;

                Log {
                file_path: file_path.clone(),
                out_display,
                log_file: true,
            }},
            None => Log {
                file_path: file_path.clone(),
                out_display,
                log_file: false,
            },
        }
    }
}

trait Record {
    fn info(&self, content: String);
    fn debug(&self, content: String);
    fn warning(&self, content: String);
    fn error(&self, content: String);
    fn fatal(&self, content: String);
}

impl Record for Log {
    fn info(&self, content: String) {
        let content = content.green();
        optp(self.log_file, self.out_display, content);
    }

    fn debug(&self, content: String) {
        let content = content.white();
        optp(self.log_file, self.out_display, content);
    }

    fn warning(&self, content: String) {
        let content = content.yellow();
        optp(self.log_file, self.out_display, content);
    }

    fn error(&self, content: String) {
        let content = content.red();
        optp(self.log_file, self.out_display, content);
    }

    fn fatal(&self, content: String) {
        let content = content.black();
        optp(self.log_file, self.out_display, content);
    }
}

fn optp(log_file: bool, out_display: bool, content: ColoredString) {
    // 日志文件是否存在
    if log_file {
        if out_display {
            
        }else {
            
        }
    }else {
        if out_display {
            
        }else {
            
        }
    }
}