extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // command to cargo:
    println!("cargo:rustc-link-lib=apt-pkg");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .whitelisted_type("URI")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
