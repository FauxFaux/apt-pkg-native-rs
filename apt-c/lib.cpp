#include <apt-pkg/pkgcache.h>
#include <apt-pkg/cachefile.h>

struct PCache {
    pkgCacheFile *cache_file;
    pkgCache *cache;
};

extern "C" {
    PCache *get_pkg_cache();
    void free_pkg_cache(PCache *cache);

    int iterate_all_packages(PCache *cache, int (*visit)(const char*));
}

PCache *get_pkg_cache() {
    pkgInitConfig(*_config);
    pkgInitSystem(*_config, _system);

    pkgCacheFile *cache_file = new pkgCacheFile();
    pkgCache *cache = cache_file->GetPkgCache();

    PCache *ret = new PCache();
    ret->cache_file = cache_file;
    ret->cache = cache;

    return ret;
}

void free_pkg_cache(PCache *cache) {
    delete cache->cache_file;
    delete cache;
}

int iterate_all_packages(PCache *cache, int (*visit)(const char*)) {
    for (pkgCache::PkgIterator iter = cache->cache->PkgBegin(); iter != cache->cache->PkgEnd(); ++iter) {
        if (!visit(iter.Name())) {
            return false;
        }
    }

    return true;
}
