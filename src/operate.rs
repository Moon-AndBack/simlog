use std::time::Duration;

use rusty_pool::ThreadPool;

lazy_static::lazy_static!{
    pub static ref TDPOOL: ThreadPool = ThreadPool::new(1, 1, Duration::from_secs(1));
}