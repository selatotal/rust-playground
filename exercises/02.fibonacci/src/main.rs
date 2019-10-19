fn main() {

    let mut before_number:u64 = 0;
    let mut number:u64 = 1;
    let mut aux:u64;
    loop {
        println!("{}", number);
        aux = number;
        number = before_number + number;
        before_number = aux;
    }
}
