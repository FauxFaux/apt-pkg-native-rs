This crate provides bindings to `libapt-pkg`.

[![Build status](https://api.travis-ci.org/FauxFaux/apt-pkg-native-rs.png)](https://travis-ci.org/FauxFaux/apt-pkg-native-rs)
[![](http://meritbadge.herokuapp.com/apt-pkg-native)](https://crates.io/crates/apt-pkg-native)


### Documentation and Examples

See the `examples/` folder for some partial implementations of some commands.

https://docs.rs/apt-pkg-native

### License Note

While the code in this crate is available under a permissive MIT license,
  it is useless without [`libapt-pkg`](https://tracker.debian.org/pkg/apt),
  which is GPL2+.

### Building

`libapt-pkg-dev` must be installed. The [`gcc`](https://crates.io/crates/gcc)
  crate is used to try and find a native compiler.
