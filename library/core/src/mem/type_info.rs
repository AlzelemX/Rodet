//! MVP for exposing compile-time information about types in a
//! runtime or const-eval processable way.
//!
//! <div lang="ar" dir="rtl">
//!
//! نسخة أولية لإتاحة معلومات الأنواع وقت الترجمة.
//!
//! </div>

use crate::any::TypeId;
use crate::marker::PointeeSized;
use crate::ptr::DynMetadata;
use crate::{fmt, intrinsics};

/// Compile-time type information.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات النوع وقت الترجمة.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[lang = "type_info"]
#[unstable(feature = "type_info", issue = "146922")]
pub struct Type {
    /// Per-type information
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// معلومات خاصة بكل نوع.
    ///
    /// </div>
    pub kind: TypeKind,
}

/// Info of a trait implementation, you can retrieve the vtable with [Self::get_vtable]
///
/// <div lang="ar" dir="rtl">
///
/// معلومات تطبيق trait.
///
/// </div>
#[derive(Debug, PartialEq, Eq)]
#[unstable(feature = "type_info", issue = "146922")]
pub struct TraitImpl<T: PointeeSized> {
    pub(crate) vtable: DynMetadata<T>,
}

impl<T: PointeeSized> TraitImpl<T> {
    /// Gets the raw vtable for type reflection mapping
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع جدول الدوال الخام.
    ///
    /// </div>
    pub const fn get_vtable(&self) -> DynMetadata<T> {
        self.vtable
    }
}

impl TypeId {
    /// Compute the type information of a concrete type.
    /// It can only be called at compile time.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحسب معلومات نوع فعلي. يُستدعى وقت الترجمة فقط.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn info(self) -> Type {
        intrinsics::reflection::type_id_type_of(self)
    }
}

impl Type {
    /// Returns the type information of the generic type parameter.
    ///
    /// Note: Unlike `TypeId`s obtained via `TypeId::of`, the `Type`
    /// struct and its fields contain `TypeId`s that are not necessarily
    /// derived from types that outlive `'static`. This means that using
    /// the `TypeId`s (transitively) obtained from this function will
    /// be able to break invariants that other `TypeId` consuming crates
    /// may have assumed to hold.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع معلومات نوع المعامل العام.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    pub const fn of<T: ?Sized>() -> Self {
        const { intrinsics::reflection::type_id::<T>().info() }
    }
}

// FIXME(reflection): get rid of the static lifetime bound on TypeId and remove this function.
/// Returns the [TypeId] of the generic type parameter.
///
/// This is identical to [TypeId::of] but without the static lifetime bound. It will be removed
/// in the future.
///
/// <div lang="ar" dir="rtl">
///
/// يرجع `TypeId` لمعامل النوع العام.
///
/// </div>
#[must_use]
#[unstable(feature = "type_info", issue = "146922")]
#[rustc_const_unstable(feature = "type_info", issue = "146922")]
pub const fn of<T: ?Sized>() -> TypeId {
    const { intrinsics::reflection::type_id::<T>() }
}

/// Compile-time type information.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات النوع وقت الترجمة.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub enum TypeKind {
    /// Tuples.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// صفوف.
    ///
    /// </div>
    Tuple,
    /// Arrays.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مصفوفات.
    ///
    /// </div>
    Array,
    /// Slices.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// شرائح.
    ///
    /// </div>
    Slice,
    /// Dynamic Traits.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// traits ديناميكية.
    ///
    /// </div>
    DynTrait(DynTrait),
    /// Structs.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// structs.
    ///
    /// </div>
    Struct,
    /// Enums.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// enums.
    ///
    /// </div>
    Enum,
    /// Unions.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// unions.
    ///
    /// </div>
    Union,
    /// Primitive boolean type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// النوع المنطقي الأساسي.
    ///
    /// </div>
    Bool,
    /// Primitive character type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نوع الحرف الأساسي.
    ///
    /// </div>
    Char,
    /// Primitive signed and unsigned integer type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أنواع الأعداد الصحيحة بإشارة وبدون إشارة.
    ///
    /// </div>
    Int,
    /// Primitive floating-point type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نوع الأرقام العشرية الأساسي.
    ///
    /// </div>
    Float,
    /// String slice type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نوع شريحة النص.
    ///
    /// </div>
    Str(Str),
    /// References.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مراجع.
    ///
    /// </div>
    Reference,
    /// Pointers.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مؤشرات.
    ///
    /// </div>
    Pointer,
    /// Function pointers.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مؤشرات دوال.
    ///
    /// </div>
    FnPtr,
    /// FIXME(#146922): add all the common types
    /// non exhaustive list:
    /// - Never
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أنواع أخرى.
    ///
    /// </div>
    Other,
}

