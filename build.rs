extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("apt-pkg-c/lib.cpp")
        .cpp(true)
        .flag("-std=gnu++11")
        .compile("libapt-pkg-c.a");
}
