fn main() {

    let data = "initial contents";
    let mut s = data.to_string();
    s = "initial contents".to_string();

    s = String::from("foo");
    s.push_str("bar");

    println!("Hello, {}!", s);
}
