use std::fs;

use structs::ShadowRestoreResponse;
mod structs;

fn main() {

    let text = fs::read_to_string("shadow.json").unwrap();
    println!("text: {}", text);

    let deserialized: ShadowRestoreResponse = serde_json::from_str("{\"dateTime\":0, \"attributes\":{} }").unwrap();
    println!("deserialized: {:?}", deserialized);

    let serialized: String = serde_json::to_string(&deserialized).unwrap();
    println!("=======SERIALIZED=========");
    println!("{}", serialized);

}
