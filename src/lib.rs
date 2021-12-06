pub(crate) mod file;
pub(crate) mod global;
pub(crate) mod timer;

use colored::{ColoredString, Colorize};
use file::create_file;
use std::{
    fmt::{Display},
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    None,
}

/// # Simlog
///
#[derive(Debug)]
pub struct Log {
    file_path: &'static str,
    pub out_display: bool,
    log_file: Option<Arc<Mutex<File>>>,
    level: Level,
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
    pub fn new(file_path: &'static str, level: Level, out_display: bool) -> Log {
        // let level = set_level(level);
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

    pub fn debug<T>(&self, format: T)
    where
        T: Display,
    {
        let temp = "DEBUG".blue();

        if self.level <= Level::Debug {
            if let Some(file_temp) = &self.log_file {
                let arc_temp = Arc::clone(file_temp);
                out_put_log(self.out_display, arc_temp, temp, format);
            } else {
                out_put_log_no_log_file(temp, format);
            }
        }
    }

    pub fn info<T>(&self, format: T)
    where
        T: Display,
    {
        let temp = "INFO".green();

        if self.level <= Level::Info {
            if let Some(file_temp) = &self.log_file {
                let arc_temp = Arc::clone(file_temp);
                out_put_log(self.out_display, arc_temp, temp, format);
            } else {
                out_put_log_no_log_file(temp, format);
            }
        }
    }

    pub fn warn<T>(&self, format: T)
    where
        T: Display,
    {
        let temp = "WARNING".yellow();

        if self.level <= Level::Warn {
            if let Some(file_temp) = &self.log_file {
                let arc_temp = Arc::clone(file_temp);
                out_put_log(self.out_display, arc_temp, temp, format);
            } else {
                out_put_log_no_log_file(temp, format);
            }
        }
    }

    pub fn error<T>(&self, format: T)
    where
        T: Display,
    {
        let temp = "ERROR".red();

        if self.level <= Level::Error {
            if let Some(file_temp) = &self.log_file {
                let arc_temp = Arc::clone(file_temp);
                out_put_log(self.out_display, arc_temp, temp, format);
            } else {
                out_put_log_no_log_file(temp, format);
            }
        }
    }

    pub fn fatal<T>(&self, format: T)
    where
        T: Display,
    {
        let temp = "FATAL".black();
        if self.level <= Level::Fatal {
            if let Some(file_temp) = &self.log_file {
                let arc_temp = Arc::clone(file_temp);
                out_put_log(self.out_display, arc_temp, temp, format);
            } else {
                out_put_log_no_log_file(temp, format);
            }
        }
    }

}

// FIXME: 有需要解决的unwrap
// 打印和输出
fn out_put_log<T>(
    out_display: bool,
    log_file: Arc<Mutex<File>>,
    color_str: ColoredString,
    content: T,
) where
    T: Display,
{
    let datatime = if let Some(temp) = get_local_time() {
        temp.red()
    } else {
        String::from("****-**-** **-**-**").red()
    };

    let out_temp = format!(
        "[{}]-[{}]: {}\n",
        datatime, color_str, content
    );
    let out_temp_file = format!(
        "[{}]-[{}]: {}\n",
        datatime.clear(),
        color_str.clear(),
        content
    );

    let arc_temp = Arc::clone(&log_file);
    let mut file_temp = arc_temp.lock().unwrap();

    if out_display {
        print!("{}", &out_temp);
        file_temp.write_all(out_temp_file.as_bytes()).unwrap();
    } else {
        file_temp.write_all(out_temp_file.as_bytes()).unwrap();
    }
}

fn out_put_log_no_log_file<T>(color_str: ColoredString, content: T)
where
    T: Display,
{
    let datatime = if let Some(temp) = get_local_time() {
        temp.red()
    } else {
        String::from("****-**-** **-**-**").red()
    };

    let out_temp = format!(
        "[{}]-[{}]: {}\n",
        datatime, color_str, content
    );

    eprint!("{}", &out_temp);
}