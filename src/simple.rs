use sane;

#[derive(Clone, Debug)]
pub struct BinaryPackage {
    pub name: String,
    pub arch: String,
}

impl BinaryPackage {
    pub fn new(iter: &sane::PkgIterator) -> Self {
        BinaryPackage {
            name: iter.name(),
            arch: iter.arch(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    pub version: String,
    pub arch: String,
    pub section: String,
    pub source_package: String,
    pub source_version: String,
    pub priority: i32,
}


impl Version {
    pub fn new(iter: &sane::VerIterator) -> Self {
        Version {
            version: iter.version(),
            arch: iter.arch(),
            section: iter.section(),
            source_package: iter.source_package(),
            source_version: iter.source_version(),
            priority: iter.priority(),
        }
    }
}
