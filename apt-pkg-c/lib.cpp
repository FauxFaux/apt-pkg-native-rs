#include <sstream>
#include <cstdint>

#include <assert.h>

#include <apt-pkg/cachefile.h>
#include <apt-pkg/pkgcache.h>
#include <apt-pkg/version.h>

#include <apt-pkg/init.h>
#include <apt-pkg/pkgsystem.h>
#include <apt-pkg/policy.h>

struct PCache {
    // Owned by us.
    pkgCacheFile *cache_file;

    // Borrowed from cache_file.
    pkgCache *cache;

    // Owned by us.
    pkgRecords *records;
};

struct PPkgIterator {
    // Owned by us.
    pkgCache::PkgIterator iterator;

    // Borrow of "static" PCache.
    PCache *cache;
};

struct PVerIterator {
    // Owned by us.
    pkgCache::VerIterator iterator;

    // Borrowed from PCache.
    pkgCache::PkgIterator *pkg;

    // Borrow of "static" PCache.
    PCache *cache;
};

struct PDepIterator {
    // Owned by us.
    pkgCache::DepIterator iterator;

    // Borrowed from PCache.
    pkgCache::VerIterator *ver;

    // Borrow of "static" PCache.
    PCache *cache;
};

struct PVerFileIterator {
    // Owned by us.
    pkgCache::VerFileIterator iterator;

    // Borrow of "static" PCache.
    PCache *cache;
};

struct PPkgFileIterator {
    // Owned by us.
    pkgCache::PkgFileIterator iterator;
};

struct PVerFileParser {
    pkgRecords::Parser *parser;
};

extern "C" {
    void init_config_system();

    PCache *pkg_cache_create();
    void pkg_cache_release(PCache *cache);

    int32_t pkg_cache_compare_versions(PCache *cache, const char *left, const char *right);

    // pkg_iter creation and deletion
    PPkgIterator *pkg_cache_pkg_iter(PCache *cache);
    PPkgIterator *pkg_cache_find_name(PCache *cache, const char *name);
    PPkgIterator *pkg_cache_find_name_arch(PCache *cache, const char *name, const char *arch);
    void pkg_iter_release(PPkgIterator *iterator);

    // pkg_iter mutation
    void pkg_iter_next(PPkgIterator *iterator);
    bool pkg_iter_end(PPkgIterator *iterator);

    // pkg_iter access
    const char *pkg_iter_name(PPkgIterator *iterator);
    const char *pkg_iter_arch(PPkgIterator *iterator);
    const char *pkg_iter_current_version(PPkgIterator *iterator);
    const char *pkg_iter_candidate_version(PPkgIterator *iterator);


    // ver_iter creation and deletion
    PVerIterator *pkg_iter_ver_iter(PPkgIterator *iterator);
    void ver_iter_release(PVerIterator *iterator);

    // ver_iter mutation
    void ver_iter_next(PVerIterator *iterator);
    bool ver_iter_end(PVerIterator *iterator);

    // ver_iter access
    const char *ver_iter_version(PVerIterator *iterator);
    const char *ver_iter_section(PVerIterator *iterator);
    const char *ver_iter_arch(PVerIterator *iterator);
    const char *ver_iter_priority_type(PVerIterator *wrapper);

#ifndef YE_OLDE_APT
    const char *ver_iter_source_package(PVerIterator *iterator);
    const char *ver_iter_source_version(PVerIterator *iterator);
    int32_t ver_iter_priority(PVerIterator *iterator);
#endif

    // dep_iter creation and deletion
    PDepIterator *ver_iter_dep_iter(PVerIterator *iterator);
    void dep_iter_release(PDepIterator *iterator);

    // dep_iter mutation
    void dep_iter_next(PDepIterator *iterator);
    bool dep_iter_end(PDepIterator *iterator);

    // dep_iter access
    PPkgIterator *dep_iter_target_pkg(PDepIterator *iterator);
    const char *dep_iter_target_ver(PDepIterator *iterator);
    const char *dep_iter_comp_type(PDepIterator *iterator);
    const char *dep_iter_dep_type(PDepIterator *iterator);

    // ver_file_iter creation and deletion
    PVerFileIterator *ver_iter_ver_file_iter(PVerIterator *iterator);
    void ver_file_iter_release(PVerFileIterator *iterator);

    // ver_file_iter mutation
    void ver_file_iter_next(PVerFileIterator *iterator);
    bool ver_file_iter_end(PVerFileIterator *iterator);

    // ver_file_parser creation
    PVerFileParser *ver_file_iter_get_parser(PVerFileIterator *iterator);

    // ver_file_parser access
    const char *ver_file_parser_short_desc(PVerFileParser *parser);
    const char *ver_file_parser_long_desc(PVerFileParser *parser);
    const char *ver_file_parser_maintainer(PVerFileParser *parser);
    const char *ver_file_parser_homepage(PVerFileParser *parser);

    // ver_file_iter has no accessors, only the creation of pkg_file_iter


    // pkg_file_iter creation
    PPkgFileIterator *ver_file_iter_pkg_file_iter(PVerFileIterator *iterator);
    void pkg_file_iter_release(PPkgFileIterator *iterator);

    // pkg_file_iter mutation
    void pkg_file_iter_next(PPkgFileIterator *iterator);
    bool pkg_file_iter_end(PPkgFileIterator *iterator);

    // pkg_file_iter access
    const char *pkg_file_iter_file_name(PPkgFileIterator *iterator);
    const char *pkg_file_iter_archive(PPkgFileIterator *iterator);
    const char *pkg_file_iter_version(PPkgFileIterator *iterator);
    const char *pkg_file_iter_origin(PPkgFileIterator *iterator);
    const char *pkg_file_iter_codename(PPkgFileIterator *iterator);
    const char *pkg_file_iter_label(PPkgFileIterator *iterator);
    const char *pkg_file_iter_site(PPkgFileIterator *iterator);
    const char *pkg_file_iter_component(PPkgFileIterator *iterator);
    const char *pkg_file_iter_architecture(PPkgFileIterator *iterator);
    const char *pkg_file_iter_index_type(PPkgFileIterator *iterator);
}

