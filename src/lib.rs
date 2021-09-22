mod timer;
mod file;

use std::{fs::File, sync::{Arc, Mutex}};

use file::create_file;

extern crate colored;
extern crate time;
extern crate chrono;
extern crate lazy_static;

const FILE_SUF: &str = ".log";

#[derive(Debug)]
pub struct Log {
    file_path: String,
    out_display: bool,
    log_file: Arc<Mutex<File>>
}

impl Log {
    pub fn new(file_path: String, out_display: bool) ->Log {
        Log {
            file_path: file_path.clone(),
            out_display,
            log_file: Arc::new(Mutex::new(create_file(file_path))),
        }
    }
}
