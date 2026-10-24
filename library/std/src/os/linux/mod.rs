//! Linux-specific definitions.
//!
//! <div lang="ar" dir="rtl">
//!
//! تعريفات خاصة بـ Linux.
//!
//! </div>

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "linux"))]

pub mod fs;
pub mod net;
pub mod process;
pub mod raw;