/// Compile-time type information about dynamic traits.
/// FIXME(#146922): Add super traits and generics
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن traits ديناميكية.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct DynTrait {
    /// The predicates of  a dynamic trait.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// شروط trait ديناميكي.
    ///
    /// </div>
    pub predicates: &'static [DynTraitPredicate],
}

/// Compile-time type information about a dynamic trait predicate.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن شرط trait ديناميكي.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct DynTraitPredicate {
    /// The type of the trait as a dynamic trait type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نوع الـ trait كنوع trait ديناميكي.
    ///
    /// </div>
    pub trait_ty: Trait,
}

/// Compile-time type information about a trait.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن trait.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct Trait {
    /// The TypeId of the trait as a dynamic type
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// `TypeId` للـ trait كنوع ديناميكي.
    ///
    /// </div>
    pub ty: TypeId,
    /// Whether the trait is an auto trait
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// هل الـ trait من نوع auto.
    ///
    /// </div>
    pub is_auto: bool,
}

/// Compile-time type information about instantiated generics of structs, enum and union variants.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن المعاملات العامة المحددة في structs وenums وunions.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
#[lang = "type_info_generic"]
pub enum Generic {
    /// Lifetimes.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أعمار.
    ///
    /// </div>
    Lifetime(Lifetime),
    /// Types.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أنواع.
    ///
    /// </div>
    Type(GenericType),
    /// Const parameters.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// معاملات ثابتة.
    ///
    /// </div>
    Const(Const),
}

/// Compile-time type information about generic lifetimes.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن الأعمار العامة.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct Lifetime {
    // No additional information to provide for now.
}

/// Compile-time type information about instantiated generic types.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن الأنواع العامة المحددة.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct GenericType {
    /// The type itself.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// النوع نفسه.
    ///
    /// </div>
    pub ty: TypeId,
}

/// Compile-time type information about generic const parameters.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن المعاملات الثابتة العامة.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct Const {
    /// The const's type.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// نوع الثابت.
    ///
    /// </div>
    pub ty: TypeId,
}

/// Compile-time type information about string slice types.
///
/// <div lang="ar" dir="rtl">
///
/// معلومات وقت الترجمة عن أنواع شرائح النص.
///
/// </div>
#[derive(Debug)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
pub struct Str {
    // No additional information to provide for now.
}

#[derive(Debug)]
#[lang = "FnPtr"]
#[unstable(feature = "type_info", issue = "146922")]
/// Function pointer, e.g. fn(u8),
///
/// <div lang="ar" dir="rtl">
///
/// مؤشر دالة، مثل `fn(u8)`.
///
/// </div>
pub struct FnPtr {
    is_unsafe: bool,
    abi: Abi,
    inputs: &'static [TypeId],
    output: TypeId,
    variadic: bool,
    // FIXME(splat): should these fields be private, or merged into an Option<u8/u16>?
    /// Is any function argument splatted?
    is_splatted: bool,

    splatted_index: u8,
}

