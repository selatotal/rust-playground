fn main() {
    println!("Hello, world!");
    another_function(5, 7);
    println!("The value of five() is {}", five());
    println!("The value of plus_one(7) is {}", plus_one(7));
}

fn another_function(x: i32, y: i32){
    println!("THe value of x is {} and y is {}", x, y);
}

fn five() -> i32{
    5
}

fn plus_one(x:i32) -> i32 {
    x + 1
}