//! Decodes a floating-point value into individual parts and error ranges.
//!
//! <div lang="ar" dir="rtl">
//!
//! يفك رقمًا عشريًا إلى أجزائه ومديات الخطأ.
//!
//! </div>

use crate::num::FpCategory;
use crate::num::imp::FloatExt;

/// Decoded unsigned finite value, such that:
///
/// - The original value equals to `mant * 2^exp`.
///
/// - Any number from `(mant - minus) * 2^exp` to `(mant + plus) * 2^exp` will
///   round to the original value. The range is inclusive only when
///   `inclusive` is `true`.
///
/// <div lang="ar" dir="rtl">
///
/// قيمة منتهية بدون إشارة بعد فكها، بحيث:
///
/// </div>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Decoded {
    /// The scaled mantissa.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الجزء المعنوي المُحجَّم.
    ///
    /// </div>
    pub mant: u64,
    /// The lower error range.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مدى الخطأ الأدنى.
    ///
    /// </div>
    pub minus: u64,
    /// The upper error range.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مدى الخطأ الأعلى.
    ///
    /// </div>
    pub plus: u64,
    /// The shared exponent in base 2.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الأس المشترك بالأساس 2.
    ///
    /// </div>
    pub exp: i16,
    /// True when the error range is inclusive.
    ///
    /// In IEEE 754, this is true when the original mantissa was even.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// صحيح عندما يكون مدى الخطأ شاملًا.
    ///
    /// </div>
    pub inclusive: bool,
}

/// Decoded unsigned value.
///
/// <div lang="ar" dir="rtl">
///
/// قيمة بدون إشارة بعد فكها.
///
/// </div>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FullDecoded {
    /// Not-a-number.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// ليس رقمًا (NaN).
    ///
    /// </div>
    Nan,
    /// Infinities, either positive or negative.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// اللانهاية، موجبة أو سالبة.
    ///
    /// </div>
    Infinite,
    /// Zero, either positive or negative.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// صفر، موجب أو سالب.
    ///
    /// </div>
    Zero,
    /// Finite numbers with further decoded fields.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أرقام منتهية مع حقول مفكوكة إضافية.
    ///
    /// </div>
    Finite(Decoded),
}

/// A floating point type which can be `decode`d.
///
/// <div lang="ar" dir="rtl">
///
/// نوع عشري يمكن فكه بـ `decode`.
///
/// </div>
pub trait DecodableFloat: FloatExt + Copy {
    /// The minimum positive normalized value.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أصغر قيمة موجبة موحَّدة.
    ///
    /// </div>
    fn min_pos_norm_value() -> Self;
}

#[cfg(target_has_reliable_f16)]
impl DecodableFloat for f16 {
    fn min_pos_norm_value() -> Self {
        f16::MIN_POSITIVE
    }
}

impl DecodableFloat for f32 {
    fn min_pos_norm_value() -> Self {
        f32::MIN_POSITIVE
    }
}

impl DecodableFloat for f64 {
    fn min_pos_norm_value() -> Self {
        f64::MIN_POSITIVE
    }
}

/// Returns a sign (true when negative) and `FullDecoded` value
/// from given floating point number.
///
/// <div lang="ar" dir="rtl">
///
/// يرجع الإشارة (true إذا كان سالبًا) وقيمة `FullDecoded` من الرقم العشري المعطى.
///
/// </div>
pub fn decode<T: DecodableFloat>(v: T) -> (/*negative?*/ bool, FullDecoded) {
    let (mant, exp, sign) = v.integer_decode();
    let even = (mant & 1) == 0;
    let decoded = match v.classify() {
        FpCategory::Nan => FullDecoded::Nan,
        FpCategory::Infinite => FullDecoded::Infinite,
        FpCategory::Zero => FullDecoded::Zero,
        FpCategory::Subnormal => {
            // neighbors: (mant - 2, exp) -- (mant, exp) -- (mant + 2, exp)
            // Float::integer_decode always preserves the exponent,
            // so the mantissa is scaled for subnormals.
            FullDecoded::Finite(Decoded { mant, minus: 1, plus: 1, exp, inclusive: even })
        }
        FpCategory::Normal => {
            let minnorm = <T as DecodableFloat>::min_pos_norm_value().integer_decode();
            if mant == minnorm.0 {
                // neighbors: (maxmant, exp - 1) -- (minnormmant, exp) -- (minnormmant + 1, exp)
                // where maxmant = minnormmant * 2 - 1
                FullDecoded::Finite(Decoded {
                    mant: mant << 2,
                    minus: 1,
                    plus: 2,
                    exp: exp - 2,
                    inclusive: even,
                })
            } else {
                // neighbors: (mant - 1, exp) -- (mant, exp) -- (mant + 1, exp)
                FullDecoded::Finite(Decoded {
                    mant: mant << 1,
                    minus: 1,
                    plus: 1,
                    exp: exp - 1,
                    inclusive: even,
                })
            }
        }
    };
    (sign < 0, decoded)
}
