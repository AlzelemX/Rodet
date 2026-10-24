//! Error types for conversion to integral types.
//!
//! <div lang="ar" dir="rtl">
//!
//! أنواع أخطاء التحويل إلى الأعداد الصحيحة.
//!
//! </div>

use crate::error::Error;
use crate::fmt;

/// The error type returned when a checked integral type conversion fails.
///
/// <div lang="ar" dir="rtl">
///
/// نوع الخطأ الذي يرجع عند فشل تحويل عدد صحيح مع التحقق.
///
/// </div>
#[stable(feature = "try_from", since = "1.34.0")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntError(pub(crate) IntErrorKind);

impl TryFromIntError {
    /// Outputs the detailed cause of converting an integer failing.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يعطي السبب التفصيلي لفشل تحويل عدد صحيح.
    ///
    /// </div>
    #[must_use]
    #[unstable(feature = "try_from_int_error_kind", issue = "153978")]
    pub const fn kind(&self) -> &IntErrorKind {
        &self.0
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            IntErrorKind::Empty | IntErrorKind::InvalidDigit => unreachable!(),
            IntErrorKind::PosOverflow => "number too large to fit in target type",
            IntErrorKind::NegOverflow => "number too small to fit in target type",
            IntErrorKind::Zero => "number would be zero for non-zero type",
            IntErrorKind::NotAPowerOfTwo => "number is not a power of two",
        }
        .fmt(f)
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl Error for TryFromIntError {}

#[stable(feature = "never_type", since = "1.100.0")]
#[rustc_const_unstable(feature = "const_convert", issue = "143773")]
const impl From<!> for TryFromIntError {
    #[inline]
    fn from(never: !) -> TryFromIntError {
        never
    }
}

/// An error which can be returned when parsing an integer.
///
/// For example, this error is returned by the `from_str_radix()` functions
/// on the primitive integer types (such as [`i8::from_str_radix`])
/// and is used as the error type in their [`FromStr`] implementations.
///
/// [`FromStr`]: crate::str::FromStr
///
/// # Potential causes
///
/// Among other causes, `ParseIntError` can be thrown because of leading or trailing whitespace
/// in the string e.g., when it is obtained from the standard input.
/// Using the [`str::trim()`] method ensures that no whitespace remains before parsing.
///
/// # Example
///
/// ```
/// if let Err(e) = i32::from_str_radix("a12", 10) {
///     println!("Failed conversion to i32: {e}");
/// }
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// خطأ قد يرجع عند قراءة عدد صحيح من نص.
///
/// </div>
#[derive(Debug, Clone, PartialEq, Eq)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct ParseIntError {
    pub(super) kind: IntErrorKind,
}

/// Enum to store the various types of errors that can cause parsing or converting an
/// integer to fail.
///
/// # Example
///
/// ```
/// # fn main() {
/// if let Err(e) = i32::from_str_radix("a12", 10) {
///     println!("Failed conversion to i32: {:?}", e.kind());
/// }
/// # }
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// enum يخزن أنواع الأخطاء المختلفة التي قد تسبب فشل قراءة أو تحويل عدد صحيح.
///
/// </div>
#[stable(feature = "int_error_matching", since = "1.55.0")]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[non_exhaustive]
pub enum IntErrorKind {
    /// Value being parsed is empty.
    ///
    /// This variant will be constructed when parsing an empty string.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// القيمة المقروءة فارغة.
    ///
    /// </div>
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    Empty,
    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// تحتوي رقمًا غير صالح في سياقها.
    ///
    /// </div>
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// العدد كبير جدًا على النوع المستهدف.
    ///
    /// </div>
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    PosOverflow,
    /// Integer is too small to store in target integer type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// العدد صغير جدًا على النوع المستهدف.
    ///
    /// </div>
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    NegOverflow,
    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string or the converting integer
    /// has a value of zero, which would be illegal for non-zero types.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// القيمة صفر.
    ///
    /// </div>
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    Zero,
    /// Value is not a power of two.
    ///
    /// This variant will be emitted when converting an integer that is not a power of
    /// two. This is required in some cases such as constructing an [`Alignment`].
    ///
    /// [`Alignment`]: core::mem::Alignment "mem::Alignment"
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// القيمة ليست قوة للعدد 2.
    ///
    /// </div>
    #[unstable(feature = "try_from_int_error_kind", issue = "153978")]
    // Also, #[unstable(feature = "ptr_alignment_type", issue = "102070")]
    NotAPowerOfTwo,
}

impl ParseIntError {
    /// Outputs the detailed cause of parsing an integer failing.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يعطي السبب التفصيلي لفشل قراءة عدد صحيح.
    ///
    /// </div>
    #[must_use]
    #[rustc_const_stable(feature = "const_int_from_str", since = "1.82.0")]
    #[stable(feature = "int_error_matching", since = "1.55.0")]
    pub const fn kind(&self) -> &IntErrorKind {
        &self.kind
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            IntErrorKind::Empty => "cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::PosOverflow => "number too large to fit in target type",
            IntErrorKind::NegOverflow => "number too small to fit in target type",
            IntErrorKind::Zero => "number would be zero for non-zero type",
            IntErrorKind::NotAPowerOfTwo => "number is not a power of two",
        }
        .fmt(f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for ParseIntError {}
