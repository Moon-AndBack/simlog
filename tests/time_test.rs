use chrono::{DateTime, Local, Utc};
use delay_timer::prelude::*;
use smol::Timer;
use std::time::Duration;

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
        }
        None => {}
    }
}

#[test]
fn aa() {
    let a = DelayTimerBuilder::default().build();
    let bb = a.insert_task(tete().unwrap());
}

fn tete() ->Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();

    let body = create_async_fn_body!({
        println!("create_async_fn_body!");

        Timer::after(Duration::from_secs(3)).await;

        println!("create_async_fn_body:i'success");
    });

    task_builder
        .set_task_id(1)
        .set_frequency_repeated_by_seconds(6)
        .set_maximum_parallel_runnable_num(2)
        .spawn(body)
}