//! This module and the contained sub-modules contains the code for efficient and robust sort
//! implementations, as well as the domain adjacent implementation of `select_nth_unstable`.
//!
//! <div lang="ar" dir="rtl">
//!
//! هذه الوحدة فيها تطبيقات ترتيب فعالة وقوية، و`select_nth_unstable`.
//!
//! </div>

pub mod stable;
pub mod unstable;

pub(crate) mod select;
pub(crate) mod shared;
