# Rust MQTT Poc
Proof-of-concept of Rust connecting to AWS MQTT broker using rumqtt lib

## Configure
- Configure certificates in AWS IoT Core (https://docs.aws.amazon.com/iot/latest/developerguide/iot-authentication.html)
- Add CA/Certificate and Private Key PEM files in certs folder and configure main.rs file to your certificates
- Configure broker with your AWS IoT Broker endpoint

## Execute
```bash
cargo run
```

## To Learn More
- [AWS IoT Core Documentation](https://docs.aws.amazon.com/iot/latest/-developerguide/what-is-aws-iot.html)
- [rumqtt create documentation](https://docs.rs/crate/rumqtt)