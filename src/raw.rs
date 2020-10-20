/// In general:
///  * `*mut c_void` are to be released by the appropriate function
///  * `*const c_chars` are short-term borrows
///  * `*mut c_chars` are to be freed by `libc::free`.
use std::sync::Mutex;

use lazy_static::lazy_static;
use libc::c_char;
use libc::c_void;

pub type PCache = *mut c_void;
pub type PPkgIterator = *mut c_void;
pub type PVerIterator = *mut c_void;
pub type PDepIterator = *mut c_void;
pub type PVerFileIterator = *mut c_void;
pub type PPkgFileIterator = *mut c_void;
pub type PVerFileParser = *mut c_void;

#[link(name = "apt-pkg-c", kind = "static")]
#[link(name = "apt-pkg")]
extern "C" {
    /// Must be called exactly once, before anything else?
    fn init_config_system();
    fn pkg_cache_create() -> PCache;
    fn pkg_cache_release(cache: PCache);

    pub fn pkg_cache_compare_versions(
        cache: PCache,
        left: *const c_char,
        right: *const c_char,
    ) -> i32;

    // Package iterators
    // =================

    pub fn pkg_cache_pkg_iter(cache: PCache) -> PPkgIterator;
    pub fn pkg_cache_find_name(cache: PCache, name: *const c_char) -> PPkgIterator;
    pub fn pkg_cache_find_name_arch(
        cache: PCache,
        name: *const c_char,
        arch: *const c_char,
    ) -> PPkgIterator;
    pub fn pkg_iter_release(iterator: PPkgIterator);

    pub fn pkg_iter_next(iterator: PPkgIterator);
    pub fn pkg_iter_end(iterator: PPkgIterator) -> bool;

    // Package iterator accessors
    // ==========================

    pub fn pkg_iter_name(iterator: PPkgIterator) -> *const c_char;
    pub fn pkg_iter_arch(iterator: PPkgIterator) -> *const c_char;
    pub fn pkg_iter_current_version(iterator: PPkgIterator) -> *const c_char;
    pub fn pkg_iter_candidate_version(iterator: PPkgIterator) -> *const c_char;

    // Version iterators
    // =================

    pub fn pkg_iter_ver_iter(pkg: PPkgIterator) -> PVerIterator;
    pub fn ver_iter_release(iterator: PVerIterator);

    pub fn ver_iter_next(iterator: PVerIterator);
    pub fn ver_iter_end(iterator: PVerIterator) -> bool;

    // Version accessors
    // =================

    pub fn ver_iter_version(iterator: PVerIterator) -> *mut c_char;
    pub fn ver_iter_section(iterator: PVerIterator) -> *mut c_char;

    #[cfg(not(feature = "ye-olde-apt"))]
    pub fn ver_iter_source_package(iterator: PVerIterator) -> *mut c_char;

    #[cfg(not(feature = "ye-olde-apt"))]
    pub fn ver_iter_source_version(iterator: PVerIterator) -> *mut c_char;
    pub fn ver_iter_arch(iterator: PVerIterator) -> *mut c_char;
    pub fn ver_iter_priority_type(iterator: PVerIterator) -> *mut c_char;

    #[cfg(not(feature = "ye-olde-apt"))]
    pub fn ver_iter_priority(iterator: PVerIterator) -> i32;

    // Dependency iterators
    // ====================

    pub fn ver_iter_dep_iter(iterator: PVerIterator) -> PDepIterator;
    pub fn dep_iter_release(iterator: PDepIterator);

    pub fn dep_iter_next(iterator: PDepIterator);
    pub fn dep_iter_end(iterator: PDepIterator) -> bool;

    // Dependency accessors
    // ====================

    pub fn dep_iter_target_pkg(iterator: PDepIterator) -> PPkgIterator;
    pub fn dep_iter_target_ver(iterator: PDepIterator) -> *const c_char;
    pub fn dep_iter_comp_type(iterator: PDepIterator) -> *const c_char;
    pub fn dep_iter_dep_type(iterator: PDepIterator) -> *const c_char;

    pub fn ver_iter_ver_file_iter(iterator: PVerIterator) -> PVerFileIterator;
    pub fn ver_file_iter_release(iterator: PVerFileIterator);

    pub fn ver_file_iter_next(iterator: PVerFileIterator);
    pub fn ver_file_iter_end(iterator: PVerFileIterator) -> bool;

    pub fn ver_file_iter_get_parser(iterator: PVerFileIterator) -> PVerFileParser;
    pub fn ver_file_parser_short_desc(parser: PVerFileParser) -> *const c_char;
    pub fn ver_file_parser_long_desc(parser: PVerFileParser) -> *const c_char;
    pub fn ver_file_parser_maintainer(parser: PVerFileParser) -> *const c_char;
    pub fn ver_file_parser_homepage(parser: PVerFileParser) -> *const c_char;

    pub fn ver_file_iter_pkg_file_iter(iterator: PVerFileIterator) -> PPkgFileIterator;
    pub fn pkg_file_iter_release(iterator: PPkgFileIterator);

    pub fn pkg_file_iter_next(iterator: PPkgFileIterator);
    pub fn pkg_file_iter_end(iterator: PPkgFileIterator) -> bool;

    pub fn pkg_file_iter_file_name(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_archive(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_version(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_origin(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_codename(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_label(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_site(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_component(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_architecture(iterator: PPkgFileIterator) -> *const c_char;
    pub fn pkg_file_iter_index_type(iterator: PPkgFileIterator) -> *const c_char;
}

pub fn pkg_cache_get_singleton() -> &'static CACHE_SINGLETON {
    &CACHE_SINGLETON
}

#[derive(Debug)]
pub struct CacheHolder {
    pub ptr: PCache,
}

unsafe impl Send for CacheHolder {}

impl CacheHolder {
    pub fn re_up(&mut self) {
        unsafe {
            pkg_cache_release(self.ptr);
            self.ptr = pkg_cache_create();
        }
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CACHE_SINGLETON: Mutex<CacheHolder> = {
        unsafe {
            init_config_system();
            Mutex::new(CacheHolder {
                ptr: pkg_cache_create()
            })
        }
    };
}
