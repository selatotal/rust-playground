fn main() {

    let base = 0;
    let inc = 12;
    let iteractions = 20;
    let mut position = base;
    for i in 0..iteractions {
        println!("{:06X}", position);
        if i < iteractions/2 {
            position = position + inc;
        } else {
            position = position - inc;
        }
    }
    println!("{:06X}", base);
}
