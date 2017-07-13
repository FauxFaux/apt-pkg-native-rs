/// In general:
///  * *mut c_void are to be released by the appropriate function
///  * *const c_chars are short-term borrows
///  * *mut c_chars are to be freed by libc::free.

use libc::c_void;
use libc::c_char;

pub type PCache = *mut c_void;
pub type PPkgIterator = *mut c_void;

#[link(name = "apt-c")]
#[link(name = "apt-pkg")]
#[link(name = "stdc++")]
extern {
    pub fn pkg_cache_create() -> PCache;
    pub fn pkg_cache_release(cache: PCache);

    pub fn pkg_cache_pkg_iter(cache: PCache) -> PPkgIterator;
    pub fn pkg_iter_release(iterator: PPkgIterator);

    pub fn pkg_iter_next(iterator: PPkgIterator);
    pub fn pkg_iter_end(iterator: PPkgIterator) -> bool;

    pub fn pkg_iter_name(iterator: PPkgIterator) -> *const c_char;
    pub fn pkg_iter_pretty(cache: PCache, iterator: PPkgIterator) -> *mut c_char;
}
