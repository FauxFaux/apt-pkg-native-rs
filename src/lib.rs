extern crate libc;

pub mod raw;

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
                if !raw::pkg_iter_next(iter) {
                    break;
                }
            }
            raw::pkg_iter_release(iter);
            raw::pkg_cache_release(cache);
        }
    }
}
