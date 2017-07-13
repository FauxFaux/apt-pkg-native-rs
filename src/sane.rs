use std::ffi;

use libc;
use raw;

// Probably not cloneable / copyable.
/// You might only be able to create one of these per process.
#[derive(Debug)]
pub struct Cache {
    ptr: raw::PCache
}

impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            raw::pkg_cache_release(self.ptr)
        }
    }
}

impl Cache {
    pub fn new() -> Cache {
        unsafe {
            raw::init_config_system_once();
            Cache {
                ptr: raw::pkg_cache_create()
            }
        }
    }

    pub fn iter(&mut self) -> PkgIterator {
        unsafe {
            PkgIterator {
                cache: self,
                ptr: raw::pkg_cache_pkg_iter(self.ptr)
            }
        }
    }
}


#[derive(Debug)]
pub struct PkgIterator<'c> {
    cache: &'c Cache,
    ptr: raw::PPkgIterator
}

impl<'c> Drop for PkgIterator<'c> {
    fn drop(&mut self) {
        unsafe {
            raw::pkg_iter_release(self.ptr)
        }
    }
}

/// Iterator-like interface.
/// Can't implement Iterator due to the mutation / lifetime constraints?
impl<'c> PkgIterator<'c> {
    pub fn next<'i>(&'i mut self) -> Option<&'i Self> {
        unsafe {
            // we were at the end last time, leave us alone!
            if raw::pkg_iter_end(self.ptr) {
                return None;
            }

            raw::pkg_iter_next(self.ptr);

            // we don't want to observe the end marker
            if raw::pkg_iter_end(self.ptr) {
                None
            } else {
                Some(self)
            }
        }
    }

    pub fn count(mut self) -> usize {
        let mut count = 0;
        loop {
            if self.next().is_none() {
                break;
            }

            count += 1;
        }

        count
    }

    pub fn map<F, B>(self, f: F) -> PkgMap<'c, F>
    where F: FnMut(&PkgIterator) -> B {
        PkgMap {
            it: self,
            f,
        }
    }
}

unsafe fn make_owned_ascii_string(ptr: *const libc::c_char) -> String {
    ffi::CStr::from_ptr(ptr)
        .to_str()
        .expect("value should always be low-ascii")
        .to_string()
}

/// Actual accessors
impl<'c> PkgIterator<'c> {
    pub fn name(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_name(self.ptr))
        }
    }

    pub fn arch(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_arch(self.ptr))
        }
    }

    pub fn current_version(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_current_version(self.ptr))
        }
    }

    pub fn pretty_print(&self) -> String {
        unsafe {
            let ptr = raw::pkg_iter_pretty(self.cache.ptr, self.ptr);
            let result = ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("package names are always low-ascii")
                .to_string();
            libc::free(ptr as *mut libc::c_void);
            return result;
        }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct PkgMap<'c, F> {
    it: PkgIterator<'c>,
    f: F,
}

impl<'c, B, F> Iterator for PkgMap<'c, F>
where F: FnMut(&PkgIterator) -> B {

    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(&mut self.f)
    }
}
