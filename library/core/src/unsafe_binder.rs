//! Operators used to turn types into unsafe binders and back.
//!
//! <div lang="ar" dir="rtl">
//!
//! عوامل لتحويل الأنواع إلى روابط غير آمنة (unsafe binders) والعكس.
//!
//! </div>

/// Unwrap an unsafe binder into its underlying type.
///
/// <div lang="ar" dir="rtl">
///
/// يفك رابطًا غير آمن إلى نوعه الأساسي.
///
/// </div>
#[allow_internal_unstable(builtin_syntax)]
#[unstable(feature = "unsafe_binders", issue = "130516")]
#[diagnostic::opaque]
pub macro unwrap_binder {
    ($expr:expr) => {
        builtin # unwrap_binder ( $expr )
    },
    ($expr:expr ; $ty:ty) => {
        builtin # unwrap_binder ( $expr, $ty )
    },
}

/// Wrap a type into an unsafe binder.
///
/// <div lang="ar" dir="rtl">
///
/// يغلّف نوعًا في رابط غير آمن.
///
/// </div>
#[allow_internal_unstable(builtin_syntax)]
#[unstable(feature = "unsafe_binders", issue = "130516")]
#[diagnostic::opaque]
pub macro wrap_binder {
    ($expr:expr) => {
        builtin # wrap_binder ( $expr )
    },
    ($expr:expr ; $ty:ty) => {
        builtin # wrap_binder ( $expr, $ty )
    },
}
