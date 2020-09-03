use curl::easy::{Easy2, Handler, WriteError};
use core::time::Duration;

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {

    let interfaces = vec!("eth0","wlan0","wwan0");
    for interface in interfaces{
        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.url("http://www.google.com").unwrap();
        easy.interface(interface).unwrap();
        easy.timeout(Duration::from_secs(10)).unwrap();
        match easy.perform() {
            Ok(_) => println!("Interface {} connected", interface),
            Err(_) => println!("Interface {} without connection", interface),
        }
    }
}
