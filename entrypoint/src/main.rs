use module1::render;
use module2::log;
use std::{thread::sleep, time::Duration};


fn main() {
    render();
    log();
    sleep(Duration::from_millis(12000));
}