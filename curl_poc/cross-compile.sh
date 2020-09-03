cross build --target mipsel-unknown-linux-musl --release
docker run --rm -v "$(pwd)":/home/rust/src orsegups/mipsel-unknown-linux-musl mipsel-linux-muslsf-strip /home/rust/src/target/mipsel-unknown-linux-musl/release/curl_poc

