use std::time::Duration;

pub fn do_some_work() {
    std::thread::sleep(Duration::from_millis(10));
}