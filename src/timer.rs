use chrono::{DateTime, Local};

pub fn get_local_time() ->Option<String> {
    let utc: String = Local::now().to_string();
    if utc.contains(".") {
        let str_temp: Vec<&str> = utc.split(".").collect();
        match str_temp.get(0) {
            Some(str) => {
                let temp = str.to_string();
                let res = temp.replace(" ", "-");
                Some(res)
            },
            None => None,
        }
    }else {
        None
    }
}