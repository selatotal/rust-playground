extern crate pnet;

use pnet::datalink;
use ipnetwork::IpNetwork;

fn main() {
    for iface in datalink::interfaces() {
        if !iface.ips.is_empty() {
            println!("{:?}", iface.ips.get(0));
            let ip = match iface.ips.get(0).unwrap() {
                IpNetwork::V4(ipv4) => ipv4.ip().to_string(),
                IpNetwork::V6(ipv6) => ipv6.ip().to_string(),
            };
            println!("{:?}", ip);
        }
    }
}