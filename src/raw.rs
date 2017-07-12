use libc::c_void;
use libc::c_char;
use libc::c_int;

#[link(name = "apt-c")]
#[link(name = "apt-pkg")]
#[link(name = "stdc++")]
extern {
    pub fn get_pkg_cache() -> *mut c_void;
    pub fn free_pkg_cache(cache: *mut c_void);

    pub fn iterate_all_packages(
        cache: *mut c_void,
        visit: extern fn(name: *const c_char) -> c_int,
    ) -> c_int;
}
