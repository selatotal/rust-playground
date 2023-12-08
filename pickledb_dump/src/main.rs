use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde_json::json;
use structs::{TakePictureRequest, Message, PictureData};
use crate::shadow::Shadow;

mod structs;
mod shadow;
mod storage;

fn main() {

    let db = PickleDb::load("/tmp/storage.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin).unwrap();

    for kv in db.iter() {
        let key = kv.get_key();
        match key {
            "shadow_bin" => {
                // let shadow_string = kv.get_value::<Shadow>().unwrap();
                // let shadow_opts = serde_json::to_string(&shadow_string);
                // println!("shadow: {}", shadow_opts.unwrap());
            }, 
            "shadow" => {
                let shadow_string = kv.get_value::<String>().unwrap();
                let shadow_opts:Shadow = serde_json::from_str(&shadow_string).unwrap();
                println!("shadow: {}", json!(shadow_opts));
            }, 
            "last_button_clicked" => println!("{}: {}", key, kv.get_value::<u64>().unwrap()),
            "last_event_timestamp" => println!("{}: {}", key, kv.get_value::<u64>().unwrap()),
            "button_count" => println!("{}: {}", key, kv.get_value::<u16>().unwrap()),
            "active_call" => println!("{}: {}", key, kv.get_value::<bool>().unwrap()),
            "sequence_code" => println!("{}: {}", key, kv.get_value::<usize>().unwrap()),
            "safe_call_number" => println!("{}: {}", key, kv.get_value::<String>().unwrap()),
            _ => {
                if key.starts_with("pic_request"){
                    println!("{}: {:?} ", key, kv.get_value::<TakePictureRequest>().unwrap())
                } else {
                    println!("{} - valor nao tratado", key)
                }
            },

        }
    }

    println!("===== Fila de eventos =====");
    let mut int = 1;
    for item in db.liter("mqtt_messages"){
        if let Some(message) = item.get_item::<Message>(){
            println!("{} - Current item is: {:?}", int, message);
        } else {
            println!("Empty item> {}", int);
        }
        int = int + 1;
    }

    println!("===== Fila de imagens =====");
    let mut int = 1;
    for item in db.liter("pictures"){
        if let Some(message) = item.get_item::<PictureData>(){
            println!("{} - Current item is: {:?}", int, message);
        } else {
            println!("Empty item> {}", int);
        }
        int = int + 1;
    }

}
