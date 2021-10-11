pub(crate) mod file;
pub(crate) mod timer;
pub(crate) mod operate;

use std::{fmt::{format, Display}, fs::File, io::Write, sync::{Arc, Mutex}, thread, time};
use colored::{ColoredString, Colorize};
use file::create_file;
use timer::get_local_time;

extern crate chrono;
extern crate colored;
extern crate lazy_static;
extern crate rusty_pool;

const FILE_SUF: &str = "log";

/// # Simlog
///
#[derive(Debug)]
pub struct Log {
    file_path: &'static str,
    pub out_display: bool,
    log_file: Option<Arc<Mutex<File>>>,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File path: {}, display: {}, file: {:?}",
            self.file_path, self.out_display, self.log_file
        )
    }
}

impl Log {
    pub fn new(file_path: &'static str, out_display: bool) -> Log {
        match create_file(file_path) {
            Some(temp) => {
                let temp = Arc::new(Mutex::new(temp));
                let temp_1 = Arc::clone(&temp);

                // 自动分割文件
                operate::TDPOOL.execute(move || {
                    loop {
                        thread::sleep(time::Duration::from_millis(50));
                        match get_local_time() {
                            Some(s) => {
                                if s.contains("00:00:00") {
                                    let mut file = temp_1.lock().unwrap();
                                    match create_file(&*file_path.to_string()) {
                                        Some(f) => *file = f,
                                        None => {}
                                    }
                                    thread::sleep(time::Duration::from_millis(950));
                                }
                            }
                            None => {}
                        }
                    }
                });

                Log {
                    file_path,
                    out_display,
                    log_file: Some(temp),
                }
            }
            None => Log {
                file_path,
                out_display,
                log_file: None,
            },
        }
    }
    pub fn debug<T>(&self, content: T)
        where
            T: Display,
    {
        let temp = "DEBUG".blue();
        match &self.log_file {
            None => {
                opt_no_file(temp, content);
            }
            Some(file_temp) => {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        }
    }

    pub fn info<T>(&self, content: T)
        where
            T: Display,
    {
        let temp = "INFO".green();
        match &self.log_file {
            None => {
                opt_no_file(temp, content);
            }
            Some(file_temp) => {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        }
    }

    pub fn warn<T>(&self, content: T)
        where
            T: Display,
    {
        let temp = "WARNING".yellow();
        match &self.log_file {
            None => {
                opt_no_file(temp, content);
            }
            Some(file_temp) => {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        }
    }

    pub fn error<T>(&self, content: T)
        where
            T: Display,
    {
        let temp = "ERROR".red();
        match &self.log_file {
            None => {
                opt_no_file(temp, content);
            }
            Some(file_temp) => {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        }
    }

    pub fn fatal<T>(&self, content: T)
        where
            T: Display,
    {
        let temp = "FATAL".black();
        match &self.log_file {
            None => {
                opt_no_file(temp, content);
            }
            Some(file_temp) => {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        }
    }
}

// FIXME: 有需要解决的unwrap
// 打印和输出
fn opt<T>(out_display: bool, log_file: Arc<Mutex<File>>, color_str: ColoredString, content: T)
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

    let arc_temp = Arc::clone(&log_file);
    let mut file_temp = arc_temp.lock().unwrap();

    if out_display {
        print!("{}", &out_temp);
        file_temp.write_all(out_temp_file.as_bytes()).unwrap();
    } else {
        file_temp.write_all(out_temp_file.as_bytes()).unwrap();
    }
}

fn opt_no_file<T>(color_str: ColoredString, content: T)
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

    let failed = "FILE FAILED!".red();
    eprint!("[{}]-{}", &failed, &out_temp);
}
