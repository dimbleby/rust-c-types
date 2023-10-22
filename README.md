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

[![Crates.io][crates-badge]][crates-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/c-types.svg
[crates-url]: https://crates.io/crates/c-types
[actions-badge]: https://github.com/dimbleby/rust-c-types/actions/workflows/build.yml/badge.svg
[actions-url]: https://github.com/dimbleby/rust-c-types/actions?query=workflow%3ACI+branch%3Amain

## Documentation ##

API documentation is [here](https://docs.rs/c-types).

## Contributing ##

This crate is non-exhaustive - I add types to it as and when I need them for my own projects.

If there's a type that could be included but isn't, there should be no problem adding it.  Please open an issue or, even better, send a pull request.