impl FnPtr {
    /// Returns the splatted function argument index, or `None` if no argument is splatted.
    ///
    /// e.g. in `fn overload(a: u8, #[rustc_splat] b: (f32, usize))` the index is 1,
    /// and it can be called as `overload(a, 1.0, 2)`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع موضع وسيط الدالة المنشور، أو `None` إذا لم يوجد.
    ///
    /// </div>
    pub const fn splatted(&self) -> Option<u8> {
        if self.is_splatted { Some(self.splatted_index) } else { None }
    }
    /// Whether this function is variadic, e.g. extern "C" fn add(n: usize, mut args: ...);
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// هل هذه الدالة ذات وسائط متغيرة.
    ///
    /// </div>
    pub const fn is_variadic(&self) -> bool {
        self.variadic
    }
    /// whether this refers to an unsafe function.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// هل هذه دالة غير آمنة.
    ///
    /// </div>
    pub const fn is_unsafe(&self) -> bool {
        self.is_unsafe
    }
    /// Returns the application binary interface. For example extern "C".
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع واجهة الـ ABI، مثل extern "C".
    ///
    /// </div>
    pub const fn abi(&self) -> Abi {
        self.abi
    }
    /// The types of the functions parameters
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أنواع معاملات الدالة.
    ///
    /// </div>
    pub const fn inputs(&self) -> &'static [TypeId] {
        self.inputs
    }
    /// List of the types returned by the function. For a function with no output
    /// specified this returns `TypeId::of<()>`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// أنواع إرجاع الدالة. إذا لم يُحدد نوع إرجاع يرجع `TypeId::of<()>`.
    ///
    /// </div>
    pub const fn output(&self) -> TypeId {
        self.output
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[unstable(feature = "type_info", issue = "146922")]
/// Abi of [FnPtr]
///
/// <div lang="ar" dir="rtl">
///
/// ABI لمؤشر الدالة.
///
/// </div>
pub enum Abi {
    /// Named abi, e.g. extern "custom", "stdcall" etc.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// ABI مسمّى، مثل extern "custom" أو "stdcall".
    ///
    /// </div>
    Named(&'static str),

    /// Default
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// الافتراضي.
    ///
    /// </div>
    #[default]
    ExternRust,

    /// C-calling convention
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// اصطلاح استدعاء C.
    ///
    /// </div>
    ExternC,
}

impl TypeId {
    /// Returns `true` if the type represented by this `TypeId` is an signed integer.
    ///
    /// For everything else this returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<i32>().is_signed() }, true);
    /// assert_eq!(const { TypeId::of::<u8>().is_signed() }, false);
    /// assert_eq!(const { TypeId::of::<bool>().is_signed() }, false);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `true` إذا كان النوع عددًا صحيحًا بإشارة.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn is_signed(self) -> bool {
        intrinsics::reflection::type_id_is_signed(self)
    }

    /// When called on a `TypeId` representing an array or slice this returns the type of each
    /// element otherwise this returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<[u32; 16]>().element_ty() }, Some(TypeId::of::<u32>()));
    /// assert_eq!(const { TypeId::of::<u8>().element_ty() }, None); // not an array or slice
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مع مصفوفة أو شريحة يرجع نوع العنصر، وإلا يرجع `None`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn element_ty(self) -> Option<TypeId> {
        intrinsics::reflection::type_id_element_ty(self)
    }

    /// When called on a `TypeId` representing an array this returns the length of the array in
    /// all other cases this returns zero.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<[u32; 16]>().array_len() }, 16);
    /// assert_eq!(const { TypeId::of::<u8>().array_len() }, 0); // not an array
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مع مصفوفة يرجع طولها، وفي غير ذلك يرجع صفرًا.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn array_len(self) -> usize {
        intrinsics::reflection::type_id_array_len(self)
    }

    /// Returns the size of the type represented by this `TypeId`. `None` if it is unsized.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<u32>().size() }, Some(4));
    /// assert_eq!(const { TypeId::of::<[u8; 16]>().size() }, Some(16));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع حجم النوع، أو `None` إذا كان غير معروف الحجم.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn size(self) -> Option<usize> {
        intrinsics::reflection::type_id_size_of(self)
    }

    /// Returns the number of variants of the type represented by this `TypeId`.
    ///
    /// For enums, this is the number of variants. For structs and unions, this is always 1.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<Option<()>>().variants() }, 2);
    ///
    /// struct Unit;
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(const { TypeId::of::<Unit>().variants() }, 1);
    /// assert_eq!(const { TypeId::of::<Point>().variants() }, 1);
    /// assert_eq!(const { TypeId::of::<(f32, f32)>().variants() }, 1);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع عدد حالات النوع.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn variants(self) -> usize {
        intrinsics::reflection::type_id_variants(self)
    }

    // FIXME(reflection): make the errors nicer. This is a wider problem,
    // TypeId::fields has nice errors in the docs but those are not the ones shown
    // by rustc.
    /// Returns the variant representing type at the given index of the type represented by this `TypeId`. Use it to
    /// get the name of an enum variant or check whether it is non_exhaustive.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// enum Enum {
    ///     Unit,
    ///     Tuple(u32, u64),
    ///     #[non_exhaustive]
    ///     Struct { x: u32, y: u32, z: String },
    /// }
    /// assert_eq!(const { TypeId::of::<Enum>().variant(1).name() }, "Tuple");
    /// assert_eq!(const { TypeId::of::<Enum>().variant(2).name() }, "Struct");
    ///
    /// assert_eq!(const { TypeId::of::<Enum>().variant(1).non_exhaustive() }, false);
    /// assert_eq!(const { TypeId::of::<Enum>().variant(2).non_exhaustive() }, true);
    /// ```
    ///
    /// The variant index refer to the source order index of a variant in a type.
    ///
    /// Variant indexes are always `0..variant_count`, regardless of any custom discriminants that may have been defined.
    ///
    /// ```
    /// enum Enum {
    ///     Foo,  // variant index == 0
    ///     Bar,  // variant index == 1
    /// }
    /// ```
    ///
    /// Calling variant on the TypeId for a struct will be treated as a compile-time error. The same
    /// is true for out-of-bounds indexing on an enum.
    ///
    /// ```compile_fail,E0080
    /// # #![feature(type_info)]
    /// # use std::any::TypeId;
    /// #
    /// # struct Point {
    /// #     x: u32,
    /// #     y: u32,
    /// # }
    /// # enum Enum {
    /// #     Unit,
    /// #     Tuple(u32, u64),
    /// #     Struct { x: u32, y: u32, z: String },
    /// # }
    /// const {
    ///     _ = TypeId::of::<Point>().variant(0); // error: cannot get the variant of a struct
    ///     _ = TypeId::of::<Enum>().variant(10); // error: indexing out of bounds: the len is 3 but the index is 10
    /// }
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع الحالة في الموضع المعطى (للحصول على اسم حالة enum مثلًا).
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn variant(self, variant_index: usize) -> VariantId {
        intrinsics::reflection::type_id_fields(self, variant_index);
        VariantId { base: self, variant: variant_index }
    }

    /// Returns the number of fields at the given `variant_index` of the type represented by this `TypeId`.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(const { TypeId::of::<u32>().fields(0) }, 0);
    ///
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(const { TypeId::of::<Point>().fields(0) }, 2);
    ///
    /// enum Enum {
    ///     Unit,
    ///     Tuple(u32, u64),
    ///     Struct { x: u32, y: u32, z: String },
    /// }
    /// assert_eq!(const { TypeId::of::<Enum>().fields(0) }, 0);
    /// assert_eq!(const { TypeId::of::<Enum>().fields(1) }, 2);
    /// assert_eq!(const { TypeId::of::<Enum>().fields(2) }, 3);
    /// ```
    ///
    /// The variant index refers to the source order index of a variant in a type.
    ///
    /// For enums, these are always `0..variant_count`, regardless of any custom discriminants that may have been defined.
    /// `struct`s, `tuples`, and `unions`s are considered to have a single variant with variant index zero.
    ///
    /// ```
    /// enum Number {
    ///     Seven = 7, // variant index == 0
    ///     Six = 6,   // variant index == 1
    /// }
    /// ```
    ///
    /// Out-of-bounds indexing will be treated as a compile-time error.
    ///
    /// ```compile_fail,E0080
    /// # #![feature(type_info)]
    /// # use std::any::TypeId;
    /// #
    /// # struct Point {
    /// #     x: u32,
    /// #     y: u32,
    /// # }
    /// # enum Enum {
    /// #     Unit,
    /// #     Tuple(u32, u64),
    /// #     Struct { x: u32, y: u32, z: String },
    /// # }
    /// const {
    ///     _ = TypeId::of::<Point>().fields(10); // error: indexing out of bounds: the len is 2 but the index is 10
    ///     _ = TypeId::of::<Enum>().fields(10); // error: indexing out of bounds: the len is 3 but the index is 10
    /// }
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع عدد الحقول في الحالة `variant_index`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    // FIXME(type_info): Add enum variant pattern types and use them to represent individual variants
    // Then add a `variant` method to get a wrapper around such a pattern type (similar to the FRT
    // type we have) and add methods on that. It's the only way to really sensibly represent
    // things like `non_exhaustive` which can be applied to variants as well.
    pub fn fields(self, variant_index: usize) -> usize {
        intrinsics::reflection::type_id_fields(self, variant_index)
    }

    /// Returns the field representing type at the given index of the type represented by this `TypeId`.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(const { TypeId::of::<Point>().field(0, 0).type_id() }, TypeId::of::<u32>());
    /// assert_eq!(const { TypeId::of::<Point>().field(0, 1).type_id() }, TypeId::of::<u32>());
    ///
    /// enum Enum {
    ///     Unit,
    ///     Tuple(u32, u64),
    ///     Struct { x: u32, y: u32, z: String },
    /// }
    /// assert_eq!(const { TypeId::of::<Enum>().field(1, 0).type_id() }, TypeId::of::<u32>());
    /// assert_eq!(const { TypeId::of::<Enum>().field(2, 2).type_id() }, TypeId::of::<String>());
    /// ```
    ///
    /// The variant index and field index refer to the source order index of a variant in a type and
    /// the source order index of a field in a variant, respectively.
    ///
    /// For enums, variant indexes are always `0..variant_count`, regardless of any custom discriminants that may have been defined.
    /// `struct`s, `tuples`, and `unions`s are considered to have a single variant with variant index zero.
    ///
    /// As for field indexes, they may not be the same as the layout order for `repr(Rust)` types, but they are for `repr(C)` types.
    ///
    /// ```
    /// enum Enum {
    ///     Foo,  // variant index == 0
    ///     Bar { // variant index == 1
    ///         a: (), // field index == 0 in `Bar`
    ///         b: (), // field index == 1 in `Bar`
    ///     }
    /// }
    /// ```
    ///
    /// Out-of-bounds indexing will be treated as a compile-time error.
    ///
    /// ```compile_fail,E0080
    /// # #![feature(type_info)]
    /// # use std::any::TypeId;
    /// #
    /// # struct Point {
    /// #     x: u32,
    /// #     y: u32,
    /// # }
    /// # enum Enum {
    /// #     Unit,
    /// #     Tuple(u32, u64),
    /// #     Struct { x: u32, y: u32, z: String },
    /// # }
    /// const {
    ///     _ = TypeId::of::<Point>().field(0, 10); // error: indexing out of bounds: the len is 2 but the index is 10
    ///     _ = TypeId::of::<Enum>().field(2, 10); // error: indexing out of bounds: the len is 3 but the index is 10
    /// }
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع الحقل في الموضع المعطى.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn field(self, variant_index: usize, field_index: usize) -> FieldId {
        FieldId {
            frt_type_id: intrinsics::reflection::type_id_field_representing_type(
                self,
                variant_index,
                field_index,
            ),
        }
    }

    /// Returns whether a type is marked with `#[non_exhaustive]`.
    /// Returns `false` for everything but adts.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع هل النوع معلَّم بـ `#[non_exhaustive]`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn non_exhaustive(self) -> bool {
        intrinsics::reflection::type_id_non_exhaustive(self)
    }

    /// Returns a list of generic parameters of the type.
    /// Returns an empty slice for everything that doesn't have generics.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع قائمة المعاملات العامة للنوع.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn generics(self) -> &'static [Generic] {
        intrinsics::reflection::type_id_generics(self)
    }

    /// Given a `TypeId` that represents a pointer this returns the `TypeId`
    /// which that pointer points to. When called on anything else this returns
    /// None.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert_eq!(
    ///     const { TypeId::of::<&i32>().points_to() },
    ///     const { Some(TypeId::of::<i32>()) },
    /// );
    ///
    /// assert_eq!(
    ///     const { TypeId::of::<*const i32>().points_to() },
    ///     const { Some(TypeId::of::<i32>()) },
    /// );
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مع مؤشر يرجع `TypeId` لما يشير إليه، وإلا يرجع None.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn points_to(self) -> Option<TypeId> {
        intrinsics::reflection::type_id_points_to(self)
    }

    /// Given a `TypeId` that represents a pointer returns whether that pointer is mutable.
    /// When called on anything else this returns `false`.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// assert!(const { TypeId::of::<&mut i32>().points_mutably() });
    /// assert!(const { !TypeId::of::<&i32>().points_mutably() });
    ///
    /// assert!(!const { TypeId::of::<*const i32>().points_mutably() });
    /// assert!(const { TypeId::of::<*mut i32>().points_mutably() });
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مع مؤشر يرجع هل هو قابل للتعديل، وإلا يرجع `false`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn points_mutably(self) -> bool {
        intrinsics::reflection::type_id_points_mutably(self)
    }

    /// Given a `TypeId` that represents a function pointer returns an
    /// [`FnPtr`]. When called on something else this returns `None`.
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    /// use std::mem::type_info::{Abi, FnPtr};
    ///
    /// const F: FnPtr = TypeId::of::<fn(u8, u16) -> usize>()
    ///     .function_ptr()
    ///     .expect("TypeId of a function ptr");
    ///
    /// assert!(F.inputs() == [TypeId::of::<u8>(), TypeId::of::<u16>()]);
    /// assert!(F.output() == TypeId::of::<usize>());
    /// assert!(F.abi() == Abi::default());
    /// ```
    /// ```
    /// #![feature(type_info)]
    /// # use std::any::TypeId;
    /// # use std::mem::type_info::{Abi, FnPtr};
    /// #
    /// const F: FnPtr = TypeId::of::<unsafe fn()>()
    ///     .function_ptr()
    ///     .expect("TypeId of a function ptr");
    ///
    /// assert!(F.inputs() == []);
    /// assert!(F.output() == TypeId::of::<()>());
    /// assert!(F.abi() == Abi::default());
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// مع مؤشر دالة يرجع `FnPtr`، وإلا يرجع `None`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn function_ptr(self) -> Option<FnPtr> {
        intrinsics::reflection::type_id_function_ptr(self)
    }
}

