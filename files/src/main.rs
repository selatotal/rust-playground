use std::fs;

use glob::glob;

fn main() {
    for entry in glob("/home/talesviegas/Temp/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("Removing {:?}", path.display());
                fs::remove_file(path).unwrap();
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
