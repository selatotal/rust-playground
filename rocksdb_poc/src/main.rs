use rocksdb::DB;
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Device {
    name: String,
    address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Test {
    name: String,
    address: String,
    number: i32,
    device: Vec<String>,
}

fn main() {

    let db = DB::open_default("/tmp/rocksdb").unwrap();

    let device1 = Device {
        name: String::from("device1"),
        address: String::from("a1:b3:c3:d4:e5:f6"),
    };

    let test = Test {
        name: String::from("teste"),
        address: String::from("address"),
        number: 100,
        device: vec!(device1.address.clone()),
    };

    db.put("test", bincode::serialize(&test).unwrap()).unwrap();

    let test = match db.get("test") {
        Ok(Some(value)) => value,
        Ok(None) => panic!("value not found"),
        Err(e) =>  panic!("Error {} getting value", e),
    };

    let test : Test = bincode::deserialize(&test).unwrap();
    println!("Test: {:?}", test);

}
