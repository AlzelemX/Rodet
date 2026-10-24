//! Compatibility module for C platform-specific types. Use [`core::ffi`] instead.
//!
//! <div lang="ar" dir="rtl">
//!
//! وحدة توافق لأنواع C الخاصة بالمنصة؛ استخدم `core::ffi` بدلًا منها.
//!
//! </div>

#![stable(feature = "raw_os", since = "1.1.0")]

#[cfg(test)]
mod tests;

macro_rules! alias_core_ffi {
    ($($t:ident)*) => {$(
        #[stable(feature = "raw_os", since = "1.1.0")]
        #[doc = include_str!(concat!("../../../../core/src/ffi/", stringify!($t), ".md"))]
        #[doc(cfg(all()))]
        pub type $t = core::ffi::$t;
    )*}
}

alias_core_ffi! {
    c_char c_schar c_uchar
    c_short c_ushort
    c_int c_uint
    c_long c_ulong
    c_longlong c_ulonglong
    c_float
    c_double
    c_void
}
