//! Some structs representing basic concepts, and utilities to copy out of "iterators".

use std::fmt;

use sane;

#[derive(Clone, Debug)]
pub struct BinaryPackage {
    pub name: String,
    pub arch: String,
    pub current_version: Option<String>,
    pub candidate_version: Option<String>,
}

impl BinaryPackage {
    pub fn new(view: &sane::PkgView) -> Self {
        BinaryPackage {
            name: view.name(),
            arch: view.arch(),
            current_version: view.current_version(),
            candidate_version: view.candidate_version(),
        }
    }
}

impl fmt::Display for BinaryPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.arch)?;
        if let Some(ref version) = self.current_version {
            write!(f, " @ {}", version)?;
        }
        if let Some(ref version) = self.candidate_version {
            write!(f, " -> {}", version)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    pub version: String,
    pub arch: String,
    pub section: Option<String>,
    pub source_package: String,
    pub source_version: String,
    pub priority: i32,
}


impl Version {
    pub fn new(view: &sane::VerView) -> Self {
        Version {
            version: view.version(),
            arch: view.arch(),
            section: view.section(),
            source_package: view.source_package(),
            source_version: view.source_version(),
            priority: view.priority(),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.version, self.arch)?;
        if let Some(ref section) = self.section {
            write!(f, " in {}", section)?;
        }
        write!(f, " from {}:{} at {}",
            self.source_package,
            self.source_version,
            self.priority,
        )
    }
}


#[derive(Clone, Debug)]
pub struct BinaryPackageVersions {
    pub pkg: BinaryPackage,
    pub versions: Vec<Version>,
}

impl BinaryPackageVersions {
    pub fn new(view: &sane::PkgView) -> Self {
        BinaryPackageVersions {
            pkg: BinaryPackage::new(view),
            versions: view.versions().map(Version::new).collect(),
        }
    }
}

impl fmt::Display for BinaryPackageVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {} versions", self.pkg, self.versions.len())
    }
}
