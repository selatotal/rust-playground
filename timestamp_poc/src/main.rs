use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread;

fn main() {
    let first = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    thread::sleep(Duration::from_secs(2));
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("Difference: {}", (now-first));
}
