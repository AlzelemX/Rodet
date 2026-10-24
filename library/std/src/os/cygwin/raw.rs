//! Cygwin-specific raw type definitions.
//!
//! <div lang="ar" dir="rtl">
//!
//! تعريفات الأنواع الخام الخاصة بـ Cygwin.
//!
//! </div>

#[stable(feature = "raw_ext", since = "1.1.0")]
pub use libc::{blkcnt_t, blksize_t, dev_t, ino_t, mode_t, nlink_t, off_t, pthread_t, time_t};
