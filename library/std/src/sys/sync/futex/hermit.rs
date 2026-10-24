use crate::os::hermit::hermit_abi;
use crate::ptr::null;
use crate::sync::atomic::Atomic;
use crate::time::Duration;

/// An atomic for use as a futex that is at least 32-bits but may be larger
///
/// <div lang="ar" dir="rtl">
///
/// قيمة ذرية تُستخدم كـ futex بحجم 32 بت على الأقل.
///
/// </div>
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex
///
/// <div lang="ar" dir="rtl">
///
/// يجب أن يكون النوع الأساسي لـ `Futex`.
///
/// </div>
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
///
/// <div lang="ar" dir="rtl">
///
/// قيمة ذرية تُستخدم كـ futex بحجم 8 بت على الأقل.
///
/// </div>
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex
///
/// <div lang="ar" dir="rtl">
///
/// يجب أن يكون النوع الأساسي لـ `SmallFutex`.
///
/// </div>
pub type SmallPrimitive = u32;

pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    // Calculate the timeout as a relative timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout.and_then(|dur| {
        Some(hermit_abi::timespec {
            tv_sec: dur.as_secs().try_into().ok()?,
            tv_nsec: dur.subsec_nanos().try_into().ok()?,
        })
    });

    let r = unsafe {
        hermit_abi::futex_wait(
            futex.as_ptr(),
            expected,
            timespec.as_ref().map_or(null(), |t| t as *const hermit_abi::timespec),
            hermit_abi::FUTEX_RELATIVE_TIMEOUT,
        )
    };

    r != -hermit_abi::errno::ETIMEDOUT
}

#[inline]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    unsafe { hermit_abi::futex_wake(futex.as_ptr(), 1) > 0 }
}

#[inline]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    unsafe {
        hermit_abi::futex_wake(futex.as_ptr(), i32::MAX);
    }
}
