use simlog::Log;

#[test]
fn file_2() {
    let a = chrono::offset::Local::now().timestamp_subsec_nanos();
    let log = Log::new("", "error", true);
    let temp_str = "todo!";

    log.debug(temp_str);
    log.info(temp_str);
    log.warn(temp_str);
    log.error(temp_str);
    log.fatal(temp_str);
    let b = chrono::offset::Local::now().timestamp_subsec_nanos();
    println!("{}", b - a);
}
