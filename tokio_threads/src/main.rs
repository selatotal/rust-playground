use tokio::task;
use tokio::time::delay_for;
use std::time::Duration;

#[tokio::main]
async fn main() {
    task::spawn(async {
        let mut count = 0;
        loop {
            println!("First Thread running {}", count);
            count = count + 1;
            // ERROR: https://github.com/tokio-rs/tokio/issues/1897
            delay_for(Duration::from_secs(1)).await;
        }
    });

    task::spawn(async {
        let mut count = 0;
        loop {
            println!("Second Thread running {}", count);
            count = count + 1;
            // ERROR: https://github.com/tokio-rs/tokio/issues/1897
            delay_for(Duration::from_secs(1)).await;
        }
    });

    println!("Program finished!");
}
