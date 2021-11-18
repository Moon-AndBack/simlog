pub(crate) mod file;
pub(crate) mod global;
pub(crate) mod timer;

use colored::{ColoredString, Colorize};
use file::create_file;
use std::{
    fmt::{format, Display},
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread, time,
};
use timer::get_local_time;

extern crate chrono;
extern crate colored;
extern crate lazy_static;
extern crate rusty_pool;

const FILE_SUF: &str = "log";

#[derive(Debug)]
enum LEVEL {
    INFO,
    DEBUG,
    WARN,
    ERROR,
    FATAL,
    NONE,
}

/// # Simlog
///
#[derive(Debug)]
pub struct Log {
    file_path: &'static str,
    pub out_display: bool,
    log_file: Option<Arc<Mutex<File>>>,
    level: LEVEL,
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
    pub fn new(file_path: &'static str, level: &'static str, out_display: bool) -> Log {
        let level = set_level(level);
        if let Some(temp) = create_file(file_path) {
            let temp = Arc::new(Mutex::new(temp));
            let temp_1 = Arc::clone(&temp);

            // 自动分割文件
            global::TDPOOL.execute(move || loop {
                thread::sleep(time::Duration::from_millis(50));
                if let Some(s) = get_local_time() {
                    if s.contains("00:00:00") {
                        let mut file = temp_1.lock().unwrap();
                        if let Some(f) = create_file(&*file_path.to_string()) {
                            *file = f
                        }
                        thread::sleep(time::Duration::from_millis(950));
                    }
                }
            });

            Log {
                file_path,
                out_display,
                log_file: Some(temp),
                level,
            }
        } else {
            Log {
                file_path,
                out_display,
                log_file: None,
                level,
            }
        }
    }

    pub fn debug<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "DEBUG".blue();
        if let Some(file_temp) = &self.log_file {
            if let LEVEL::DEBUG = self.level {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        } else {
            opt_no_file(temp, content);
        }
    }

    pub fn info<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "INFO".green();
        if let Some(file_temp) = &self.log_file {
            if let LEVEL::DEBUG | LEVEL::INFO = self.level {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        } else {
            opt_no_file(temp, content);
        }
    }

    pub fn warn<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "WARNING".yellow();
        if let Some(file_temp) = &self.log_file {
            if let LEVEL::WARN | LEVEL::INFO | LEVEL::DEBUG = self.level {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        } else {
            opt_no_file(temp, content);
        }
    }

    pub fn error<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "ERROR".red();
        if let Some(file_temp) = &self.log_file {
            if let LEVEL::ERROR | LEVEL::WARN | LEVEL::INFO | LEVEL::DEBUG = self.level {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        } else {
            opt_no_file(temp, content);
        }
    }

    pub fn fatal<T>(&self, content: T)
    where
        T: Display,
    {
        let temp = "FATAL".black();
        if let Some(file_temp) = &self.log_file {
            if let LEVEL::FATAL | LEVEL::ERROR | LEVEL::WARN | LEVEL::INFO | LEVEL::DEBUG = self.level {
                let arc_temp = Arc::clone(file_temp);
                opt(self.out_display, arc_temp, temp, content);
            }
        } else {
            opt_no_file(temp, content);
        }
    }
}

fn set_level(level: &str) -> LEVEL {
    match level {
        "info" => LEVEL::INFO,
        "debug" => LEVEL::DEBUG,
        "warn" => LEVEL::WARN,
        "error" => LEVEL::ERROR,
        "fatal" => LEVEL::FATAL,
        _ => LEVEL::NONE,
    }
}

// FIXME: 有需要解决的unwrap
// 打印和输出
fn opt<T>(out_display: bool, log_file: Arc<Mutex<File>>, color_str: ColoredString, content: T)
where
    T: Display,
{
    let datatime = if let Some(temp) = get_local_time() {
        temp.red()
    } else {
        String::from("****-**-** **-**-**").red()
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
    let datatime = if let Some(temp) = get_local_time() {
        temp.red()
    } else {
        String::from("****-**-** **-**-**").red()
    };
    let out_temp = format(format_args!(
        "[{}]-[{}]: {}\n",
        datatime, color_str, content
    ));

    let failed = "FILE FAILED!".red();
    eprint!("[{}]-{}", &failed, &out_temp);
}