/// Variant representing type ID. Representing a variant of an enum.
///
/// <div lang="ar" dir="rtl">
///
/// معرّف حالة enum.
///
/// </div>
#[derive(Copy, PartialOrd, Ord, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[unstable(feature = "type_info", issue = "146922")]
pub struct VariantId {
    base: TypeId,
    variant: usize,
}

#[unstable(feature = "type_info", issue = "146922")]
impl fmt::Debug for VariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Variant({:#034x}-{})", self.base.as_u128(), self.variant)
    }
}

impl VariantId {
    /// Returns the name of the variant.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// enum Enum {
    ///     Unit,
    ///     Tuple(bool),
    ///     Struct { a: bool },
    /// }
    /// assert_eq!(
    ///     const { TypeId::of::<Enum>().variant(1).name() },
    ///     "Tuple",
    /// );
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع اسم الحالة.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn name(self) -> &'static str {
        intrinsics::reflection::type_id_variant_name(self.base, self.variant)
    }

    /// Returns whether this variant is marked with `#[non_exhaustive]`.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع هل الحالة معلَّمة بـ `#[non_exhaustive]`.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn non_exhaustive(self) -> bool {
        intrinsics::reflection::type_id_variant_non_exhaustive(self.base, self.variant)
    }
}

/// Field representing type ID. Representing a field of a struct, tuple or enum variant.
///
/// <div lang="ar" dir="rtl">
///
/// معرّف حقل في struct أو صف أو حالة enum.
///
/// </div>
#[derive(Copy, PartialOrd, Ord, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[unstable(feature = "type_info", issue = "146922")]
pub struct FieldId {
    frt_type_id: TypeId,
}

