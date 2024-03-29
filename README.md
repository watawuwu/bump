# bump

bump is a simple tool to increment [semver](https://semver.org/)-like versions.

![test](https://github.com/watawuwu/bump/workflows/Test/badge.svg?branch=master)
[![codecov](https://codecov.io/gh/watawuwu/bump/branch/master/graph/badge.svg)](https://codecov.io/gh/watawuwu/bump)
[![Latest version](https://img.shields.io/crates/v/bump-bin.svg)](https://crates.io/crates/bump-bin)
[![Documentation](https://docs.rs/bump-bin/badge.svg)](https://docs.rs/crate/bump-bin)
[![Docker](https://img.shields.io/docker/pulls/watawuwu/bump)](https://hub.docker.com/repository/docker/watawuwu/bump/)
![License](https://img.shields.io/crates/l/bump-bin.svg)

## Getting Started

- Bump patch version

```
$ bump patch 1.0.0
1.0.1
```

- Bump minor version

```
$ bump minor 1.0.0
1.1.0
```

- Bump major version

```
$ bump major 1.0.0
2.0.0
```

- Replace pre release version

```
$ bump pre beta.0 1.0.0-alpha.0
1.0.0-beta.0
```

- Replace build release version

```
$ bump build 20190720CCDD 1.0.0+20190720AABB
1.0.0+20190720CCDD
```

- If semver has the version prefix char, bump ignore the prefix.

```
$ bump patch v1.0.0
v1.0.1

$ bump patch release-1.0.1
release-1.0.1
```

- Can specify file

```
$ echo 1.0.0 > version.txt
$ bump patch -f version.txt
1.0.1
```

- Can read from pipeline

```
$ echo 1.0.0 | bump patch
1.0.1
```

- Other usage

```
Increments version with semver specification

Usage: bump <COMMAND>

Commands:
  patch
          Increment patch version
  minor
          Increment minor version
  major
          Increment major version
  pre
          Replace pre-release version
  build
          Replace build metadata
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help
  -V, --version
          Print version
```

### Installing


- Install with cargo

```
$ cargo install bump-bin
```

- Downloads assets from Github release

https://github.com/watawuwu/bump/releases/latest

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Authors

* [Wataru Matsui](watawuwu@3bi.tech)
