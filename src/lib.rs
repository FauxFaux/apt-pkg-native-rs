extern crate libc;

pub mod raw;

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CStr;

    extern fn print_pkg(iter: *mut libc::c_void) -> libc::c_int {
        unsafe {
            println!("{}", CStr::from_ptr(raw::pkg_iter_name(iter)).to_str().expect("package names are always valid utf-8, in that they're always valid low ascii"));
            let pretty = raw::pkg_iter_pretty(cache, iter);
            println!("{}", CStr::from_ptr(pretty).to_str().expect("package names are always valid utf-8, in that they're always valid low ascii"));
            libc::free(pretty as *mut libc::c_void);
            return 1;
        }
    }

    #[test]
    fn it_works() {
        unsafe {
            let cache = raw::get_pkg_cache();
            println!("{:?}", raw::iterate_packages(cache, print_pkg));
            raw::free_pkg_cache(cache);
        }
    }
}
