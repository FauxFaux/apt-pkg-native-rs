extern crate cc;

const SRC: &str = "apt-pkg-c/lib.cpp";

fn main() {
    println!("cargo:rerun-if-changed={SRC}");

    let mut build = cc::Build::new();
    build.file(SRC);
    build.cpp(true);
    build.flag("-std=gnu++17");

    #[cfg(feature = "ye-olde-apt")]
    {
        build.define("YE_OLDE_APT", "1");
    }

    build.compile("libapt-pkg-c.a");

    println!("cargo:rustc-link-lib=apt-pkg");
}
