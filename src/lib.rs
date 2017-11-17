//! Here lies bindings to `libapt-pkg`, which is what the `apt`, `apt-get`, `apt-cache`, etc.
//! commands use to view and manipulate the state of packages on the system.
//!
//! Currently, not much is exposed. You can pretty much only view basic package
//! information, like with `apt-cache policy foo`.
//!
//! `libapt-pkg` has basically no documentation. `python-apt` is slightly better,
//! but is also pretty inconsistent on the documentation front. The design of this
//! crate is closer to `libapt-pkg`, despite it being pretty insane.
//!
//! The core concept here is an "iterator". Forget everything you know about iterators,
//! these iterators are pretty much pointers. The crate attempts to make them act
//! a bit more like Rust `Iterator`s, but is crippled by the insanity.
//!
//! Methods which "find" something will reposition one of these "iterators" at the right place
//! in an existing stream of items.
//!
//! I recommend using `.map()` to turn an "iterator" into a Rust type as soon as possible.
//! The returned map-like thing *is* a Rust `Iterator`, so you can do normal operations on it.
//!
//! Here's an example: normally you wouldn't need this ugly `.map(|_| ())` (read as "map anything
//! to the empty object"), but here, it is *also* converting a sh... apt "iterator" into a
//! real Iterator.
//!
//! ```rust,no_run
//! extern crate apt_pkg_native;
//! let mut cache = apt_pkg_native::Cache::get_singleton();
//! let total_packages = cache.iter().map(|_| ()).count();
//! ```
//!
//! `libapt-pkg` also just segfaults if you do anything wrong, or re-use anything at the wrong time,
//! or etc. I've tried to hide this, but I advise you not to push or outsmart the borrow checker.

#[macro_use]
extern crate lazy_static;
extern crate libc;

mod citer;
mod raw;
mod sane;
pub mod simple;

pub use sane::Cache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_all() {
        let mut cache = Cache::get_singleton();
        let read_all_and_count = cache.iter().map(simple::BinaryPackageVersions::new).count();
        assert!(read_all_and_count > 2);
        assert_eq!(read_all_and_count, cache.iter().count());
    }

    #[test]
    fn find_a_package() {
        let mut cache = Cache::get_singleton();

        if let Some(view) = cache.find_by_name("apt").next() {
            assert_eq!("apt", view.name());
        } else {
            panic!("not found!");
        }

        assert!(
            cache
                .find_by_name(
                    "this-package-doesnt-exist-and-if-someone-makes-it-ill-be-really-angry"
                )
                .next()
                .is_none()
        );
    }

    #[test]
    fn find_by_filter_map() {
        let mut cache = Cache::get_singleton();
    }

    #[test]
    fn compare_versions() {
        use std::cmp::Ordering;
        let cache = Cache::get_singleton();
        assert_eq!(Ordering::Less, cache.compare_versions("3.0", "3.1"));
        assert_eq!(Ordering::Greater, cache.compare_versions("3.1", "3.0"));
        assert_eq!(Ordering::Equal, cache.compare_versions("3.0", "3.0"));
    }

    #[test]
    fn reload() {
        let mut cache = Cache::get_singleton();
        cache.reload();
        cache.reload();
        cache.reload();
        cache.reload();
    }
}
