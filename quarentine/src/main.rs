use chrono::prelude::*;

fn main() {

    // Day when you start querentine
    let start = Local.ymd(2020, 3, 20);
    let today = Local::today();

    // Quarentine days
    let duration = today.signed_duration_since(start);
    
    println!("Today is: {}", today.format("%Y-%m-%d"));
    println!("My quarentine started in: {}", start.format("%Y-%m-%d"));
    println!("Days in quarentine: {}", duration.num_days());
    println!("Days in quarentine (in binary): {:b}", duration.num_days());

}
