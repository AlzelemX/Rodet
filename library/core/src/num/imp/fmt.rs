//! Shared utilities used by both float and integer formatting.
//!
//! <div lang="ar" dir="rtl">
//!
//! أدوات مشتركة لتنسيق الأرقام العشرية والصحيحة.
//!
//! </div>
#![doc(hidden)]
#![unstable(
    feature = "numfmt",
    reason = "internal routines only exposed for testing",
    issue = "none"
)]

/// Formatted parts.
///
/// <div lang="ar" dir="rtl">
///
/// الأجزاء المنسَّقة.
///
/// </div>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Part<'a> {
    /// Given number of zero digits.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// عدد محدد من الأصفار.
    ///
    /// </div>
    Zero(usize),
    /// A literal number up to 5 digits.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// رقم حرفي حتى 5 أرقام.
    ///
    /// </div>
    Num(u16),
    /// A verbatim copy of given bytes.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نسخة حرفية من البايتات المعطاة.
    ///
    /// </div>
    Copy(&'a [u8]),
}

impl<'a> Part<'a> {
    /// Returns the exact byte length of given part.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع طول الجزء بالبايتات بالضبط.
    ///
    /// </div>
    pub fn len(&self) -> usize {
        match *self {
            Part::Zero(nzeroes) => nzeroes,
            Part::Num(v) => v.checked_ilog10().unwrap_or_default() as usize + 1,
            Part::Copy(buf) => buf.len(),
        }
    }

    /// Writes a part into the supplied buffer.
    /// Returns the number of written bytes, or `None` if the buffer is not enough.
    /// (It may still leave partially written bytes in the buffer; do not rely on that.)
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يكتب جزءًا في المخزن المعطى، ويرجع عدد البايتات المكتوبة، أو `None` إذا لم يكفِ المخزن.
    ///
    /// </div>
    pub fn write(&self, out: &mut [u8]) -> Option<usize> {
        let len = self.len();
        if out.len() >= len {
            match *self {
                Part::Zero(nzeroes) => {
                    for c in &mut out[..nzeroes] {
                        *c = b'0';
                    }
                }
                Part::Num(mut v) => {
                    for c in out[..len].iter_mut().rev() {
                        *c = b'0' + (v % 10) as u8;
                        v /= 10;
                    }
                }
                Part::Copy(buf) => {
                    out[..buf.len()].copy_from_slice(buf);
                }
            }
            Some(len)
        } else {
            None
        }
    }
}

/// Formatted result containing one or more parts.
/// This can be written to the byte buffer or converted to the allocated string.
///
/// <div lang="ar" dir="rtl">
///
/// نتيجة منسَّقة فيها جزء أو أكثر. يمكن كتابتها في مخزن بايتات أو تحويلها إلى نص.
///
/// </div>
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Formatted<'a> {
    /// A byte slice representing a sign, either `""`, `"-"` or `"+"`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// شريحة بايتات تمثل الإشارة: `""` أو `"-"` أو `"+"`.
    ///
    /// </div>
    pub sign: &'static str,
    /// Formatted parts to be rendered after a sign and optional zero padding.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الأجزاء المنسَّقة التي تُعرض بعد الإشارة والحشو بالأصفار.
    ///
    /// </div>
    pub parts: &'a [Part<'a>],
}

impl<'a> Formatted<'a> {
    /// Returns the byte length of combined formatted result.
    ///
    /// Saturates at `usize::MAX` if the actual length is larger.
    ///
    /// This matters on 16-bit targets, where exponential formatting can exceed
    /// `usize::MAX` by emitting `u16::MAX` trailing zeroes plus `"1."` / `"e0"`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع طول النتيجة المنسَّقة كاملة بالبايتات.
    ///
    /// </div>
    pub fn len(&self) -> usize {
        self.parts.iter().fold(self.sign.len(), |len, part| len.saturating_add(part.len()))
    }

    /// Writes all formatted parts into the supplied buffer.
    /// Returns the number of written bytes, or `None` if the buffer is not enough.
    /// (It may still leave partially written bytes in the buffer; do not rely on that.)
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يكتب كل الأجزاء المنسَّقة في المخزن المعطى، ويرجع عدد البايتات المكتوبة، أو `None` إذا لم
    /// يكفِ المخزن.
    ///
    /// </div>
    pub fn write(&self, out: &mut [u8]) -> Option<usize> {
        out.get_mut(..self.sign.len())?.copy_from_slice(self.sign.as_bytes());

        let mut written = self.sign.len();
        for part in self.parts {
            let len = part.write(&mut out[written..])?;
            written += len;
        }
        Some(written)
    }
}
