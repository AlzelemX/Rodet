//! WASI-specific networking functionality
//!
//! <div lang="ar" dir="rtl">
//!
//! وظائف شبكات خاصة بـ WASI.
//!
//! </div>

#![unstable(feature = "wasi_ext", issue = "71213")]

use crate::os::fd::AsRawFd;
use crate::sys::err2io;
use crate::{io, net};

/// WASI-specific extensions to [`std::net::TcpListener`].
///
/// [`std::net::TcpListener`]: crate::net::TcpListener
///
/// <div lang="ar" dir="rtl">
///
/// امتدادات خاصة بـ WASI لـ `std::net::TcpListener`.
///
/// </div>
pub trait TcpListenerExt {
    /// Accept a socket.
    ///
    /// This corresponds to the `sock_accept` syscall.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يقبل مقبسًا.
    ///
    /// </div>
    fn sock_accept(&self, flags: u16) -> io::Result<u32>;
}

impl TcpListenerExt for net::TcpListener {
    fn sock_accept(&self, flags: u16) -> io::Result<u32> {
        unsafe { wasip1::sock_accept(self.as_raw_fd() as wasip1::Fd, flags).map_err(err2io) }
    }
}
