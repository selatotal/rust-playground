use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Command {
    id: String,
    command_type_id: String,
    central_id: String,
    expiration_datetime: i64,
    send_datetime: i64,
    ack_datetime: i64,
    execution_datetime: i64,
    status: i32,
    error_code: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let res = reqwest::blocking::get("http://www.mocky.io/v2/5dd2a9f23300002b007a3d99")?;
    let text : String = res.text().unwrap();

    let deserialized: Command = serde_json::from_str(&text).unwrap();
    println!("deserialized: {:?}", deserialized);

    let serialized: String = serde_json::to_string(&deserialized).unwrap();
    println!("serialized: {}", serialized);

    Ok(())
}
