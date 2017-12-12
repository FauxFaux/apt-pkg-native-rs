use std::cmp;
use std::ffi;
use std::marker::PhantomData;
use std::sync::MutexGuard;

use libc;
use raw;

use citer::CIterator;
use citer::RawIterator;

/// A reference to the package cache singleton,
/// from which most functionality can be accessed.
#[derive(Debug)]
pub struct Cache {
    ptr_mutex: &'static raw::CACHE_SINGLETON,
}

impl Cache {
    /// Get a reference to the singleton.
    pub fn get_singleton() -> Cache {
        Cache {
            ptr_mutex: raw::pkg_cache_get_singleton(),
        }
    }

    /// Drop the cache, and re-create it from scratch.
    ///
    /// It's super important that there are no other outstanding
    /// references to the cache at this point. Again, I remind you
    /// not to try and outsmart the borrow checker. It doesn't know
    /// how much trouble there is in here.
    pub fn reload(&mut self) {
        self.ptr_mutex.lock().expect("poisoned mutex").re_up()
    }

    /// Walk through all of the packages, in a random order.
    ///
    /// If there are multiple architectures, multiple architectures will be returned.
    ///
    /// See the module documentation for apologies about how this isn't an iterator.
    pub fn iter(&mut self) -> CIterator<PkgIterator> {
        let lock = self.ptr_mutex.lock().expect("poisoned mutex");
        unsafe {
            let raw_iter = raw::pkg_cache_pkg_iter(lock.ptr);
            PkgIterator::new(lock, raw_iter)
        }
    }

    /// Find a package by name. It's not clear whether this picks a random arch,
    /// or the primary one.
    ///
    /// The returned iterator will either be at the end, or at a package with the name.
    pub fn find_by_name(&mut self, name: &str) -> CIterator<PkgIterator> {
        let lock = self.ptr_mutex.lock().expect("poisoned mutex");
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let ptr = raw::pkg_cache_find_name(lock.ptr, name.as_ptr());
            PkgIterator::new(lock, ptr)
        }
    }

    /// Find a package by name and architecture.
    ///
    /// The returned iterator will either be at the end, or at a matching package.
    pub fn find_by_name_arch(&mut self, name: &str, arch: &str) -> CIterator<PkgIterator> {
        let lock = self.ptr_mutex.lock().expect("poisoned mutex");
        unsafe {
            let name = ffi::CString::new(name).unwrap();
            let arch = ffi::CString::new(arch).unwrap();
            let ptr = raw::pkg_cache_find_name_arch(lock.ptr, name.as_ptr(), arch.as_ptr());
            PkgIterator::new(lock, ptr)
        }
    }

    /// Compare two versions, returning an `Ordering`, as used by most Rusty `sort()` methods.
    ///
    /// This uses the "versioning scheme" currently set, which, in theory, can change,
    /// but in practice is always the "Standard .deb" scheme. As of 2017, there aren't even any
    /// other implementations. As such, this may eventually become a static method somewhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # let mut cache = apt_pkg_native::Cache::get_singleton();
    /// let mut packages = vec!["3.0", "3.1", "3.0~1"];
    /// packages.sort_by(|left, right| cache.compare_versions(left, right));
    /// assert_eq!(vec!["3.0~1", "3.0", "3.1"], packages);
    /// ```
    pub fn compare_versions(&self, left: &str, right: &str) -> cmp::Ordering {
        unsafe {
            let left = ffi::CString::new(left).unwrap();
            let right = ffi::CString::new(right).unwrap();

            let lock = self.ptr_mutex.lock().expect("poisoned mutex");
            raw::pkg_cache_compare_versions(lock.ptr, left.as_ptr(), right.as_ptr()).cmp(&0)
        }
    }
}

/// An "iterator"/pointer to a point in a package list.
#[derive(Debug)]
pub struct PkgIterator<'c> {
    cache: MutexGuard<'c, raw::CacheHolder>,
    ptr: raw::PPkgIterator,
}

impl<'c> PkgIterator<'c> {
    fn new(cache: MutexGuard<'c, raw::CacheHolder>, ptr: raw::PCache) -> CIterator<Self> {
        CIterator {
            first: true,
            raw: PkgIterator { cache, ptr },
        }
    }
}

// TODO: could this be a ref to the iterator?
// TODO: Can't get the lifetimes to work.
pub struct PkgView<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PPkgIterator,
}

