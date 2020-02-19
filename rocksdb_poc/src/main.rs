extern crate rocksdb;

use rocksdb::DB;

struct Device {
    name: String,
    address: String,
}

struct Test {
    name: String,
    address: int,
    device: Vec<String>,
}

fn main() {

    let db = DB::open_default("/tmp/rocksdb").unwrap();

    let device1 = Device {
        name = String::from("device1"),
        address = String::from("a1:b3:c3:d4:e5:f6"),
    }

    let test = {
        name = String::from("teste"),
        address = String::from"address",
        device = vec!(device1.address.clone()),
    } 

    db.put(b)


}
