extern crate apt_pkg_native;

use apt_pkg_native::Cache;
use apt_pkg_native::simple;

fn main() {
    let mut cache = Cache::get_singleton();
    for item in cache.iter().map(simple::BinaryPackageVersions::new) {
        println!("{}: {:?}", item.pkg, item.versions);
    }
}
