//! Helpers module for exporting the `view_types` macro.
//!
//! <div lang="ar" dir="rtl">
//!
//! وحدة مساعدة لتصدير الماكرو `view_types`.
//!
//! </div>

/// Creates a view type.
/// ```
/// #![feature(view_types, view_type_macro)]
//
/// struct Foo {
///     bar: usize,
///     baz: u32,
/// }
///
/// type FooBar = std::view::view_type!(Foo.{ bar });
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// ينشئ نوع عرض (view type).
///
/// </div>
#[macro_export]
#[rustc_builtin_macro(view_type)]
#[unstable(feature = "view_type_macro", issue = "155938")]
macro_rules! view_type {
    ($($arg:tt)*) => {
        /* compiler built-in */
    };
}
