extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main(){

    println!("cargo:rustc-link-lib=nbus");
    println!("cargo:rustc-link-search=dependency=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}