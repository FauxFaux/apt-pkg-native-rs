use sane;

#[derive(Clone, Debug)]
pub struct BinaryPackage {
    name: String,
    arch: String,
}

impl BinaryPackage {
    pub fn from_iter(iter: &sane::PkgIterator) -> Self {
        BinaryPackage {
            name: iter.name(),
            arch: iter.arch(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    version: String,
    arch: String,
    section: String,
    source_package: String,
    source_version: String,
}


impl Version {
    pub fn from_iter(iter: &sane::VerIterator) -> Self {
        Version {
            version: iter.version(),
            arch: iter.arch(),
            section: iter.section(),
            source_package: iter.source_package(),
            source_version: iter.source_version(),
        }
    }
}