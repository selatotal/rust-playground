enum IpAddrKind {
    V4(String),
    V6(String),
}

let home = IpAddrKind::V4(String.from("127.0.0.1"));
let loopback = IpAddrKind::V6(String.from("::1"));
