use std::fmt::*;
use std::fs::{self, File};
use std::path::Path;

use crate::timer::*;
use colored::{ColoredString, Colorize};

// 创建文件
pub fn create_file(path: &str) -> Option<File> {
    if let Err(_) = fs::create_dir_all(path) {
        let temp = "FILE PATH".red();
        display(temp, "create file dir failed!");
        return None;
    };
    if let Some(str_temp) = get_local_time() {
        let str_temp = get_file_path(path, str_temp, crate::FILE_SUF.to_string());
        let path_temp = Path::new(&str_temp);
        let a = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(path_temp);
        if let Ok(temp) = a {
            return Some(temp);
        } else {
            let temp = "FILE".red();
            display(temp, "create log file failed!");
            return None;
        }
    } else {
        let temp = "TIME".red();
        display(temp, "get local time failed!");
        return None;
    };
}

fn display<T>(color_str: ColoredString, content: T)
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
    eprintln!("{}", out_temp);
}

// 文件路径拼接
fn get_file_path(path: &str, file_name: String, file_suf: String) -> String {
    let file_name: Vec<&str> = file_name.split(" ").collect();
    format(format_args!(
        "{}/{}.{}",
        path,
        file_name.get(0).unwrap().to_string(),
        file_suf
    ))
}
