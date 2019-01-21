use apt_pkg_native::Cache;

use boolinator::Boolinator;

fn main() {
    let mut cache = Cache::get_singleton();
    for item in cache.iter().filter_map(|f| {
        f.versions()
            .any(|version| version.version().contains(':'))
            .as_some_from(|| f.name())
    }) {
        println!("{}", item);
    }
}
