//! WASI-specific extensions to general I/O primitives.
//!
//! <div lang="ar" dir="rtl">
//!
//! امتدادات خاصة بـ WASI لأساسيات الإدخال والإخراج.
//!
//! </div>

#![stable(feature = "io_safety_wasi", since = "1.65.0")]

#[stable(feature = "io_safety_wasi", since = "1.65.0")]
pub use crate::os::fd::*;

// Tests for this module
#[cfg(test)]
mod tests;
