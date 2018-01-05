# rust-c-types #

Re-exports types defined in both `libc` and `winapi`, to reduce the amount of conditional compilation required in your code.  Stop having to write code like this:

```rust
#[cfg(unix)]
use libc::some_type;

#[cfg(windows)]
use winapi::some_type;
```

Instead, write code like this:
```rust
use c_types::some_type;
```

[![Build Status](https://travis-ci.org/dimbleby/rust-c-types.svg?branch=master)](https://travis-ci.org/dimbleby/rust-c-types)
[![Build status](https://ci.appveyor.com/api/projects/status/lg8k62ahyks2u681/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/rust-c-types/branch/master)
[![crates.io](https://meritbadge.herokuapp.com/c-types)](https://crates.io/crates/c-types)

## Documentation ##

API documentation is [here](https://docs.rs/c-types).

## Contributing ##

This crate is non-exhaustive - I add types to it as and when I need them for my own projects.

If there's a type that could be included but isn't, there should be no problem adding it.  Please open an issue or, even better, send a pull request.
