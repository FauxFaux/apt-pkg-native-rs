This crate provides bindings to `libapt-pkg`.


### Documentation and Examples

See the `examples/` folder for some partial implementations of some commands.

https://docs.rs/apt-pkg-native

### License Note

While the code in this crate is available under a permissive MIT license,
  it is useless without [`libapt-pkg`](https://tracker.debian.org/pkg/apt),
  which is GPL2+.

### Building

`libapt-pkg-dev` must be installed. The [`cc`](https://crates.io/crates/cc)
  crate is used to try and find a native compiler.

The `ye-olde-apt` feature provides support for `apt <1.2` (Ubuntu 14.04 (Trusty),
Debian 7 (Jessie) (2015)). This works by just deleting methods which are not
available in that version. See
[#2](https://github.com/FauxFaux/apt-pkg-native-rs/issues/2#issuecomment-351180818).
This feature is no longer tested on CI, as these distros are long gone.


### Thread safety

It is intended that the crate should be usable from multiple threads.
However, this is generally implemented using singletons, which may be really
annoying for your use-case.

The current way the singleton is managed is awful, and it's not been fixed
while I've been learning about the problems. A major version bump, and a
proper singleton, may resolve some of the issues. This needs to be done eventually.

`apt` does not have a concurrency model: you may not use threads.

Since `apt` 1.4 or 1.5 (in Debian Stretch (2017), but not in Xenial 16.04),
some operations are thread safe: it should be possible to initialise the cache
twice in parallel. As these versions of `apt` are not widespread, the API of
this crate does not attempt to expose this.


### Alternatives

There is a similar project at [rust-apt](https://crates.io/crates/rust-apt).


### Switching distro with `docker`

`examples/on-sid` has a docker file which builds a minimum Debian image with
typical package lists downloaded. You can run a tool in it, from this directory,
by:

```
(cd examples/on-sid && make)
docker run -v $(pwd):/mnt sid-sources-list /mnt/target/release/examples/sources
```
