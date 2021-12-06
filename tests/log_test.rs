use simlog::{Log, Level};

#[test]
fn log() {
    let log = Log::new("log", Level::Fatal, true);
    let temp_str = "todo!";

    log.debug(format!("{}", temp_str));
    log.info(format!("{}", temp_str));
    log.warn(format!("{}", temp_str));
    log.error(format!("{}", temp_str));
    log.fatal(format!("{}", temp_str));
}