impl<'c> RawIterator for PkgIterator<'c> {
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
            cache: PhantomData,
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
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerIterator,
}

pub struct VerView<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerIterator,
}

impl<'c> RawIterator for VerIterator<'c> {
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

    #[cfg(not(feature="ye-olde-apt"))]
    pub fn source_package(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_source_package(self.ptr))
                .expect("versions always have a source package")
        }
    }

    #[cfg(not(feature="ye-olde-apt"))]
    pub fn source_version(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::ver_iter_source_version(self.ptr))
                .expect("versions always have a source_version")
        }
    }

    #[cfg(not(feature="ye-olde-apt"))]
    pub fn priority(&self) -> i32 {
        unsafe { raw::ver_iter_priority(self.ptr) }
    }

    pub fn origin_iter(&self) -> CIterator<VerFileIterator> {
        CIterator {
            first: true,
            raw: VerFileIterator {
                cache: PhantomData,
                ptr: unsafe { raw::ver_iter_ver_file_iter(self.ptr) },
            },
        }
    }
}

/// An "iterator"/pointer to a point in a version's file list(?).
pub struct VerFileIterator<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerFileIterator,
}

// TODO: could this be a ref to the iterator?
// TODO: Can't get the lifetimes to work.
pub struct VerFileView<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerFileIterator,
}


impl<'c> RawIterator for VerFileIterator<'c> {
    type View = VerFileView<'c>;

    fn is_end(&self) -> bool {
        unsafe { raw::ver_file_iter_end(self.ptr) }
    }

    fn next(&mut self) {
        unsafe { raw::ver_file_iter_next(self.ptr) }
    }

    fn as_view(&self) -> Self::View {
        assert!(!self.is_end());

        VerFileView {
            ptr: self.ptr,
            cache: self.cache,
        }
    }

    fn release(&mut self) {
        unsafe { raw::ver_file_iter_release(self.ptr) }
    }
}

impl<'c> VerFileView<'c> {
    pub fn file(&self) -> CIterator<PkgFileIterator> {
        CIterator {
            first: true,
            raw: PkgFileIterator {
                cache: PhantomData,
                ptr: unsafe { raw::ver_file_iter_pkg_file_iter(self.ptr) },
            },
        }
    }
}


/// An "iterator"/pointer to a point in a file list.
pub struct PkgFileIterator<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerFileIterator,
}

// TODO: could this be a ref to the iterator?
// TODO: Can't get the lifetimes to work.
pub struct PkgFileView<'c> {
    cache: PhantomData<&'c MutexGuard<'c, raw::CacheHolder>>,
    ptr: raw::PVerFileIterator,
}

impl<'c> RawIterator for PkgFileIterator<'c> {
    type View = PkgFileView<'c>;

    fn is_end(&self) -> bool {
        unsafe { raw::pkg_file_iter_end(self.ptr) }
    }

    fn next(&mut self) {
        unsafe { raw::pkg_file_iter_next(self.ptr) }
    }

    fn as_view(&self) -> Self::View {
        assert!(!self.is_end());

        PkgFileView {
            ptr: self.ptr,
            cache: self.cache,
        }
    }

    fn release(&mut self) {
        unsafe { raw::pkg_file_iter_release(self.ptr) }
    }
}

impl<'c> PkgFileView<'c> {
    pub fn file_name(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_file_iter_file_name(self.ptr))
                .expect("package file always has a file name")
        }
    }
    pub fn archive(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_file_iter_archive(self.ptr))
                .expect("package file always has an archive")
        }
    }
    pub fn version(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_version(self.ptr)) }
    }
    pub fn origin(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_origin(self.ptr)) }
    }
    pub fn codename(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_codename(self.ptr)) }
    }
    pub fn label(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_label(self.ptr)) }
    }
    pub fn site(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_site(self.ptr)) }
    }
    pub fn component(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_file_iter_component(self.ptr))
                .expect("package file always has a component")
        }
    }
    pub fn architecture(&self) -> Option<String> {
        unsafe { make_owned_ascii_string(raw::pkg_file_iter_architecture(self.ptr)) }
    }
    pub fn index_type(&self) -> String {
        unsafe {
            make_owned_ascii_string(raw::pkg_file_iter_index_type(self.ptr))
                .expect("package file always has a index_type")
        }
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
