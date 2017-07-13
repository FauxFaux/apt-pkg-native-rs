extern crate libc;

pub mod raw;
pub mod sane;

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CStr;

    // Leaks on panic.
    #[test]
    fn goin_in_raw() {
        unsafe {
            let cache = raw::pkg_cache_create();
            let iter = raw::pkg_cache_pkg_iter(cache);
            loop {
                println!("{}", CStr::from_ptr(raw::pkg_iter_name(iter)).to_str().expect("package names are always valid utf-8, in that they're always valid low ascii"));
                let pretty = raw::pkg_iter_pretty(cache, iter);
                println!("{}", CStr::from_ptr(pretty).to_str().expect("package names are always valid utf-8, in that they're always valid low ascii"));
                libc::free(pretty as *mut libc::c_void);
                raw::pkg_iter_next(iter);
                if raw::pkg_iter_end(iter) {
                    break;
                }
            }
            raw::pkg_iter_release(iter);
            raw::pkg_cache_release(cache);
        }
    }

    #[test]
    fn list_all() {
        let mut cache = sane::Cache::new();
        for name in cache.iter().map(|item: &sane::PkgIterator| item.pretty_print()) {
            println!("{}", name);
        }
    }
}
