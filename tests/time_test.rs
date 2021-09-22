use chrono::{DateTime, Local, Utc};

#[test]
fn time_1() {
    let utc = Local::now().to_string(); 
    println!("{:?}", utc);
    let mut str_temp: Vec<&str> = utc.split(".").collect();
        match str_temp.get(0) {
            Some(str) => {
                let temp = str.to_string();
                let res = temp.replace(" ", "-");
                println!("{}", &res);
            },
            None => {},
        }
}