use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

fn main() -> Result<(), Error>{

    let user = User {
        username: "my-user".to_string(),
        password: "my-password".to_string(),
    };

    // Oen a new database file and start a writable transaction
    let mut db = DB::open("../my-database.db")?;
    let mut tx = db.tx(true)?;

    // Create a bucket to store users
    let users_bucket = tx.create_bucket("users")?;

    // Serialize struct to bytes and store in bucket
    let user_bytes = rmp_serde::to_vec(&user).unwrap();
    users_bucket.put(b"user1", user_bytes)?;

    tx.commit()?;
    println!("User Saved");
    Ok(())
}
