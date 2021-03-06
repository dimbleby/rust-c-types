//! A crate that re-exports various types defined in both `libc` and `winapi`.
//!
//! Stop having to write code like this:
//!
//! ```ignore
//! #[cfg(unix)]
//! use libc::some_type;
//!
//! #[cfg(windows)]
//! use winapi::some_type;
//! ```
//!
//! Instead, write code like this:
//!
//! ```ignore
//! use c_types::some_type;
//! ```
#![allow(non_camel_case_types)]

#[cfg(unix)]
mod unix {
    extern crate libc;

    pub type fd_set = libc::fd_set;
    pub type hostent = libc::hostent;
    pub type in_addr = libc::in_addr;
    pub type in6_addr = libc::in6_addr;
    pub type iovec = libc::iovec;
    pub type sa_family_t = libc::sa_family_t;
    pub type sockaddr = libc::sockaddr;
    pub type sockaddr_in = libc::sockaddr_in;
    pub type sockaddr_in6 = libc::sockaddr_in6;
    pub type socklen_t = libc::socklen_t;

    pub const AF_UNSPEC: i32 = libc::AF_UNSPEC;
    pub const AF_INET: i32 = libc::AF_INET;
    pub const AF_INET6: i32 = libc::AF_INET6;
}

#[cfg(windows)]
mod windows {
    extern crate libc;
    extern crate winapi;

    pub type fd_set = winapi::um::winsock2::fd_set;
    pub type hostent = winapi::um::winsock2::hostent;
    pub type in_addr = winapi::shared::inaddr::in_addr;
    pub type in6_addr = winapi::shared::in6addr::in6_addr;
    #[repr(C)]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: libc::size_t,
    }
    pub type sa_family_t = winapi::shared::ws2def::ADDRESS_FAMILY;
    pub type sockaddr = winapi::shared::ws2def::SOCKADDR;
    pub type sockaddr_in = winapi::shared::ws2def::SOCKADDR_IN;
    pub type sockaddr_in6 = winapi::shared::ws2ipdef::SOCKADDR_IN6_LH;
    pub type socklen_t = winapi::um::ws2tcpip::socklen_t;

    pub const AF_UNSPEC: i32 = winapi::shared::ws2def::AF_UNSPEC;
    pub const AF_INET: i32 = winapi::shared::ws2def::AF_INET;
    pub const AF_INET6: i32 = winapi::shared::ws2def::AF_INET6;
}

#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
pub use self::windows::*;
