#[cfg(target_arch = "wasm32")]
use core::arch::wasm32 as wasm;
#[cfg(target_arch = "wasm64")]
use core::arch::wasm64 as wasm;

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

/// Wait for a futex_wake operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
///
/// <div lang="ar" dir="rtl">
///
/// ينتظر عملية `futex_wake` لتوقظنا.
///
/// </div>
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    let timeout = timeout.and_then(|t| t.as_nanos().try_into().ok()).unwrap_or(-1);
    unsafe {
        wasm::memory_atomic_wait32(
            futex as *const Atomic<u32> as *mut i32,
            expected as i32,
            timeout,
        ) < 2
    }
}

/// Wakes up one thread that's blocked on `futex_wait` on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// <div lang="ar" dir="rtl">
///
/// يوقظ خيطًا واحدًا ينتظر في `futex_wait` على هذا الـ futex.
///
/// </div>
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    unsafe { wasm::memory_atomic_notify(futex as *const Atomic<u32> as *mut i32, 1) > 0 }
}

/// Wakes up all threads that are waiting on `futex_wait` on this futex.
///
/// <div lang="ar" dir="rtl">
///
/// يوقظ كل الخيوط التي تنتظر في `futex_wait` على هذا الـ futex.
///
/// </div>
pub fn futex_wake_all(futex: &Atomic<u32>) {
    unsafe {
        wasm::memory_atomic_notify(futex as *const Atomic<u32> as *mut i32, i32::MAX as u32);
    }
}
