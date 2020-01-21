extern crate futures;
extern crate paho_mqtt as mqtt;

use std::{time, env, process};
use futures::{Future, Stream};
use futures::future::{Either, ok};

// The topics to which we subscribe.
const TOPICS: &[&str] = &[ "central/test/centralcoretest/incoming" ];
const QOS: &[i32] = &[1];

fn main() {

    // We use the trust store from the Paho C tls-testing/keys directory,
    // but we assume there's a copy in the current directory.
    const TRUST_STORE: &str = "certs/ca-cert.pem";
    const CLIENT_STORE: &str = "certs/cert.pem";
    const KEY_STORE: &str = "certs/key.pem";

    // We assume that we are in a valid directory.
    let mut trust_store = env::current_dir().unwrap();
    trust_store.push(TRUST_STORE);

    if !trust_store.exists() {
        println!("The trust store file does not exist: {:?}", trust_store);
        println!("  Get a copy from \"paho.mqtt.c/test/ssl/test-root-ca.crt\"");
        process::exit(1);
    }

    let mut key_store = env::current_dir().unwrap();
    key_store.push(KEY_STORE);

    if !key_store.exists() {
        println!("The key store file does not exist: {:?}", key_store);
        println!("  Get a copy from \"paho.mqtt.c/test/ssl/client.pem\"");
        process::exit(1);
    }

    let mut client_store = env::current_dir().unwrap();
    client_store.push(CLIENT_STORE);

    if !client_store.exists() {
        println!("The client store file does not exist: {:?}", key_store);
        println!("  Get a copy from \"paho.mqtt.c/test/ssl/client.pem\"");
        process::exit(1);
    }

    // Let the user override the host, but note the "ssl://" protocol.
    let host = env::args().skip(1).next().unwrap_or(
        "ssl://a3jend8qj3q5ix-ats.iot.us-east-1.amazonaws.com:8883".to_string()
    );

    println!("Connecting to host: '{}'", host);

    // Create a client & define connect options
    let create_opts = mqtt::CreateOptionsBuilder::new()
                    .server_uri(&host)
                    .client_id("centralcoretest")
                    .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store.to_str().unwrap())
        .key_store(client_store.to_str().unwrap())
        .private_key(key_store.to_str().unwrap())
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .mqtt_version(mqtt::MQTT_VERSION_DEFAULT)
        .finalize();

    let rx = cli.get_stream(10);
    // Make the connection to the broker
    println!("Connecting to the MQTT server...");
    cli.connect(conn_opts)
        .and_then(|rsp| {
            let mut fut = mqtt::Token::from_success();
            if let Some((server_uri, ver, session_present)) = rsp.connect_response() {
                println!("Connected to: '{}' with MQTT version {}", server_uri, ver);
                if !session_present {
                    // Subscribe to multiple topics
                    println!("Subscribing to topics: {:?}", TOPICS);
                    fut = cli.subscribe_many(TOPICS, QOS)
                }
            }
            fut
        })
        .and_then(|rsp| {
            if let Some(qosv) = rsp.subscribe_many_response() {
                if !qosv.is_empty() {
                    println!("QoS granted: {:?}", qosv);
                }
            }
            mqtt::Token::from_success()
        })
        .wait().unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(2);
        });

    println!("Waiting for messages...");
    rx.for_each(|opt_msg| {
        if let Some(msg) = opt_msg {
            println!("{}", msg);
            ok(())
        }
        else {
            println!("Stream disruption");
            err(())
        }
    }).wait().unwrap_or_else(|_| {
        println!("Done");
    });
}