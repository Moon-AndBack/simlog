pub(crate) mod file;
pub(crate) mod operate;
pub(crate) mod timer;

use std::{
    fmt::{format, Display},
    fs::File,
    io::Write,
    sync::{Arc, Mutex, RwLock},
    thread, time,
};

use colored::{ColoredString, Colorize};
use file::create_file;
use timer::get_local_time;

extern crate chrono;
extern crate colored;
extern crate lazy_static;

const FILE_SUF: &str = "log";

lazy_static::lazy_static! {
    pub static ref LOG_FILE: Arc<Mutex<File>> = Arc::new(Mutex::new(File::create("flog.lock").unwrap()));
    pub static ref IF_FILE: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    pub static ref PATH_FILE: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));

}

/// ### 这是一个开箱即用的日志组件
/// **仅需要路径参数和是否在控制台打印，并且是否在控制台打印是pub的，你可以随时修改它**
/// # 例子
///
///```
///use std::{time, thread};
///use flog::Log;
///let log = Log::new("./log".to_string(), true);
///loop {
///   thread::sleep(time::Duration::from_secs(2));
///   log.debug(&x);
///   log.info(&x);
///   log.warn(&x);
///   log.error(&x);
///   log.fatal(&x);
///}
///```
///
///![avatar](./images/image_test.png)
#[derive(Debug)]
pub struct Log {
    file_path: String,
    pub out_display: bool,
    log_file: bool,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File path: {}, display: {}, file: {}",
            self.file_path, self.out_display, self.log_file
        )
    }
}

impl Log {
    pub fn new(file_path: String, out_display: bool) -> Log {
        match create_file(file_path.clone()) {
            Some(temp) => {
                {
                    let arc_temp = Arc::clone(&LOG_FILE);
                    let mut mut_temp = arc_temp.lock().unwrap();
                    *mut_temp = temp;

                    let arc_temp1 = Arc::clone(&IF_FILE);
                    let mut mut_temp1 = arc_temp1.write().unwrap();
                    *mut_temp1 = true;

                    let arc_temp2 = Arc::clone(&PATH_FILE);
                    let mut mut_temp2 = arc_temp2.write().unwrap();
                    *mut_temp2 = file_path.clone();
                }
                operate::TDPOOL.execute(auto_seg_file);
                Log {
                    file_path: file_path.clone(),
                    out_display,
                    log_file: true,
                }
            }
            None => Log {
                file_path: file_path.clone(),
                out_display,
                log_file: false,
            },
        }
    }
    pub fn debug<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "DEBUG".blue();
        optp(self.log_file, self.out_display, temp, content);
    }

    pub fn info<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "INFO".green();
        optp(self.log_file, self.out_display, temp, content);
    }

    pub fn warn<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "WARNING".yellow();
        optp(self.log_file, self.out_display, temp, content);
    }

    pub fn error<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "ERROR".red();
        optp(self.log_file, self.out_display, temp, content);
    }

    pub fn fatal<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "FATAL".black();
        optp(self.log_file, self.out_display, temp, content);
    }
}

// FIXME: 有需要解决的unwrap
// 打印和输出
fn optp<T>(log_file: bool, out_display: bool, color_str: ColoredString, content: T)
where
    T: Display,
{
    let datatime = match get_local_time() {
        Some(temp) => temp.red(),
        None => String::from("****-**-** **-**-**").red(),
    };
    let out_temp = format(format_args!(
        "[{}]-[{}]: {}\n",
        datatime, color_str, content
    ));
    let out_temp_file = format(format_args!(
        "[{}]-[{}]: {}\n",
        datatime.clear(),
        color_str.clear(),
        content
    ));

    let arc_temp = Arc::clone(&LOG_FILE);
    let mut file_temp = arc_temp.lock().unwrap();
    // 日志文件是否存在
    if log_file {
        if out_display {
            print!("{}", &out_temp);
            file_temp.write_all(out_temp_file.as_bytes()).unwrap();
        } else {
            file_temp.write_all(out_temp_file.as_bytes()).unwrap();
        }
    } else {
        let failed = "FILE FAILED!".red();
        eprint!("[{}]-{}", &failed, &out_temp);
    }
}

// 自动分割文件
fn auto_seg_file() {
    let arc_temp = Arc::clone(&IF_FILE);
    let mut_temp = arc_temp.read().unwrap();
    if *mut_temp {
        loop {
            thread::sleep(time::Duration::from_millis(59000));
            match get_local_time() {
                Some(s) => {
                    if s.contains("00:00:") {
                        let arc_temp = Arc::clone(&LOG_FILE);
                        let mut mut_temp = arc_temp.lock().unwrap();

                        let arc_temp1 = Arc::clone(&PATH_FILE);
                        let mut_temp1 = arc_temp1.read().unwrap();

                        match create_file(mut_temp1.to_string()) {
                            Some(f) => *mut_temp = f,
                            None => {}
                        }
                        thread::sleep(time::Duration::from_millis(1000));
                    }
                }
                None => {}
            }
        }
    }
}
