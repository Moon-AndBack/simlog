use std::{
    fs::{self, read_dir, File, OpenOptions},
    io::Write,
    path::{self, Path},
};

#[test]
fn file_1() {
    let mut x = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("a.log")
        .unwrap();

    x.write("aaaaaa".as_bytes());
    x.write_all(buf)
}
