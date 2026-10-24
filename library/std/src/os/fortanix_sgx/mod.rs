//! Functionality specific to the `x86_64-fortanix-unknown-sgx` target.
//!
//! This includes functions to deal with memory isolation, usercalls, and the
//! SGX instruction set.
//!
//! <div lang="ar" dir="rtl">
//!
//! وظائف خاصة بالهدف `x86_64-fortanix-unknown-sgx`.
//!
//! </div>

#![deny(missing_docs)]
#![unstable(feature = "sgx_platform", issue = "56975")]

/// Low-level interfaces to usercalls. See the [ABI documentation] for more
/// information.
///
/// [ABI documentation]: https://docs.rs/fortanix-sgx-abi/
///
/// <div lang="ar" dir="rtl">
///
/// واجهات منخفضة المستوى لاستدعاءات المستخدم (usercalls)؛ راجع توثيق الـ ABI.
///
/// </div>
pub mod usercalls {
    pub use crate::sys::abi::usercalls::*;

    /// Primitives for allocating memory in userspace as well as copying data
    /// to and from user memory.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أساسيات لحجز الذاكرة في مساحة المستخدم ونسخ البيانات من وإلى ذاكرة المستخدم.
    ///
    /// </div>
    pub mod alloc {
        pub use crate::sys::abi::usercalls::alloc::*;
    }

    /// Lowest-level interfaces to usercalls and usercall ABI type definitions.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أدنى واجهات لاستدعاءات المستخدم وتعريفات أنواع ABI الخاصة بها.
    ///
    /// </div>
    pub mod raw {
        pub use crate::sys::abi::usercalls::raw::{
            ByteBuffer, Cancel, EV_RETURNQ_NOT_EMPTY, EV_UNPARK, EV_USERCALLQ_NOT_FULL, Error,
            FD_STDERR, FD_STDIN, FD_STDOUT, Fd, FifoDescriptor, RESULT_SUCCESS, Register,
            RegisterArgument, Result, Return, ReturnValue, Tcs, USERCALL_USER_DEFINED, Usercall,
            Usercalls as UsercallNrs, WAIT_INDEFINITE, WAIT_NO, accept_stream, alloc, async_queues,
            bind_stream, close, connect_stream, do_usercall, exit, flush, free, insecure_time,
            launch_thread, read, read_alloc, send, wait, write,
        };
    }
}

/// Functions for querying mapping information for pointers.
///
/// <div lang="ar" dir="rtl">
///
/// دوال للاستعلام عن معلومات تعيين المؤشرات.
///
/// </div>
pub mod mem {
    pub use crate::sys::abi::mem::*;
}

pub mod arch;
pub mod ffi;
pub mod io;

/// Functions for querying thread-related information.
///
/// <div lang="ar" dir="rtl">
///
/// دوال للاستعلام عن معلومات الخيوط.
///
/// </div>
pub mod thread {
    pub use crate::sys::abi::thread::current;
}
