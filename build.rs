extern crate gcc;

const SRC: &str = "apt-pkg-c/lib.cpp";

fn main() {
    println!("cargo:rerun-if-changed={}", SRC);

    gcc::Build::new()
        .file(SRC)
        .cpp(true)
        .flag("-std=gnu++11")
        .compile("libapt-pkg-c.a");
}
