use sane;

pub struct BinaryPackage {
    name: String,
    arch: String,
}

impl BinaryPackage {
    pub fn from_iter(iter: &sane::PkgIterator) -> BinaryPackage {
        BinaryPackage {
            name: iter.name(),
            arch: iter.arch(),
        }
    }
}
