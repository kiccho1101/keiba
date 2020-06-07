use dotenv::dotenv;
use std::{env, thread, time};

pub fn sleep() {
    dotenv().ok();
    let sleep_millisec: u64 = env::var("SLEEP")
        .expect("SLEEP must be set")
        .parse()
        .expect("SLEEP must be integer");
    thread::sleep(time::Duration::from_millis(sleep_millisec));
}