void init_config_system() {
    pkgInitConfig(*_config);
    pkgInitSystem(*_config, _system);
}

PCache *pkg_cache_create() {
    pkgCacheFile *cache_file = new pkgCacheFile();
    pkgCache *cache = cache_file->GetPkgCache();
    pkgRecords *records = new pkgRecords(*cache);

    PCache *ret = new PCache();
    ret->cache_file = cache_file;
    ret->cache = cache;
    ret->records = records;

    return ret;
}

void pkg_cache_release(PCache *cache) {
    // TODO: is cache->cache cleaned up with cache->cache_file?
    delete cache->records;
    delete cache->cache_file;
    delete cache;
}

int32_t pkg_cache_compare_versions(PCache *cache, const char *left, const char *right) {
    // an int is returned here; presumably it will always be -1, 0 or 1.
    return cache->cache->VS->DoCmpVersion(left, left+strlen(left), right, right+strlen(right));
}

PPkgIterator *pkg_cache_pkg_iter(PCache *cache) {
    PPkgIterator *wrapper = new PPkgIterator();
    wrapper->iterator = cache->cache->PkgBegin();
    wrapper->cache = cache;
    return wrapper;
}

PPkgIterator *pkg_cache_find_name(PCache *cache, const char *name) {
    PPkgIterator *wrapper = new PPkgIterator();
    wrapper->iterator = cache->cache->FindPkg(name);
    wrapper->cache = cache;
    return wrapper;
}

PPkgIterator *pkg_cache_find_name_arch(PCache *cache, const char *name, const char *arch) {
    PPkgIterator *wrapper = new PPkgIterator();
    wrapper->iterator = cache->cache->FindPkg(name, arch);
    wrapper->cache = cache;
    return wrapper;
}

void pkg_iter_release(PPkgIterator *wrapper) {
    delete wrapper;
}

void pkg_iter_next(PPkgIterator *wrapper) {
    ++wrapper->iterator;
}

bool pkg_iter_end(PPkgIterator *wrapper) {
    return wrapper->cache->cache->PkgEnd() == wrapper->iterator;
}

const char *pkg_iter_name(PPkgIterator *wrapper) {
    return wrapper->iterator.Name();
}

const char *pkg_iter_arch(PPkgIterator *wrapper) {
    return wrapper->iterator.Arch();
}

const char *pkg_iter_current_version(PPkgIterator *wrapper) {
    return wrapper->iterator.CurVersion();
}

const char *pkg_iter_candidate_version(PPkgIterator *wrapper) {
    pkgCache::VerIterator it = wrapper->cache->cache_file->GetPolicy()->GetCandidateVer(wrapper->iterator);
    if (it.end()) {
        return nullptr;
    }
    return it.VerStr();
}

PVerIterator *pkg_iter_ver_iter(PPkgIterator *wrapper) {
    PVerIterator *new_wrapper = new PVerIterator();
    new_wrapper->iterator = wrapper->iterator.VersionList();
    new_wrapper->pkg = &wrapper->iterator;
    new_wrapper->cache = wrapper->cache;
    return new_wrapper;
}

void ver_iter_release(PVerIterator *wrapper) {
    delete wrapper;
}

void ver_iter_next(PVerIterator *wrapper) {
    ++wrapper->iterator;
}

bool ver_iter_end(PVerIterator *wrapper) {
    return wrapper->iterator.end();
}


const char *ver_iter_version(PVerIterator *wrapper) {
    return wrapper->iterator.VerStr();
}

const char *ver_iter_section(PVerIterator *wrapper) {
   return wrapper->iterator.Section();
}

const char *ver_iter_priority_type(PVerIterator *wrapper) {
    return wrapper->iterator.PriorityType();
}

