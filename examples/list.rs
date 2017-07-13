extern crate apt_pkg_native;

use apt_pkg_native::Cache;

fn main() {
    let mut cache = Cache::get_singleton();
    for item in cache.iter().map(|item| item.pretty_print()) {
        println!("{}", item);
    }
}
