//! L4Re-specific definitions.
//!
//! <div lang="ar" dir="rtl">
//!
//! تعريفات خاصة بـ L4Re.
//!
//! </div>

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "l4re"))]

pub mod fs;
pub mod raw;
