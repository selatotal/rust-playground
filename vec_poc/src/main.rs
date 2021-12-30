fn main() {
    let test : Vec<&str> = vec!("teste");
    let empty = test.is_empty();
    println!("Teste: {:?} ", test);
    println!("Result: {} ", empty);

    let a = [1, 2, 3, 4, 5];
    let b = [1, 2, 5, 3, 4];

    let matching = b.iter().filter(|&b| a.contains(b)).count();
    println!("{}", matching);

    println!("{:?}", a);
}
