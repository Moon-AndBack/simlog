use std::{
    fs::{self, read_dir, File, OpenOptions},
    io::Write,
    path::{self, Path},
    thread, time,
};

use simlog::Log;

#[test]
fn file_1() {
    let mut x = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("./log/12.log")
        .unwrap();

    x.write("aaaaaa".as_bytes());
    // x.write_all(buf);
}

#[test]
fn file_2() {
    let x = Log::new("./log".to_string(), true);
    // loop {
    //     thread::sleep(time::Duration::from_secs(2));
    x.debug(&x);
    x.info(&x);
    x.warn(&x);
    x.error(&x);
    x.fatal(&x);
    // }
}
