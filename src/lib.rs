extern crate libc;

mod raw;
mod sane;
mod simple;

pub use sane::Cache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_all() {
        let mut cache = Cache::new();
        for name in cache.iter().map(|item| item.pretty_print()) {
            println!("{}", name);
        }
    }
}
