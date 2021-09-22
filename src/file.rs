use std::fmt::*;
use std::fs::{self, File};

use crate::timer::*;
use colored::Colorize;

pub fn create_file(path: String) -> Option<File> {
    if let Some(str_temp) = get_local_time() {
        let path_temp = get_file_path(path, str_temp, crate::FILE_SUF.to_string());
        match fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path_temp)
        {
            Ok(temp) => temp,
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
    None
}

fn get_file_path(path: String, file_name: String, file_suf: String) -> String {
    format(format_args!("{}/{}.{}", path, file_name, file_suf))
}
