use std::fs::File;

use crate::timer::*;
use colored::Colorize;

pub fn create_file(path: String) ->File {
    if let Some(str_temp) = get_local_time() {
        
    }else {
        let temp = "create local time failed!".red();
        eprintln!("{}", &temp);
    };
    return File::open("aa").unwrap();
}