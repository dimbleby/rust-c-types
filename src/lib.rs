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

    pub type ADDRESS_FAMILY = libc::c_int;
    pub const AF_UNSPEC: ADDRESS_FAMILY = libc::AF_UNSPEC;
    pub const AF_INET: ADDRESS_FAMILY = libc::AF_INET;
    pub const AF_INET6: ADDRESS_FAMILY = libc::AF_INET6;
}

#[cfg(windows)]
mod windows {
    extern crate libc;
    extern crate windows_sys;

    pub type fd_set = windows_sys::Win32::Networking::WinSock::FD_SET;
    pub type hostent = windows_sys::Win32::Networking::WinSock::HOSTENT;
    pub type in_addr = windows_sys::Win32::Networking::WinSock::IN_ADDR;
    pub type in6_addr = windows_sys::Win32::Networking::WinSock::IN6_ADDR;
    #[repr(C)]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: libc::size_t,
    }
    pub type sa_family_t = windows_sys::Win32::Networking::WinSock::sa_family_t;
    pub type sockaddr = windows_sys::Win32::Networking::WinSock::SOCKADDR;
    pub type sockaddr_in = windows_sys::Win32::Networking::WinSock::SOCKADDR_IN;
    pub type sockaddr_in6 = windows_sys::Win32::Networking::WinSock::SOCKADDR_IN6;
    pub type socklen_t = windows_sys::Win32::Networking::WinSock::socklen_t;

    pub type ADDRESS_FAMILY = windows_sys::Win32::Networking::WinSock::ADDRESS_FAMILY;
    pub const AF_UNSPEC: ADDRESS_FAMILY = windows_sys::Win32::Networking::WinSock::AF_UNSPEC;
    pub const AF_INET: ADDRESS_FAMILY = windows_sys::Win32::Networking::WinSock::AF_INET;
    pub const AF_INET6: ADDRESS_FAMILY = windows_sys::Win32::Networking::WinSock::AF_INET6;
}

#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
pub use self::windows::*;
