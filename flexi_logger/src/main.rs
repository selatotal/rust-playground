use flexi_logger::{Logger, opt_format, Criterion, Naming, Cleanup};
use log::*;

fn main() {

    let mut log_handle = Logger::with_str("debug")
        .log_to_file()
        .append()
        .directory("log")
        .format(opt_format)
        .rotate(Criterion::Size(10000), Naming::Numbers, Cleanup::KeepLogAndZipFiles(2, 2))
        .start()
        .unwrap();

    for i in 0..10000 {
        debug!("Hello, world! {}", i);
        if i == 3000{
            log_handle.parse_new_spec("info");
        }
    }
}
