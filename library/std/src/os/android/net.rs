//! Android-specific networking functionality.
//!
//! <div lang="ar" dir="rtl">
//!
//! وظائف شبكات خاصة بـ Android.
//!
//! </div>

#![stable(feature = "unix_socket_abstract", since = "1.70.0")]

#[stable(feature = "unix_socket_abstract", since = "1.70.0")]
pub use crate::os::net::linux_ext::addr::SocketAddrExt;
#[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
pub use crate::os::net::linux_ext::socket::UnixSocketExt;
#[stable(feature = "tcp_quickack", since = "1.89.0")]
pub use crate::os::net::linux_ext::tcp::TcpStreamExt;
