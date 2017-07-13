extern crate libc;

mod raw;
mod sane;
mod simple;

pub use sane::Cache;

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
    fn list_all() {
        let mut cache = Cache::new();
        for name in cache.iter().map(|item| item.pretty_print()) {
            println!("{}", name);
        }
    }

    #[test]
    fn find_a_package() {
        let mut cache = Cache::new();
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