#ifndef YE_OLDE_APT

const char *ver_iter_source_package(PVerIterator *wrapper) {
    return wrapper->iterator.SourcePkgName();
}

const char *ver_iter_source_version(PVerIterator *wrapper) {
    return wrapper->iterator.SourceVerStr();
}

int32_t ver_iter_priority(PVerIterator *wrapper) {
    // The priority is a "short", which is roughly a (signed) int16_t;
    // going bigger just in case
    return wrapper->cache->cache_file->GetPolicy()->GetPriority(wrapper->iterator);
}

#endif

const char *ver_iter_arch(PVerIterator *wrapper) {
    return wrapper->iterator.Arch();
}


PDepIterator *ver_iter_dep_iter(PVerIterator *wrapper) {
    PDepIterator *new_wrapper = new PDepIterator();
    new_wrapper->iterator = wrapper->iterator.DependsList();
    new_wrapper->cache = wrapper->cache;
    return new_wrapper;
}

void dep_iter_release(PDepIterator *wrapper) {
    delete wrapper;
}

void dep_iter_next(PDepIterator *wrapper) {
    ++wrapper->iterator;
}

bool dep_iter_end(PDepIterator *wrapper) {
    return wrapper->iterator.end();
}

PPkgIterator *dep_iter_target_pkg(PDepIterator *wrapper) {
    PPkgIterator *new_wrapper = new PPkgIterator();
    new_wrapper->iterator = wrapper->iterator.TargetPkg();
    new_wrapper->cache = wrapper->cache;
    return new_wrapper;
}

const char *dep_iter_target_ver(PDepIterator *wrapper) {
    return wrapper->iterator.TargetVer();
}

const char *dep_iter_comp_type(PDepIterator *wrapper) {
    return wrapper->iterator.CompType();
}

const char *dep_iter_dep_type(PDepIterator *wrapper) {
    return wrapper->iterator.DepType();
}


PVerFileIterator *ver_iter_ver_file_iter(PVerIterator *wrapper) {
    PVerFileIterator *new_wrapper = new PVerFileIterator();
    new_wrapper->iterator = wrapper->iterator.FileList();
    new_wrapper->cache = wrapper->cache;
    return new_wrapper;
}

void ver_file_iter_release(PVerFileIterator *wrapper) {
    delete wrapper;
}

void ver_file_iter_next(PVerFileIterator *wrapper) {
    ++wrapper->iterator;
}

PVerFileParser *ver_file_iter_get_parser(PVerFileIterator *wrapper) {
    PVerFileParser *parser = new PVerFileParser();
    parser->parser = &wrapper->cache->records->Lookup(wrapper->iterator);
    return parser;
}

const char *to_c_string(std::string s) {
    char *cstr = new char[s.length()+1];
    std::strcpy(cstr, s.c_str());
    return cstr;
}

const char *ver_file_parser_short_desc(PVerFileParser *parser) {
    std::string desc = parser->parser->ShortDesc();
    return to_c_string(desc);
}

const char *ver_file_parser_long_desc(PVerFileParser *parser) {
    std::string desc = parser->parser->LongDesc();
    return to_c_string(desc);
}

const char *ver_file_parser_maintainer(PVerFileParser *parser) {
    std::string maint = parser->parser->Maintainer();
    return to_c_string(maint);
}

const char *ver_file_parser_homepage(PVerFileParser *parser) {
    std::string hp = parser->parser->Homepage();
    return to_c_string(hp);
}

bool ver_file_iter_end(PVerFileIterator *wrapper) {
    return wrapper->iterator.end();
}

PPkgFileIterator *ver_file_iter_pkg_file_iter(PVerFileIterator *wrapper) {
    PPkgFileIterator *new_wrapper = new PPkgFileIterator();
    new_wrapper->iterator = wrapper->iterator.File();
    return new_wrapper;
}

void pkg_file_iter_release(PPkgFileIterator *wrapper) {
    delete wrapper;
}

void pkg_file_iter_next(PPkgFileIterator *wrapper) {
    ++wrapper->iterator;
}

bool pkg_file_iter_end(PPkgFileIterator *wrapper) {
    return wrapper->iterator.end();
}

const char *pkg_file_iter_file_name(PPkgFileIterator *wrapper) {
    return wrapper->iterator.FileName();
}

const char *pkg_file_iter_archive(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Archive();
}

const char *pkg_file_iter_version(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Version();
}

const char *pkg_file_iter_origin(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Origin();
}

const char *pkg_file_iter_codename(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Codename();
}

const char *pkg_file_iter_label(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Label();
}

const char *pkg_file_iter_site(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Site();
}

const char *pkg_file_iter_component(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Component();
}

const char *pkg_file_iter_architecture(PPkgFileIterator *wrapper) {
    return wrapper->iterator.Architecture();
}

const char *pkg_file_iter_index_type(PPkgFileIterator *wrapper) {
    return wrapper->iterator.IndexType();
}
