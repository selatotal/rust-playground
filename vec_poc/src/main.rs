fn main() {
    let test : Vec<&str> = vec!("teste");
    let empty = test.is_empty();
    println!("Result: {} ", empty);
}
