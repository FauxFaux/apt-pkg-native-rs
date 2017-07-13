#include <sstream>

#include <assert.h>

#include <apt-pkg/pkgcache.h>
#include <apt-pkg/prettyprinters.h>
#include <apt-pkg/cachefile.h>

struct PCache {
    pkgCacheFile *cache_file;
    pkgCache *cache;
};

extern "C" {
    PCache *get_pkg_cache();
    void free_pkg_cache(PCache *cache);

    int iterate_packages(PCache *cache, int (*visit)(pkgCache::PkgIterator *iterator));
    const char *pkg_iter_name(pkgCache::PkgIterator *iterator);

    // freed by caller
    char *pkg_iter_pretty(PCache *cache, pkgCache::PkgIterator *iterator);
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
    // TODO: is cache->cache cleaned up with cache->cache_file?
    delete cache->cache_file;
    delete cache;
}

int iterate_packages(PCache *cache, int (*visit)(pkgCache::PkgIterator*)) {
    for (pkgCache::PkgIterator iter = cache->cache->PkgBegin(); iter != cache->cache->PkgEnd(); ++iter) {
        if (!visit(&iter)) {
            return false;
        }
    }

    return true;
}

const char *pkg_iter_name(pkgCache::PkgIterator *iterator) {
    return iterator->Name();
}

char *pkg_iter_pretty(PCache *cache, pkgCache::PkgIterator *iterator) {
    assert(cache);
    assert(iterator);
    std::stringstream ss;
    ss << APT::PrettyPkg(cache->cache_file->GetDepCache(), *iterator);
    return strdup(ss.str().c_str());
}

//const void *pkg_iter_name(pkgCache::PkgIterator *iterator) {
//    return iterator->VersionList();
//}
