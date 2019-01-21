use std::env;

use apt_pkg_native::simple;
use apt_pkg_native::Cache;

fn main() {
    let pkg = env::args()
        .nth(1)
        .expect("usage: first argument: package name");
    let arch = env::args().nth(2);

    let mut cache = Cache::get_singleton();
    let mut found = match arch {
        Some(arch) => cache.find_by_name_arch(pkg.as_str(), arch.as_str()),
        None => cache.find_by_name(pkg.as_str()),
    };

    if let Some(view) = found.next() {
        println!("{}:{}:", view.name(), view.arch());

        let installed_version = view
            .current_version()
            .unwrap_or_else(|| "(none)".to_string());
        println!("  Installed: {}", installed_version);
        println!(
            "  Candidate: {}",
            view.candidate_version()
                .unwrap_or_else(|| "(none)".to_string(),)
        );

        println!("  Version table:");
        for simple::VersionOrigins { version, origins } in
            view.versions().map(simple::VersionOrigins::new)
        {
            println!(
                " {} {} {}",
                if version.version == installed_version {
                    "***"
                } else {
                    "   "
                },
                version.version,
                version.priority,
            );

            for origin in origins {
                println!("       {:4} {}", "XXX", origin);
            }
        }
    } else {
        println!("unrecognised package: {}", pkg);
    }
}
