#[macro_use] extern crate lazy_static;
extern crate libc;

mod raw;
mod sane;
mod simple;

pub use sane::Cache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_all() {
        let mut cache = Cache::get_singleton();
        assert!(cache.iter().map(|item| item.pretty_print()).count() > 0);
    }

    #[test]
    fn find_a_package() {
        let mut cache = Cache::get_singleton();
        {
            let iter = cache.find_by_name("apt");
            assert!(!iter.is_empty());
            assert_eq!("apt", iter.name());
        }

        {
            let iter = cache.find_by_name("this-package-doesnt-exist-and-if-someone-makes-it-ill-be-really-angry");
            assert!(iter.is_empty());
        }
    }
}
