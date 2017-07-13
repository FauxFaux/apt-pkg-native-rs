use std::marker::PhantomData;
use std::ffi;

use citer;
use libc;
use raw;

use citer::CIterator;

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
    pub fn iter(&mut self) -> CIterator<PkgIterator> {
        unsafe { PkgIterator::new(self, raw::pkg_cache_pkg_iter(self.ptr)) }
    }

    /// Find a package by name. It's not clear whether this picks a random arch,
    /// or the primary one.
    ///
    /// The returned iterator will either be at the end, or at a package with the name.
    pub fn find_by_name(&mut self, name: &str) -> CIterator<PkgIterator> {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let ptr = raw::pkg_cache_find_name(self.ptr, name.as_ptr());
            PkgIterator::new(self, ptr)
        }
    }

    /// Find a package by name and architecture.
    ///
    /// The returned iterator will either be at the end, or at a matching package.
    pub fn find_by_name_arch(&mut self, name: &str, arch: &str) -> CIterator<PkgIterator> {
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let arch = ffi::CString::new(arch).unwrap();
            let ptr = raw::pkg_cache_find_name_arch(self.ptr, name.as_ptr(), arch.as_ptr());
            PkgIterator::new(self, ptr)
        }
    }
}

/// An "iterator"/pointer to a point in a package list.
#[derive(Debug)]
pub struct PkgIterator<'c> {
    cache: &'c Cache,
    ptr: raw::PPkgIterator,
}

impl<'c> PkgIterator<'c> {
    fn new(cache: &'c Cache, ptr: raw::PCache) -> citer::CIterator<Self> {
        citer::CIterator {
            first: true,
            raw: PkgIterator { cache, ptr },
        }
    }
}

// TODO: could this be a ref to the iterator?
// TODO: Can't get the lifetimes to work.
pub struct PkgView<'c> {
    ptr: raw::PPkgIterator,
    cache: &'c Cache,
}

impl<'c> citer::RawIterator for PkgIterator<'c> {
    type View = PkgView<'c>;

    fn is_end(&self) -> bool {
        unsafe { raw::pkg_iter_end(self.ptr) }
    }

    fn next(&mut self) {
        unsafe { raw::pkg_iter_next(self.ptr) }
    }

    fn as_view(&self) -> Self::View {
        assert!(!self.is_end());

        PkgView {
            ptr: self.ptr,
            cache: self.cache,
        }
    }

    fn release(&mut self) {
        unsafe { raw::pkg_iter_release(self.ptr) }
    }
}


/// Actual accessors
impl<'c> PkgView<'c> {
    pub fn name(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_name(self.ptr))
                .expect("packages always have names")
        }
    }

    pub fn arch(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_iter_arch(self.ptr))
                .expect("packages always have architectures")
        }
    }

    pub fn current_version(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_iter_current_version(self.ptr)) }
    }

    pub fn candidate_version(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_iter_candidate_version(self.ptr)) }
    }

    pub fn versions(&self) -> CIterator<VerIterator> {
        CIterator {
            first: true,
            raw: VerIterator {
                cache: PhantomData,
                ptr: unsafe { raw::pkg_iter_ver_iter(self.ptr) },
            },
        }
    }
}

/// An "iterator"/pointer to a point in a version list.
pub struct VerIterator<'c> {
    cache: PhantomData<&'c Cache>,
    ptr: raw::PVerIterator,
}

// TODO: could this be a ref to the iterator?
// TODO: Can't get the lifetimes to work.
pub struct VerView<'c> {
    cache: PhantomData<&'c Cache>,
    ptr: raw::PVerIterator,
}

impl<'c> citer::RawIterator for VerIterator<'c> {
    type View = VerView<'c>;

    fn is_end(&self) -> bool {
        unsafe { raw::ver_iter_end(self.ptr) }
    }

    fn next(&mut self) {
        unsafe { raw::ver_iter_next(self.ptr) }
    }

    fn as_view(&self) -> Self::View {
        assert!(!self.is_end());

        VerView {
            ptr: self.ptr,
            cache: self.cache,
        }
    }

    fn release(&mut self) {
        unsafe { raw::ver_iter_release(self.ptr) }
    }
}

/// Actual accessors
impl<'c> VerView<'c> {
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

    pub fn section(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::ver_iter_section(self.ptr)) }
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
