extern crate gcc;

fn main() {
    gcc::compile_library("libapt-pkg-c.a", &["apt-pkg-c/lib.cpp"]);
}
