#include <sstream>

#include <assert.h>

#include <apt-pkg/pkgcache.h>
#include <apt-pkg/prettyprinters.h>
#include <apt-pkg/cachefile.h>

struct PCache {
    // Owned by us.
    pkgCacheFile *cache_file;

    // Borrowed from cache_file.
    pkgCache *cache;
};

struct PPkgIterator {
    // Owned by us.
    pkgCache::PkgIterator iterator;

    // Borrowed from PCache.
    pkgCache *cache;
};

extern "C" {
    PCache *pkg_cache_create();
    void pkg_cache_release(PCache *cache);

    PPkgIterator *pkg_cache_pkg_iter(PCache *cache);
    void pkg_iter_release(PPkgIterator *iterator);

    bool pkg_iter_next(PPkgIterator *iterator);

    const char *pkg_iter_name(PPkgIterator *iterator);

    // freed by caller
    char *pkg_iter_pretty(PCache *cache, PPkgIterator *iterator);
}

PCache *pkg_cache_create() {
    pkgInitConfig(*_config);
    pkgInitSystem(*_config, _system);

    pkgCacheFile *cache_file = new pkgCacheFile();
    pkgCache *cache = cache_file->GetPkgCache();

    PCache *ret = new PCache();
    ret->cache_file = cache_file;
    ret->cache = cache;

    return ret;
}

void pkg_cache_release(PCache *cache) {
    // TODO: is cache->cache cleaned up with cache->cache_file?
    delete cache->cache_file;
    delete cache;
}

PPkgIterator *pkg_cache_pkg_iter(PCache *cache) {
    PPkgIterator *wrapper = new PPkgIterator();
    wrapper->iterator = cache->cache->PkgBegin();
    wrapper->cache = cache->cache;
    return wrapper;
}

void pkg_iter_release(PPkgIterator *wrapper) {
    delete wrapper;
}

bool pkg_iter_next(PPkgIterator *wrapper) {
    ++wrapper->iterator;
    return wrapper->cache->PkgEnd() != wrapper->iterator;
}

const char *pkg_iter_name(PPkgIterator *wrapper) {
    return wrapper->iterator.Name();
}

char *pkg_iter_pretty(PCache *cache, PPkgIterator *wrapper) {
    assert(cache);
    assert(wrapper);
    std::stringstream ss;
    ss << APT::PrettyPkg(cache->cache_file->GetDepCache(), wrapper->iterator);
    return strdup(ss.str().c_str());
}

//const void *pkg_iter_name(pkgCache::PkgIterator *iterator) {
//    return iterator->VersionList();
//}
