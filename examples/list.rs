use itertools::Itertools;

use apt_pkg_native::simple;
use apt_pkg_native::Cache;

fn main() {
    let mut cache = Cache::get_singleton();
    for item in cache.iter().map(simple::BinaryPackageVersions::new) {
        println!(
            "{} [{}]",
            item.pkg,
            item.versions.iter().map(|x| format!("{}", x)).join(", ")
        );
    }
}
