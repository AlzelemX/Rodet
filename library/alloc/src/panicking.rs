#![doc(hidden)]
#![unstable(feature = "std_internals", issue = "none")]

use core::any::Any;
use core::fmt::Display;

use crate::boxed::Box;

/// An internal trait used by std to pass data from std to `panic_unwind` and
/// other panic runtimes. Not intended to be stabilized any time soon, do not
/// use.
///
/// <div lang="ar" dir="rtl">
///
/// trait داخلي تستخدمه std لتمرير بيانات الـ panic إلى `panic_unwind` وغيرها؛ ليس للاستخدام العام.
///
/// </div>
pub trait PanicPayload: Display {
    /// Take full ownership of the contents.
    ///
    /// After this method got called, only some dummy default value is left in `self`.
    /// Calling this method twice, or calling `get` after calling this method, is an error.
    ///
    /// The argument is borrowed because the panic runtime (`__rust_start_panic`) only
    /// gets a borrowed `dyn PanicPayload`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يأخذ ملكية المحتوى كاملًا.
    ///
    /// </div>
    fn take_box(&mut self) -> Box<dyn Any + Send>;

    /// Just borrow the contents.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يستعير المحتوى فقط.
    ///
    /// </div>
    fn get(&mut self) -> &(dyn Any + Send);

    /// Tries to borrow the contents as `&str`, if possible without doing any allocations.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحاول استعارة المحتوى كـ `&str` بدون أي حجز إن أمكن.
    ///
    /// </div>
    fn as_str(&mut self) -> Option<&str> {
        None
    }
}
