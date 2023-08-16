extern crate futures;
extern crate paho_mqtt as mqtt;
use log::*;
use std::{env, process};
use std::time::{Duration};

// The topics to which we subscribe.
const TOPICS: &[&str] = &[ "central/test/centralcoretest/incoming" ];
const QOS: &[i32] = &[1];

fn main() {
    pretty_env_logger::init();

    // We use the trust store from the Paho C tls-testing/keys directory,
    // but we assume there's a copy in the current directory.
    const TRUST_STORE: &str = "certs/ca-cert.pem";
    const CLIENT_STORE: &str = "certs/cert.pem";
    const KEY_STORE: &str = "certs/key.pem";

    // We assume that we are in a valid directory.
    let mut trust_store = env::current_dir().unwrap();
    trust_store.push(TRUST_STORE);

    if !trust_store.exists() {
        error!("The trust store file does not exist: {:?}", trust_store);
        error!("  Get a copy from \"paho.mqtt.c/test/ssl/test-root-ca.crt\"");
        process::exit(1);
    }

    let mut key_store = env::current_dir().unwrap();
    key_store.push(KEY_STORE);

    if !key_store.exists() {
        error!("The key store file does not exist: {:?}", key_store);
        error!("  Get a copy from \"paho.mqtt.c/test/ssl/client.pem\"");
        process::exit(1);
    }

    let mut client_store = env::current_dir().unwrap();
    client_store.push(CLIENT_STORE);

    if !client_store.exists() {
        error!("The client store file does not exist: {:?}", key_store);
        error!("  Get a copy from \"paho.mqtt.c/test/ssl/client.pem\"");
        process::exit(1);
    }

    // Let the user override the host, but note the "ssl://" protocol.
    let host = env::args().skip(1).next().unwrap_or(
        "ssl://a993sqym4h3gr-ats.iot.us-east-1.amazonaws.com:8883".to_string()
    );

    info!("Connecting to host: '{}'", host);

    // Create a client & define connect options
    let create_opts = mqtt::CreateOptionsBuilder::new()
                    .server_uri(&host)
                    .client_id("a453ee2016bb")
                    .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        error!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(&TRUST_STORE).unwrap()
        .key_store(&CLIENT_STORE).unwrap()
        .private_key(&KEY_STORE).unwrap()
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .connect_timeout(Duration::from_secs(15))
        .retry_interval(Duration::from_secs(10))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
        .finalize();

    // Make the connection to the broker
    info!("Connecting to the MQTT server...");
    cli.set_connected_callback(|cli| {
        let mut _fut = mqtt::Token::from_success();
        info!("Connected");
        // Subscribe to multiple topics
        info!("Subscribing to topics: {:?}", TOPICS);
        _fut = cli.subscribe_many(TOPICS, QOS)
    });

    cli.set_connection_lost_callback(|_cli| {
        error!("Connection lost...");
    });

    cli.set_message_callback(move |_cli, msg| {
        info!("Waiting for messages...");
        if let Some(msg) = msg {
            info!("{}", msg);
        }
        else {
            info!("Stream disruption");
        }

    });

    cli.connect(conn_opts);

    loop {}
 }