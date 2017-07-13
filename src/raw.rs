use libc::c_void;
use libc::c_char;
use libc::c_int;

#[link(name = "apt-c")]
#[link(name = "apt-pkg")]
#[link(name = "stdc++")]
extern {
    pub fn get_pkg_cache() -> *mut c_void;
    pub fn free_pkg_cache(cache: *mut c_void);

    pub fn iterate_packages(
        cache: *mut c_void,
        visit: extern fn(name: *mut c_void) -> c_int,
    ) -> c_int;

    pub fn pkg_iter_name(iterator: *mut c_void) -> *const c_char;
    pub fn pkg_iter_pretty(cache: *mut c_void, iterator: *mut c_void) -> *mut c_char;
}
