use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

fn main() -> Result<(), Error> {

    let mut db = DB::open("../my-database.db")?;
    let mut tx = db.tx(true)?;
    let users_bucket = tx.get_bucket("users")?;
    if let Some(Data::KeyValue(kv)) = users_bucket.get(b"user1") {
        let db_user : User = rmp_serde::from_slice(kv.value()).unwrap();
        println!("{:?}", db_user);
    }
    Ok(())
}
