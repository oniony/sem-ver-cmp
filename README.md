SEM.VER.CMP

# Overview

SEM.VER.CMP compares [semantic version](https://semver.org/) numbers.

Handy for comparing versions in continuous integration (CI) scripts, &c.

# Features

* [X] Compare two semantic version numbers withoun suffixes
* [ ] Support for label suffixes
* [ ] Validate a single semantic version number
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

## Compare Two Versions

```sh
semvercmp LEFT RIGHT 
```

LEFT and RIGHT are semantic versions with or without `v` prefix.

Prints text to standard output to indicate the relationship between LEFT and RIGHT:

* `less`: LEFT is less than RIGHT
* `equal`: LEFT and RIGHT are equal
* `greater`: LEFT is greater than RIGHT

Returns an error code if either version is invalid.

### Examples

```
$ semvercmp v1.2.3 v1.2.5
less
$ semvercmp 4.5.10 4.5.1
greater
```

# About

SemVerCmp is written and maintained by Paul Ruane (<paul.ruane@oniony.com>) and
is available at <http://github.com/oniony/semvercmp/>.

Written in Rust: <http://rust-lang.org/>

- - -

© Copyright 2025 Paul Ruane

Copying and distribution of this file, with or without modification, are
permitted in any medium without royalty provided the copyright notice and this
notice are preserved.  This file is offered as-is, without any warranty.
