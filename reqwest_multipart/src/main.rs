use reqwest::blocking::multipart;
use uuid::Uuid;
use chrono::{DateTime, Utc};

fn main() {

    let uuid = Uuid::new_v4();
    let formatted_timestamp : DateTime<Utc> = Utc::now();

    let payload_image_service = format!(" {{ 
        \"creation\": \"{}\", \
        \"principalOwner\": {{ \
            \"id\": \"{}\", \
            \"name\": \"event\" \
        }} \
    }}", formatted_timestamp.format("%Y%m%d%H%M%S"), uuid.clone());

    let form = multipart::Form::new()
        .text("imageInput", payload_image_service)
        .file("imageFile", "image.jpg");

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://server/rest/image")
        .multipart(form.unwrap())
        .send().unwrap();

    println!("Result: {:?}", res.text().unwrap());
}
