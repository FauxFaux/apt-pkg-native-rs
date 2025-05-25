//! Some structs representing basic concepts, and utilities to copy out of "iterators".

use std::fmt;

use crate::sane;

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
            write!(f, " @ {version}")?;
        }
        if let Some(ref version) = self.candidate_version {
            write!(f, " -> {version}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct VersionDetails {
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
    pub maintainer: Option<String>,
    pub homepage: Option<String>,
}

impl Default for VersionDetails {
    fn default() -> Self {
        Self {
            short_desc: None,
            long_desc: None,
            maintainer: None,
            homepage: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    pub version: String,
    pub arch: String,
    pub section: Option<String>,

    #[cfg(not(feature = "ye-olde-apt"))]
    pub source_package: String,
    #[cfg(not(feature = "ye-olde-apt"))]
    pub source_version: String,
    #[cfg(not(feature = "ye-olde-apt"))]
    pub priority: i32,

    pub details: VersionDetails,
}

impl Version {
    pub fn new(view: &sane::VerView) -> Self {
        // assume there is either zero or only one set of details per version
        let details = if let Some(ver_file) = view.origin_iter().next() {
            VersionDetails {
                short_desc: ver_file.short_desc(),
                long_desc: ver_file.long_desc(),
                maintainer: ver_file.maintainer(),
                homepage: ver_file.homepage(),
            }
        } else {
            Default::default()
        };

        Version {
            version: view.version(),
            arch: view.arch(),
            section: view.section(),
            #[cfg(not(feature = "ye-olde-apt"))]
            source_package: view.source_package(),
            #[cfg(not(feature = "ye-olde-apt"))]
            source_version: view.source_version(),
            #[cfg(not(feature = "ye-olde-apt"))]
            priority: view.priority(),
            details,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.version, self.arch)?;
        if let Some(ref section) = self.section {
            write!(f, " in {section}")?;
        }
        #[cfg(not(feature = "ye-olde-apt"))]
        write!(
            f,
            " from {}:{} at {}",
            self.source_package, self.source_version, self.priority,
        )?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Origin {
    pub file_name: String,
    pub archive: String,
    pub version: Option<String>,
    pub origin: Option<String>,
    pub codename: Option<String>,
    pub label: Option<String>,
    pub site: Option<String>,
    pub component: String,
    pub architecture: Option<String>,
    pub index_type: String,
}

impl Origin {
    pub fn from_ver_file(view: &sane::VerFileView) -> Option<Self> {
        view.file().next().map(|x| Self::new(&x))
    }

    pub fn new(view: &sane::PkgFileView) -> Self {
        Origin {
            file_name: view.file_name(),
            archive: view.archive(),
            version: view.version(),
            origin: view.origin(),
            codename: view.codename(),
            label: view.label(),
            site: view.site(),
            component: view.component(),
            architecture: view.architecture(),
            index_type: view.index_type(),
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // trying to simulate apt-cache policy, but a lot of information is missing
        if self.site.is_some()
            && self.origin.is_some()
            && self.label.is_some()
            && self.codename.is_some()
            && self.architecture.is_some()
        {
            write!(
                f,
                "TODO://{}/TODO(o:{}/l:{}/c:{}) {}/{} {} (f:{})",
                self.site.as_ref().unwrap(),
                self.origin.as_ref().unwrap(),
                self.label.as_ref().unwrap(),
                self.codename.as_ref().unwrap(),
                self.archive,
                self.component,
                self.architecture.as_ref().unwrap(),
                self.file_name
            )
        } else {
            write!(f, "{}", self.file_name)
        }
    }
}

#[derive(Clone, Debug)]
pub struct VersionOrigins {
    pub version: Version,
    pub origins: Vec<Origin>,
}

impl VersionOrigins {
    pub fn new(view: &sane::VerView) -> Self {
        VersionOrigins {
            version: Version::new(view),
            origins: view
                .origin_iter()
                .map(|o| {
                    Origin::from_ver_file(o)
                        .expect("a version's origin should always have a backing file")
                })
                .collect(),
        }
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
