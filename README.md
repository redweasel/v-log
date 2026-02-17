v-log
===

A Rust library providing a lightweight visual logging/debugging *facade*.

[![Build status](https://img.shields.io/github/actions/workflow/status/rust-lang/v-log/main.yml?branch=master)](https://github.com/rust-lang/v-log/actions)
[![Latest version](https://img.shields.io/crates/v/v-log.svg)](https://crates.io/crates/v-log)
[![Documentation](https://docs.rs/v-log/badge.svg)](https://docs.rs/v-log)
![License](https://img.shields.io/crates/l/v-log.svg)

* [`v-log` documentation](https://docs.rs/v-log)

A visual logging facade provides a single visual logging API that abstracts over the actual
visual logging implementation. Libraries can use the visual logging API provided by this
crate, and the consumer of those libraries can choose the visual logging
implementation that is most suitable for its use case.


## Minimum supported `rustc`

`1.68.0+`

This version is explicitly tested in CI and may be bumped in any release as needed. Maintaining compatibility with older compilers is a priority though, so the bar for bumping the minimum supported version is set very high. Any changes to the supported minimum version will be called out in the release notes.

## Usage

### In libraries

Libraries should link only to the `v-log` crate, and use the provided macros to
log whatever geometric information will be useful to downstream consumers:

```toml
[dependencies]
v-log = "0.1"
```

```rust
use v_log::{line, label};

pub fn shave_the_yak(yak: &mut Yak) {
    label!("yak_surface", [10.0, 10.0], (12.0, Warn, "<"), "Commencing yak shaving!");

    for (&pos1, &pos2) in &yak.lines {
        polyline!("yak_surface", (pos1, pos2), 2.0, Base);
    }
}
```

### In executables

In order to produce log output, executables have to use a vlogger implementation compatible with the facade.
There is no vlogger implemented yet.

Executables should choose a vlogger implementation and initialize it early in the
runtime of the program. Vlogger implementations will typically include a
function to do this. Any v-log messages generated before the vlogger is
initialized will be ignored.

The executable itself may use the `v-log` crate to log geometry as well.
