//! A crate that re-exports various types defined in both `libc` and `winapi`.
//!
//! Stop having to write code like this:
//!
//! ```
//! #[cfg(unix)]
//! use libc::some_type;
//!
//! #[cfg(windows)]
//! use winapi::some_type;
//! ```
//!
//! Instead, write code like this:
//!
//! ```
//! use c_types::some_type;
//! ```
mod ctypes;
pub use ctypes::*;
