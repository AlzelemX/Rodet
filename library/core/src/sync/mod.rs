//! Synchronization primitives
//!
//! <div lang="ar" dir="rtl">
//!
//! أدوات التزامن بين الخيوط.
//!
//! </div>

#![stable(feature = "rust1", since = "1.0.0")]

pub mod atomic;
mod sync_view;
#[stable(feature = "exclusive_wrapper", since = "CURRENT_RUSTC_VERSION")]
pub use sync_view::SyncView;
