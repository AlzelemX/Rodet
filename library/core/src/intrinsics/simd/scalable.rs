//! Scalable vector compiler intrinsics.
//!
//! In this module, a "vector" is any `#[rustc_scalable_vector]`-annotated type.
//!
//! <div lang="ar" dir="rtl">
//!
//! دوال المترجم الداخلية للمتجهات القابلة للتحجيم.
//!
//! </div>

/// Create a tuple of two vectors.
///
/// `SVecTup` must be a scalable vector tuple (`#[rustc_scalable_vector]`) and `SVec` must be a
/// scalable vector (`#[rustc_scalable_vector(N)]`). `SVecTup` must be a tuple of vectors of
/// type `SVec`.
///
/// Corresponds to Clang's `__builtin_sve_svcreate2*` builtins.
///
/// <div lang="ar" dir="rtl">
///
/// ينشئ tuple من متجهين.
///
/// </div>
#[rustc_nounwind]
#[rustc_intrinsic]
pub unsafe fn sve_tuple_create2<SVec, SVecTup>(x0: SVec, x1: SVec) -> SVecTup;

/// Create a tuple of three vectors.
///
/// `SVecTup` must be a scalable vector tuple (`#[rustc_scalable_vector]`) and `SVec` must be a
/// scalable vector (`#[rustc_scalable_vector(N)]`). `SVecTup` must be a tuple of vectors of
/// type `SVec`.
///
/// Corresponds to Clang's `__builtin_sve_svcreate3*` builtins.
///
/// <div lang="ar" dir="rtl">
///
/// ينشئ tuple من ثلاثة متجهات.
///
/// </div>
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn sve_tuple_create3<SVec, SVecTup>(x0: SVec, x1: SVec, x2: SVec) -> SVecTup;

/// Create a tuple of four vectors.
///
/// `SVecTup` must be a scalable vector tuple (`#[rustc_scalable_vector]`) and `SVec` must be a
/// scalable vector (`#[rustc_scalable_vector(N)]`). `SVecTup` must be a tuple of vectors of
/// type `SVec`.
///
/// Corresponds to Clang's `__builtin_sve_svcreate4*` builtins.
///
/// <div lang="ar" dir="rtl">
///
/// ينشئ tuple من أربعة متجهات.
///
/// </div>
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn sve_tuple_create4<SVec, SVecTup>(x0: SVec, x1: SVec, x2: SVec, x3: SVec) -> SVecTup;

/// Get one vector from a tuple of vectors.
///
/// `SVecTup` must be a scalable vector tuple (`#[rustc_scalable_vector]`) and `SVec` must be a
/// scalable vector (`#[rustc_scalable_vector(N)]`). `SVecTup` must be a tuple of vectors of
/// type `SVec`.
///
/// Corresponds to Clang's `__builtin_sve_svget*` builtins.
///
/// # Safety
///
/// `IDX` must be in-bounds of the tuple.
///
/// <div lang="ar" dir="rtl">
///
/// يرجع متجهًا واحدًا من tuple متجهات.
///
/// </div>
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn sve_tuple_get<SVecTup, SVec, const IDX: i32>(tuple: SVecTup) -> SVec;

/// Change one vector in a tuple of vectors.
///
/// `SVecTup` must be a scalable vector tuple (`#[rustc_scalable_vector]`) and `SVec` must be a
/// scalable vector (`#[rustc_scalable_vector(N)]`). `SVecTup` must be a tuple of vectors of
/// type `SVec`.
///
/// Corresponds to Clang's `__builtin_sve_svset*` builtins.
///
/// # Safety
///
/// `IDX` must be in-bounds of the tuple.
///
/// <div lang="ar" dir="rtl">
///
/// يغيّر متجهًا واحدًا في tuple متجهات.
///
/// </div>
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn sve_tuple_set<SVecTup, SVec, const IDX: i32>(tuple: SVecTup, x: SVec) -> SVecTup;
