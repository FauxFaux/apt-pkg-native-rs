extern crate libc;

pub mod raw;

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CStr;

    extern fn print_arg(arg: *const libc::c_char) -> libc::c_int {
        unsafe {
            println!("{:?}", CStr::from_ptr(arg).to_str());
            return 1;
        }
    }

    #[test]
    fn it_works() {
        unsafe {
            let cache = raw::get_pkg_cache();
            println!("{:?}", raw::iterate_all_packages(cache, print_arg));
            raw::free_pkg_cache(cache);
        }
    }
}
