use std::collections::HashMap;
use std::collections::HashSet;
/// A port of a randomly selected Python program:
///
/// ```python
/// #!/usr/bin/python3
/// import collections
/// import apt
/// cache = apt.cache.Cache()
/// def versions_in(suite):
///     source_versions = collections.defaultdict(set)
///
///     for package in cache:
///         for version in package.versions:
///             if suite and suite not in (origin.archive for origin in version.origins):
///                 continue
///             source_versions[version.source_name].add(version.source_version)
///     return source_versions
///
/// if '__main__' == __name__:
///     import sys
///     sources = versions_in(sys.argv[1] if len(sys.argv) > 1 else None)
///     for src in sorted(sources.keys()):
///         # sort lexographically for determinism, not for any other reason
///         for ver in sorted(sources[src]):
///             print('{}={}'.format(src, ver))
/// ```
use std::env;

use apt_pkg_native::Cache;

#[cfg(feature = "ye-olde-apt")]
fn main() {
    eprintln!("ye-olde-apt pre-dates source versions")
}

#[cfg(not(feature = "ye-olde-apt"))]
fn main() {
    let archive_filter = env::args().nth(1);

    let mut cache = Cache::get_singleton();
    let mut source_versions = HashMap::new();
    {
        let mut all_packages = cache.iter();

        while let Some(binary) = all_packages.next() {
            let mut binary_versions = binary.versions();
            while let Some(version) = binary_versions.next() {
                if let Some(ref target_archive) = archive_filter {
                    if version
                        .origin_iter()
                        .map(|origin| origin.file().next().unwrap().archive())
                        .any(|archive| archive == *target_archive)
                    {
                        continue;
                    }
                }

                source_versions
                    .entry(version.source_package())
                    .or_insert_with(HashSet::new)
                    .insert(version.source_version());
            }
        }
    }

    for src in lexicographic_sort(source_versions.keys()) {
        let mut sorted_versions: Vec<&String> = source_versions[src].iter().collect();
        sorted_versions.sort_by(|left, right| cache.compare_versions(left, right));
        for ver in sorted_versions {
            println!("{}={}", src, ver);
        }
    }
}

fn lexicographic_sort<I, T>(input: I) -> Vec<T>
where
    T: Ord + Clone,
    I: Iterator<Item = T>,
{
    let mut val: Vec<T> = input.collect();
    val.sort();
    val
}
