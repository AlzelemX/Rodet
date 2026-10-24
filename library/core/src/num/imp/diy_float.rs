//! Extended precision "soft float", for internal use only.
//!
//! <div lang="ar" dir="rtl">
//!
//! رقم عشري برمجي بدقة ممتدة، للاستخدام الداخلي فقط.
//!
//! </div>

// This module is only for dec2flt and flt2dec, and only public because of coretests.
// It is not intended to ever be stabilized.
#![doc(hidden)]
#![unstable(
    feature = "core_private_diy_float",
    reason = "internal routines only exposed for testing",
    issue = "none"
)]

/// A custom 64-bit floating point type, representing `f * 2^e`.
#[derive(Copy, Clone, Debug)]
#[doc(hidden)]
pub struct Fp {
    /// The integer mantissa.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الجزء المعنوي كعدد صحيح.
    ///
    /// </div>
    pub f: u64,
    /// The exponent in base 2.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الأس بالأساس 2.
    ///
    /// </div>
    pub e: i16,
}

impl Fp {
    /// Returns a correctly rounded product of itself and `other`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع حاصل ضرب نفسه في `other` مقرَّبًا بشكل صحيح.
    ///
    /// </div>
    pub fn mul(self, other: Self) -> Self {
        let (lo, hi) = self.f.carrying_mul(other.f, 0);
        let f = hi + (lo >> 63) /* round */;
        let e = self.e + other.e + 64;
        Self { f, e }
    }

    /// Normalizes itself so that the resulting mantissa is at least `2^63`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يوحّد نفسه بحيث يكون الجزء المعنوي الناتج `2^63` على الأقل.
    ///
    /// </div>
    pub fn normalize(self) -> Self {
        let lz = self.f.leading_zeros();
        let f = self.f << lz;
        let e = self.e - lz as i16;
        debug_assert!(f >= (1 << 63));
        Self { f, e }
    }

    /// Normalizes itself to have the shared exponent.
    /// It can only decrease the exponent (and thus increase the mantissa).
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يوحّد نفسه ليكون له الأس المشترك. يمكنه فقط إنقاص الأس (وزيادة الجزء المعنوي).
    ///
    /// </div>
    pub fn normalize_to(self, e: i16) -> Self {
        let edelta = self.e - e;
        assert!(edelta >= 0);
        let edelta = edelta as usize;
        assert_eq!(self.f << edelta >> edelta, self.f);
        Self { f: self.f << edelta, e }
    }
}
