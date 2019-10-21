#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        heigth: 50
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Rect is {:#?}", rect1);
}
