use std::fs;

fn main() {
    let paths = fs::read_dir("/tmp/").unwrap();
    let mut str_paths = vec!();

    for path in paths {
        str_paths.push(format!("{}", path.unwrap().path().display()));
    }
    println!("{}", format!("{:?}", str_paths));
}
