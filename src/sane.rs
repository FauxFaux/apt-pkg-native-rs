use std::marker::PhantomData;
use std::ffi;

use libc;
use raw;

/// A reference to the package cache singleton,
/// from which most functionality can be accessed.
#[derive(Debug)]
pub struct Cache {
    ptr: raw::PCache,
}

impl Cache {
    /// Get a reference to the singleton.
    pub fn get_singleton() -> Cache {
        Cache { ptr: raw::pkg_cache_get() }
    }

    /// Walk through all of the packages, in a random order.
    ///
    /// If there are multiple architectures, multiple architectures will be returned.
    ///
    /// See the module documentation for apologies about how this isn't an iterator.
    pub fn iter(&mut self) -> PkgIterator {
        unsafe {
            PkgIterator {
                cache: self,
                first: true,
                ptr: raw::pkg_cache_pkg_iter(self.ptr),
            }
        }
    }

    /// Find a package by name. It's not clear whether this picks a random arch,
    /// or the primary one.
    ///
    /// The returned iterator will either be at the end, or at a package with the name.
    pub fn find_by_name(&mut self, name: &str) -> PkgIterator {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let ptr = raw::pkg_cache_find_name(self.ptr, name.as_ptr());
            PkgIterator {
                cache: self,
                first: true,
                ptr,
            }
        }
    }

    /// Find a package by name and architecture.
    ///
    /// The returned iterator will either be at the end, or at a matching package.
    pub fn find_by_name_arch(&mut self, name: &str, arch: &str) -> PkgIterator {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let arch = ffi::CString::new(arch).unwrap();
            let ptr = raw::pkg_cache_find_name_arch(self.ptr, name.as_ptr(), arch.as_ptr());
            PkgIterator {
                cache: self,
                first: true,
                ptr,
            }
        }
    }
}

/// An "iterator"/pointer to a point in a package list.
#[derive(Debug)]
pub struct PkgIterator<'c> {
    cache: &'c Cache,
    first: bool,
    ptr: raw::PPkgIterator,
}

impl<'c> Drop for PkgIterator<'c> {
    fn drop(&mut self) {
        unsafe { raw::pkg_iter_release(self.ptr) }
    }
}

/// Iterator-like interface
impl<'c> PkgIterator<'c> {
    pub fn next(&mut self) -> Option<&Self> {
        unsafe {
            // we were at the end last time, leave us alone!
            if self.is_empty() {
                return None;
            }

            if !self.first {
                raw::pkg_iter_next(self.ptr);
            }

            self.first = false;

            // we don't want to observe the end marker
            if self.is_empty() { None } else { Some(self) }
        }
    }

    /// Check if we're at the end of the iteration.
    /// Not useful/necessary if you're using `next()`,
    /// but useful for `find_..`.
    pub fn is_empty(&self) -> bool {
        // TODO: Can we get this inlined such that all the asserts will be eliminated?
        unsafe { raw::pkg_iter_end(self.ptr) }
    }

    pub fn map<F, B>(self, f: F) -> PkgMap<'c, F>
    where
        F: FnMut(&PkgIterator) -> B,
    {
        PkgMap { it: self, f }
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
        unsafe { make_owned_ascii_string(raw::pkg_iter_current_version(self.ptr)) }
    }

    pub fn candidate_version(&self) -> Option<String> {
        assert!(!self.is_empty());
        unsafe { make_owned_ascii_string(raw::pkg_iter_candidate_version(self.ptr)) }
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
            result
        }
    }

    pub fn versions(&self) -> VerIterator {
        VerIterator {
            cache: PhantomData,
            first: true,
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
where
    F: FnMut(&PkgIterator) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(&mut self.f)
    }
}

/// An "iterator"/pointer to a point in a version list.
pub struct VerIterator<'c> {
    cache: PhantomData<&'c Cache>,
    first: bool,
    ptr: raw::PVerIterator,
}

impl<'c> Drop for VerIterator<'c> {
    fn drop(&mut self) {
        unsafe { raw::ver_iter_release(self.ptr) }
    }
}

/// Iterator-like interface
impl<'c> VerIterator<'c> {
    pub fn next(&mut self) -> Option<&Self> {
        unsafe {
            // we were at the end last time, leave us alone!
            if self.is_empty() {
                return None;
            }

            if !self.first {
                raw::ver_iter_next(self.ptr);
            }

            self.first = false;

            // we don't want to observe the end marker
            if self.is_empty() { None } else { Some(self) }
        }
    }

    /// Check if we're at the end of the iteration.
    /// Not useful/necessary if you're using `next()`,
    /// but useful for `find_..`.
    pub fn is_empty(&self) -> bool {
        // TODO: Can we get this inlined such that all the asserts will be eliminated?
        unsafe { raw::ver_iter_end(self.ptr) }
    }

    pub fn map<F, B>(self, f: F) -> VerMap<'c, F>
    where
        F: FnMut(&VerIterator) -> B,
    {
        VerMap { it: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct VerMap<'c, F> {
    it: VerIterator<'c>,
    f: F,
}

impl<'c, B, F> Iterator for VerMap<'c, F>
where
    F: FnMut(&VerIterator) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(&mut self.f)
    }
}

/// Actual accessors
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

    pub fn priority(&self) -> i32 {
        unsafe { raw::ver_iter_priority(self.ptr) }
    }
}

unsafe fn make_owned_ascii_string(ptr: *const libc::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("value should always be low-ascii")
                .to_string(),
        )
    }
}
