use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TakePictureRequest {
    pub command_id: String,
    pub device_code: String,
    pub expiration_datetime: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    Event(String),
    Shadow {
        payload: String,
        shadow: String,
    },
    ShadowCommand {
        payload: String,
        shadow: String,
        command: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PictureData {
    pub file: String,
    pub device_code: String,
    pub device_id: String,
    pub payload: String,
    pub uuid: String,
    pub command_id: Option<String>,
    pub source: TakePictureSource,
    pub hidden_zone: bool,
}

#[derive(Clone, Hash, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum TakePictureSource {
    #[serde(rename = "0")]
    Request,
    #[serde(rename = "1")]
    Movement,
    None,
}
