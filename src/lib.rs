extern crate libc;

pub mod raw;
pub mod sane;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_all() {
        let mut cache = sane::Cache::new();
        for name in cache.iter().map(|item: &sane::PkgIterator| item.pretty_print()) {
            println!("{}", name);
        }
    }
}
