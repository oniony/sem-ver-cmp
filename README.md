SEM·VER·CMP

# Overview

SEM·VER·CMP compares [semantic version](https://semver.org/) numbers.

Useful for comparing versions in continuous integration (CI) scripts, &c.

# Features

* [X] Compare two semantic version numbers without suffixes
* [X] Validate a single semantic version number
* [ ] Support for pre-release suffixes (e.g. `1.2.3-pre.1.blah.2`)
* [ ] Support for build suffixes (e.g. `1.2.3-pre+build123`)
* [ ] Ranges
    - [ ] Partial versions, e.g. v1.2 contains v1.2.3
    - [ ] Explicit ranges, e.g. v1.2.0..v1.2.5 contains v1.2.3
* [ ] Other operations
    - [ ] Sort set of versions
    - [ ] Get maximum/minimum of set of versions

# Compilation

* Install Rust from <https://www.rust-lang.org/>
* Build SEM.VER.CMP:

    $ git clone git@github.com:oniony/semvercmp.git
    $ cd semvercmp
    $ cargo build
    
# Usage

## Verify a Single Version

```sh
semvercmp VERSION
```

VERSION is a semantic version with or without a `v` prefix.

A zero exit status code indicates a valid version number.

## Compare Two Versions

```sh
semvercmp ONE TWO 
```

ONE and TWO are semantic versions with or without `v` prefix.

A zero exit status code indicates equal versions.
An exit status code of 1 indicates version ONE is greater; a status code of 2 indicates TWO is greater.
An exit status code of 255 indicates an invalid status code was provided.

### Examples

```
$ semvercmp v1.2.3
$ semver cmp v1.2.3
$ semvercmp v1.2.3 v1.2.5
$ semvercmp 4.5.10 4.5.1
```

# About

SEM·VER·CMP is written and maintained by Paul Ruane (<paul.ruane@oniony.com>) and
is available at <http://github.com/oniony/semvercmp/>.

Written in Rust: <http://rust-lang.org/>

- - -

© Copyright 2025 Paul Ruane

Copying and distribution of this file, with or without modification, are
permitted in any medium without royalty provided the copyright notice and this
notice are preserved.  This file is offered as-is, without any warranty.
