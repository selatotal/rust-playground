use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Hello, {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2 : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Hello, {:?}", scores2);
    let key = String::from("Blue");
    match scores2.get(&key) {
        Some(n) => println!("Blue Value: {:?}", n),
        None => println!("No value for Blue"),
    }
    for (key, value) in &scores {
        println!("Key {} => Value {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("Hello, {:?}", map);

    // Override value
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);
    println!("Scores 3 = {:?}", scores3);

    // Insert if key has no value
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);
    scores4.entry(String::from("Yellow")).or_insert(50);
    scores4.entry(String::from("Blue")).or_insert(50);
    println!("Scores 4 = {:?}", scores4);

    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Counter: {:?}", map);

}
