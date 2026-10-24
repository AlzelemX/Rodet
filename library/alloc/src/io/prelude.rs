//! The I/O Prelude.
//!
//! The purpose of this module is to alleviate imports of many common I/O traits
//! by adding a glob import to the top of I/O heavy modules:
//!
//! ```
//! # #![feature(alloc_io)]
//! # #![allow(unused_imports)]
//! use alloc::io::prelude::*;
//! ```
//!
//! <div lang="ar" dir="rtl">
//!
//! مقدمة الإدخال والإخراج (I/O Prelude): ما يُستورد عادةً للعمل مع I/O.
//!
//! </div>

#[stable(feature = "rust1", since = "1.0.0")]
pub use crate::io::{BufRead, Read, Seek, Write};
