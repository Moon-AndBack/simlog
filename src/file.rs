use std::fmt::*;
use std::fs::{self, File};
use std::path::Path;

use crate::timer::*;
use colored::Colorize;

// 创建文件
pub fn create_file(path: String) -> Option<File> {
    match fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(_) => {
            let temp = "create file path failed!".red();
            eprintln!("{}", &temp);
            return None;
        }
    };
    if let Some(str_temp) = get_local_time() {
        let str_temp = get_file_path(path, str_temp, crate::FILE_SUF.to_string());
        let path_temp = Path::new(&str_temp);
        let a = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path_temp);
        match a {
            Ok(temp) => {
                return Some(temp);
            }
            Err(_) => {
                let temp = "create file failed!".red();
                eprintln!("{}", &temp);
                return None;
            }
        }
    } else {
        let temp = "create local time failed!".red();
        eprintln!("{}", &temp);
        return None;
    };
}

// 文件路径拼接
fn get_file_path(path: String, file_name: String, file_suf: String) -> String {
    let file_name = file_name.replace(":", "-");
    format(format_args!("{}/{}.{}", path, file_name, file_suf))
}
