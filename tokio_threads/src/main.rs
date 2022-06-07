use tokio::task;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    task::spawn(async {
        let mut count = 0;
        loop {
            println!("First Thread running {}", count);
            count = count + 1;
            sleep(Duration::from_secs(1)).await;
        }
    });

    task::spawn(async {
        let mut count = 0;
        loop {
            println!("Second Thread running {}", count);
            count = count + 1;
            sleep(Duration::from_secs(1)).await;
        }
    });

    println!("Program finished!");
}
