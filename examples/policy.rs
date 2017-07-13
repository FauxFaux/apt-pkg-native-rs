extern crate apt_pkg_native;
use std::env;

use apt_pkg_native::simple;
use apt_pkg_native::Cache;

fn main() {
    let pkg = env::args().nth(1).expect("usage: first argument: package name");

    let mut cache = Cache::get_singleton();
    let found = cache.find_by_name(pkg.as_str());
    if found.is_empty() {
        println!("unrecognised package: {}", pkg);
        return;
    }

    println!("{}", found.pretty_print());

    for version in found.versions().map(simple::Version::from_iter) {
        println!("{:?}", version);
    }
}