#[unstable(feature = "type_info", issue = "146922")]
impl fmt::Debug for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldId({:#034x})", self.frt_type_id.as_u128())
    }
}

impl FieldId {
    /// Returns the `TypeId` of the actual field type.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(
    ///     const { TypeId::of::<Point>().field(0, 0).type_id() },
    ///     TypeId::of::<u32>()
    /// );
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `TypeId` لنوع الحقل الفعلي.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn type_id(self) -> TypeId {
        intrinsics::field_representing_type_actual_type_id(self.frt_type_id)
    }

    /// Returns the name of the field.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(
    ///     const { TypeId::of::<Point>().field(0, 0).name() },
    ///     "x",
    /// );
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع اسم الحقل.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn name(self) -> &'static str {
        intrinsics::field_representing_type_name(self.frt_type_id)
    }
    /// Returns the offset of the field wrt to its containing type.
    ///
    /// ```
    /// #![feature(type_info)]
    /// use std::any::TypeId;
    ///
    /// #[repr(C)]
    /// struct Point {
    ///     x: u32,
    ///     y: u32,
    /// }
    /// assert_eq!(
    ///     const { TypeId::of::<Point>().field(0, 1).offset() },
    ///     4,
    /// );
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع إزاحة الحقل داخل النوع الذي يحتويه.
    ///
    /// </div>
    #[unstable(feature = "type_info", issue = "146922")]
    #[rustc_const_unstable(feature = "type_info", issue = "146922")]
    #[rustc_comptime]
    pub fn offset(self) -> usize {
        intrinsics::field_representing_type_offset(self.frt_type_id)
    }
}
