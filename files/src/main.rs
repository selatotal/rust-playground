use std::fs;

fn main() {

    match fs::read_dir("/home/talesviegas/Temasp/") {
        Ok(files) => files.into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.file_name().to_string_lossy().contains("db.temp"))
            .for_each(|path| {
                    println!("Removing {:?}", path);
                    fs::remove_file(path.path()).unwrap();
        }),
        Err(e) => println!("Error reading files on path: {:?}", e),
    }
}
