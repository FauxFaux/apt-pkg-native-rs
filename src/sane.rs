use std::ffi;

use libc;
use raw;

/// A reference to the package cache singleton.
/// Basically just a collection of related methods.
#[derive(Debug)]
pub struct Cache {
    ptr: raw::PCache
}

impl Cache {
    pub fn get_singleton() -> Cache {
        Cache {
            ptr: raw::pkg_cache_get()
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

    pub fn find_by_name(&mut self, name: &str) -> PkgIterator {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let ptr = raw::pkg_cache_find_name(self.ptr, name.as_ptr());
            PkgIterator {
                cache: self,
                ptr,
            }
        }
    }

    pub fn find_by_name_arch(&mut self, name: &str, arch: &str) -> PkgIterator {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let arch = ffi::CString::new(arch).unwrap();
            let ptr = raw::pkg_cache_find_name_arch(self.ptr, name.as_ptr(), arch.as_ptr());
            PkgIterator {
                cache: self,
                ptr,
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
            if self.is_empty() {
                return None;
            }

            raw::pkg_iter_next(self.ptr);

            // we don't want to observe the end marker
            if self.is_empty() {
                None
            } else {
                Some(self)
            }
        }
    }

    /// Check if we're at the end of the iteration.
    /// Not useful/necessary if you're using `next()`,
    /// but useful for `find_..`.
    pub fn is_empty(&self) -> bool {
        // TODO: Can we get this inlined such that all the asserts will be eliminated?
        unsafe {
            raw::pkg_iter_end(self.ptr)
        }
    }

    pub fn map<F, B>(self, f: F) -> PkgMap<'c, F>
    where F: FnMut(&PkgIterator) -> B {
        PkgMap {
            it: self,
            f,
        }
    }
}

/// Actual accessors
impl<'c> PkgIterator<'c> {
    pub fn name(&self) -> String {
        assert!(!self.is_empty());
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_name(self.ptr))
                .expect("packages always have names")
        }
    }

    pub fn arch(&self) -> String {
        assert!(!self.is_empty());
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_arch(self.ptr))
                .expect("packages always have architectures")
        }
    }

    pub fn current_version(&self) -> Option<String> {
        assert!(!self.is_empty());
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_current_version(self.ptr))
        }
    }

    pub fn pretty_print(&self) -> String {
        assert!(!self.is_empty());
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

    pub fn versions(&self) -> VerIterator {
        VerIterator {
            cache: self.cache,
            ptr: unsafe { raw::pkg_iter_ver_iter(self.ptr) },
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

pub struct VerIterator<'c> {
    cache: &'c Cache,
    ptr: raw::PVerIterator,
}

impl<'c> Drop for VerIterator<'c> {
    fn drop(&mut self) {
        unsafe {
            raw::ver_iter_release(self.ptr)
        }
    }
}

impl<'c> VerIterator<'c> {
    pub fn next<'i>(&'i mut self) -> Option<&'i Self> {
        unsafe {
            // we were at the end last time, leave us alone!
            if self.is_empty() {
                return None;
            }

            raw::ver_iter_next(self.ptr);

            // we don't want to observe the end marker
            if self.is_empty() {
                None
            } else {
                Some(self)
            }
        }
    }

    /// Check if we're at the end of the iteration.
    /// Not useful/necessary if you're using `next()`,
    /// but useful for `find_..`.
    pub fn is_empty(&self) -> bool {
        // TODO: Can we get this inlined such that all the asserts will be eliminated?
        unsafe {
            raw::ver_iter_end(self.ptr)
        }
    }

    pub fn map<F, B>(self, f: F) -> VerMap<'c, F>
    where F: FnMut(&VerIterator) -> B {
        VerMap {
            it: self,
            f,
        }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct VerMap<'c, F> {
    it: VerIterator<'c>,
    f: F,
}

impl<'c, B, F> Iterator for VerMap<'c, F>
where F: FnMut(&VerIterator) -> B {

    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(&mut self.f)
    }
}

impl<'c> VerIterator<'c> {
    pub fn version(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_version(self.ptr))
                .expect("versions always have a version")
        }
    }

    pub fn arch(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_arch(self.ptr))
                .expect("versions always have an arch")
        }
    }

    pub fn section(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_section(self.ptr))
                .expect("versions always have a section")
        }
    }

    pub fn source_package(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_source_package(self.ptr))
                .expect("versions always have a source package")
        }
    }

    pub fn source_version(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_source_version(self.ptr))
                .expect("versions always have a source_version")
        }
    }
}

unsafe fn make_owned_ascii_string(ptr: *const libc::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(ffi::CStr::from_ptr(ptr)
            .to_str()
            .expect("value should always be low-ascii")
            .to_string())
    }
}
