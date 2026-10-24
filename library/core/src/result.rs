//! Error handling with the `Result` type.
//!
//! [`Result<T, E>`][`Result`] is the type used for returning and propagating
//! errors. It is an enum with the variants, [`Ok(T)`], representing
//! success and containing a value, and [`Err(E)`], representing error
//! and containing an error value.
//!
//! ```
//! # #[allow(dead_code)]
//! enum Result<T, E> {
//!    Ok(T),
//!    Err(E),
//! }
//! ```
//!
//! Functions return [`Result`] whenever errors are expected and
//! recoverable. In the `std` crate, [`Result`] is most prominently used
//! for [I/O](../../std/io/index.html).
//!
//! A simple function returning [`Result`] might be
//! defined and used like so:
//!
//! ```
//! #[derive(Debug)]
//! enum Version { Version1, Version2 }
//!
//! fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
//!     match header.get(0) {
//!         None => Err("invalid header length"),
//!         Some(&1) => Ok(Version::Version1),
//!         Some(&2) => Ok(Version::Version2),
//!         Some(_) => Err("invalid version"),
//!     }
//! }
//!
//! let version = parse_version(&[1, 2, 3, 4]);
//! match version {
//!     Ok(v) => println!("working with version: {v:?}"),
//!     Err(e) => println!("error parsing header: {e:?}"),
//! }
//! ```
//!
//! Pattern matching on [`Result`]s is clear and straightforward for
//! simple cases, but [`Result`] comes with some convenience methods
//! that make working with it more succinct.
//!
//! ```
//! // The `is_ok` and `is_err` methods do what they say.
//! let good_result: Result<i32, i32> = Ok(10);
//! let bad_result: Result<i32, i32> = Err(10);
//! assert!(good_result.is_ok() && !good_result.is_err());
//! assert!(bad_result.is_err() && !bad_result.is_ok());
//!
//! // `map` and `map_err` consume the `Result` and produce another.
//! let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
//! let bad_result: Result<i32, i32> = bad_result.map_err(|i| i - 1);
//! assert_eq!(good_result, Ok(11));
//! assert_eq!(bad_result, Err(9));
//!
//! // Use `and_then` to continue the computation.
//! let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));
//! assert_eq!(good_result, Ok(true));
//!
//! // Use `or_else` to handle the error.
//! let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
//! assert_eq!(bad_result, Ok(29));
//!
//! // Consume the result and return the contents with `unwrap`.
//! let final_awesome_result = good_result.unwrap();
//! assert!(final_awesome_result)
//! ```
//!
//! # Results must be used
//!
//! A common problem with using return values to indicate errors is
//! that it is easy to ignore the return value, thus failing to handle
//! the error. [`Result`] is annotated with the `#[must_use]` attribute,
//! which will cause the compiler to issue a warning when a Result
//! value is ignored. This makes [`Result`] especially useful with
//! functions that may encounter errors but don't otherwise return a
//! useful value.
//!
//! Consider the [`write_all`] method defined for I/O types
//! by the [`Write`] trait:
//!
//! ```
//! use std::io;
//!
//! trait Write {
//!     fn write_all(&mut self, bytes: &[u8]) -> Result<(), io::Error>;
//! }
//! ```
//!
//! *Note: The actual definition of [`Write`] uses [`io::Result`], which
//! is just a synonym for <code>[Result]<T, [io::Error]></code>.*
//!
//! This method doesn't produce a value, but the write may
//! fail. It's crucial to handle the error case, and *not* write
//! something like this:
//!
//! ```no_run
//! # #![allow(unused_must_use)] // \o/
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! let mut file = File::create("valuable_data.txt").unwrap();
//! // If `write_all` errors, then we'll never know, because the return
//! // value is ignored.
//! file.write_all(b"important message");
//! ```
//!
//! If you *do* write that in Rust, the compiler will give you a
//! warning (by default, controlled by the `unused_must_use` lint).
//!
//! You might instead, if you don't want to handle the error, simply
//! assert success with [`expect`]. This will panic if the
//! write fails, providing a message explaining why the write was expected
//! to succeed:
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! let mut file = File::create("valuable_data.txt").unwrap();
//! file.write_all(b"important message").expect("writing to the file should succeed");
//! ```
//!
//! You might also simply assert success:
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::io::prelude::*;
//! # let mut file = File::create("valuable_data.txt").unwrap();
//! assert!(file.write_all(b"important message").is_ok());
//! ```
//!
//! Or propagate the error up the call stack with [`?`]:
//!
//! ```
//! # use std::fs::File;
//! # use std::io::prelude::*;
//! # use std::io;
//! # #[allow(dead_code)]
//! fn write_message() -> io::Result<()> {
//!     let mut file = File::create("valuable_data.txt")?;
//!     file.write_all(b"important message")?;
//!     Ok(())
//! }
//! ```
//!
//! # The question mark operator, `?`
//!
//! When writing code that calls many functions that return the
//! [`Result`] type, the error handling can be tedious. The question mark
//! operator, [`?`], hides some of the boilerplate of propagating errors
//! up the call stack.
//!
//! It replaces this:
//!
//! ```
//! # #![allow(dead_code)]
//! use std::fs::File;
//! use std::io::prelude::*;
//! use std::io;
//!
//! struct Info {
//!     name: String,
//!     age: i32,
//!     rating: i32,
//! }
//!
//! fn write_info(info: &Info) -> io::Result<()> {
//!     // Early return on error
//!     let mut file = match File::create("my_best_friends.txt") {
//!            Err(e) => return Err(e),
//!            Ok(f) => f,
//!     };
//!     if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
//!         return Err(e)
//!     }
//!     if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
//!         return Err(e)
//!     }
//!     if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
//!         return Err(e)
//!     }
//!     Ok(())
//! }
//! ```
//!
//! With this:
//!
//! ```
//! # #![allow(dead_code)]
//! use std::fs::File;
//! use std::io::prelude::*;
//! use std::io;
//!
//! struct Info {
//!     name: String,
//!     age: i32,
//!     rating: i32,
//! }
//!
//! fn write_info(info: &Info) -> io::Result<()> {
//!     let mut file = File::create("my_best_friends.txt")?;
//!     // Early return on error
//!     file.write_all(format!("name: {}\n", info.name).as_bytes())?;
//!     file.write_all(format!("age: {}\n", info.age).as_bytes())?;
//!     file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
//!     Ok(())
//! }
//! ```
//!
//! *It's much nicer!*
//!
//! Ending the expression with [`?`] will result in the [`Ok`]'s unwrapped value, unless the result
//! is [`Err`], in which case [`Err`] is returned early from the enclosing function.
//!
//! [`?`] can be used in functions that return [`Result`] because of the
//! early return of [`Err`] that it provides.
//!
//! [`expect`]: Result::expect
//! [`Write`]: ../../std/io/trait.Write.html "io::Write"
//! [`write_all`]: ../../std/io/trait.Write.html#method.write_all "io::Write::write_all"
//! [`io::Result`]: ../../std/io/type.Result.html "io::Result"
//! [`?`]: crate::ops::Try
//! [`Ok(T)`]: Ok
//! [`Err(E)`]: Err
//! [io::Error]: ../../std/io/struct.Error.html "io::Error"
//!
//! # Representation
//!
//! In some cases, [`Result<T, E>`] comes with size, alignment, and ABI
//! guarantees. Specifically, one of either the `T` or `E` type must be a type
//! that qualifies for the `Option` [representation guarantees][opt-rep] (let's
//! call that type `I`), and the *other* type is a zero-sized type with
//! alignment 1 (a "1-ZST").
//!
//! If that is the case, then `Result<T, E>` has the same size, alignment, and
//! [function call ABI] as `I` (and therefore, as `Option<I>`). If `I` is `T`,
//! it is therefore sound to transmute a value `t` of type `I` to type
//! `Result<T, E>` (producing the value `Ok(t)`) and to transmute a value
//! `Ok(t)` of type `Result<T, E>` to type `I` (producing the value `t`). If `I`
//! is `E`, the same applies with `Ok` replaced by `Err`.
//!
//! For example, `NonZeroI32` qualifies for the `Option` representation
//! guarantees and `()` is a zero-sized type with alignment 1. This means that
//! both `Result<NonZeroI32, ()>` and `Result<(), NonZeroI32>` have the same
//! size, alignment, and ABI as `NonZeroI32` (and `Option<NonZeroI32>`). The
//! only difference between these is in the implied semantics:
//!
//! * `Option<NonZeroI32>` is "a non-zero i32 might be present"
//! * `Result<NonZeroI32, ()>` is "a non-zero i32 success result, if any"
//! * `Result<(), NonZeroI32>` is "a non-zero i32 error result, if any"
//!
//! [opt-rep]: ../option/index.html#representation "Option Representation"
//! [function call ABI]: ../primitive.fn.html#abi-compatibility
//!
//! # Method overview
//!
//! In addition to working with pattern matching, [`Result`] provides a
//! wide variety of different methods.
//!
//! ## Querying the variant
//!
//! The [`is_ok`] and [`is_err`] methods return [`true`] if the [`Result`]
//! is [`Ok`] or [`Err`], respectively.
//!
//! The [`is_ok_and`] and [`is_err_and`] methods apply the provided function
//! to the contents of the [`Result`] to produce a boolean value. If the [`Result`] does not have the expected variant
//! then [`false`] is returned instead without executing the function.
//!
//! [`is_err`]: Result::is_err
//! [`is_ok`]: Result::is_ok
//! [`is_ok_and`]: Result::is_ok_and
//! [`is_err_and`]: Result::is_err_and
//!
//! ## Adapters for working with references
//!
//! * [`as_ref`] converts from `&Result<T, E>` to `Result<&T, &E>`
//! * [`as_mut`] converts from `&mut Result<T, E>` to `Result<&mut T, &mut E>`
//! * [`as_deref`] converts from `&Result<T, E>` to `Result<&T::Target, &E>`
//! * [`as_deref_mut`] converts from `&mut Result<T, E>` to
//!   `Result<&mut T::Target, &mut E>`
//!
//! [`as_deref`]: Result::as_deref
//! [`as_deref_mut`]: Result::as_deref_mut
//! [`as_mut`]: Result::as_mut
//! [`as_ref`]: Result::as_ref
//!
//! ## Extracting contained values
//!
//! These methods extract the contained value in a [`Result<T, E>`] when it
//! is the [`Ok`] variant. If the [`Result`] is [`Err`]:
//!
//! * [`expect`] panics with a provided custom message
//! * [`unwrap`] panics with a generic message
//! * [`unwrap_or`] returns the provided default value
//! * [`unwrap_or_default`] returns the default value of the type `T`
//!   (which must implement the [`Default`] trait)
//! * [`unwrap_or_else`] returns the result of evaluating the provided
//!   function
//! * [`unwrap_unchecked`] produces *[undefined behavior]*
//!
//! The panicking methods [`expect`] and [`unwrap`] require `E` to
//! implement the [`Debug`] trait.
//!
//! [`Debug`]: crate::fmt::Debug
//! [`expect`]: Result::expect
//! [`unwrap`]: Result::unwrap
//! [`unwrap_or`]: Result::unwrap_or
//! [`unwrap_or_default`]: Result::unwrap_or_default
//! [`unwrap_or_else`]: Result::unwrap_or_else
//! [`unwrap_unchecked`]: Result::unwrap_unchecked
//! [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
//!
//! These methods extract the contained value in a [`Result<T, E>`] when it
//! is the [`Err`] variant. They require `T` to implement the [`Debug`]
//! trait. If the [`Result`] is [`Ok`]:
//!
//! * [`expect_err`] panics with a provided custom message
//! * [`unwrap_err`] panics with a generic message
//! * [`unwrap_err_unchecked`] produces *[undefined behavior]*
//!
//! [`Debug`]: crate::fmt::Debug
//! [`expect_err`]: Result::expect_err
//! [`unwrap_err`]: Result::unwrap_err
//! [`unwrap_err_unchecked`]: Result::unwrap_err_unchecked
//! [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
//!
//! ## Transforming contained values
//!
//! These methods transform [`Result`] to [`Option`]:
//!
//! * [`err`][Result::err] transforms [`Result<T, E>`] into [`Option<E>`],
//!   mapping [`Err(e)`] to [`Some(e)`] and [`Ok(v)`] to [`None`]
//! * [`ok`][Result::ok] transforms [`Result<T, E>`] into [`Option<T>`],
//!   mapping [`Ok(v)`] to [`Some(v)`] and [`Err(e)`] to [`None`]
//! * [`transpose`] transposes a [`Result`] of an [`Option`] into an
//!   [`Option`] of a [`Result`]
//!
// Do NOT add link reference definitions for `err` or `ok`, because they
// will generate numerous incorrect URLs for `Err` and `Ok` elsewhere, due
// to case folding.
//!
//! [`Err(e)`]: Err
//! [`Ok(v)`]: Ok
//! [`Some(e)`]: Option::Some
//! [`Some(v)`]: Option::Some
//! [`transpose`]: Result::transpose
//!
//! These methods transform the contained value of the [`Ok`] variant:
//!
//! * [`map`] transforms [`Result<T, E>`] into [`Result<U, E>`] by applying
//!   the provided function to the contained value of [`Ok`] and leaving
//!   [`Err`] values unchanged
//! * [`inspect`] takes ownership of the [`Result`], applies the
//!   provided function to the contained value by reference,
//!   and then returns the [`Result`]
//!
//! [`map`]: Result::map
//! [`inspect`]: Result::inspect
//!
//! These methods transform the contained value of the [`Err`] variant:
//!
//! * [`map_err`] transforms [`Result<T, E>`] into [`Result<T, F>`] by
//!   applying the provided function to the contained value of [`Err`] and
//!   leaving [`Ok`] values unchanged
//! * [`inspect_err`] takes ownership of the [`Result`], applies the
//!   provided function to the contained value of [`Err`] by reference,
//!   and then returns the [`Result`]
//!
//! [`map_err`]: Result::map_err
//! [`inspect_err`]: Result::inspect_err
//!
//! These methods transform a [`Result<T, E>`] into a value of a possibly
//! different type `U`:
//!
//! * [`map_or`] applies the provided function to the contained value of
//!   [`Ok`], or returns the provided default value if the [`Result`] is
//!   [`Err`]
//! * [`map_or_else`] applies the provided function to the contained value
//!   of [`Ok`], or applies the provided default fallback function to the
//!   contained value of [`Err`]
//!
//! [`map_or`]: Result::map_or
//! [`map_or_else`]: Result::map_or_else
//!
//! ## Boolean operators
//!
//! These methods treat the [`Result`] as a boolean value, where [`Ok`]
//! acts like [`true`] and [`Err`] acts like [`false`]. There are two
//! categories of these methods: ones that take a [`Result`] as input, and
//! ones that take a function as input (to be lazily evaluated).
//!
//! The [`and`] and [`or`] methods take another [`Result`] as input, and
//! produce a [`Result`] as output. The [`and`] method can produce a
//! [`Result<U, E>`] value having a different inner type `U` than
//! [`Result<T, E>`]. The [`or`] method can produce a [`Result<T, F>`]
//! value having a different error type `F` than [`Result<T, E>`].
//!
//! | method  | self     | input     | output   |
//! |---------|----------|-----------|----------|
//! | [`and`] | `Err(e)` | (ignored) | `Err(e)` |
//! | [`and`] | `Ok(x)`  | `Err(d)`  | `Err(d)` |
//! | [`and`] | `Ok(x)`  | `Ok(y)`   | `Ok(y)`  |
//! | [`or`]  | `Err(e)` | `Err(d)`  | `Err(d)` |
//! | [`or`]  | `Err(e)` | `Ok(y)`   | `Ok(y)`  |
//! | [`or`]  | `Ok(x)`  | (ignored) | `Ok(x)`  |
//!
//! [`and`]: Result::and
//! [`or`]: Result::or
//!
//! The [`and_then`] and [`or_else`] methods take a function as input, and
//! only evaluate the function when they need to produce a new value. The
//! [`and_then`] method can produce a [`Result<U, E>`] value having a
//! different inner type `U` than [`Result<T, E>`]. The [`or_else`] method
//! can produce a [`Result<T, F>`] value having a different error type `F`
//! than [`Result<T, E>`].
//!
//! | method       | self     | function input | function result | output   |
//! |--------------|----------|----------------|-----------------|----------|
//! | [`and_then`] | `Err(e)` | (not provided) | (not evaluated) | `Err(e)` |
//! | [`and_then`] | `Ok(x)`  | `x`            | `Err(d)`        | `Err(d)` |
//! | [`and_then`] | `Ok(x)`  | `x`            | `Ok(y)`         | `Ok(y)`  |
//! | [`or_else`]  | `Err(e)` | `e`            | `Err(d)`        | `Err(d)` |
//! | [`or_else`]  | `Err(e)` | `e`            | `Ok(y)`         | `Ok(y)`  |
//! | [`or_else`]  | `Ok(x)`  | (not provided) | (not evaluated) | `Ok(x)`  |
//!
//! [`and_then`]: Result::and_then
//! [`or_else`]: Result::or_else
//!
//! ## Comparison operators
//!
//! If `T` and `E` both implement [`PartialOrd`] then [`Result<T, E>`] will
//! derive its [`PartialOrd`] implementation.  With this order, an [`Ok`]
//! compares as less than any [`Err`], while two [`Ok`] or two [`Err`]
//! compare as their contained values would in `T` or `E` respectively.  If `T`
//! and `E` both also implement [`Ord`], then so does [`Result<T, E>`].
//!
//! ```
//! assert!(Ok(1) < Err(0));
//! let x: Result<i32, ()> = Ok(0);
//! let y = Ok(1);
//! assert!(x < y);
//! let x: Result<(), i32> = Err(0);
//! let y = Err(1);
//! assert!(x < y);
//! ```
//!
//! ## Iterating over `Result`
//!
//! A [`Result`] can be iterated over. This can be helpful if you need an
//! iterator that is conditionally empty. The iterator will either produce
//! a single value (when the [`Result`] is [`Ok`]), or produce no values
//! (when the [`Result`] is [`Err`]). For example, [`into_iter`] acts like
//! [`once(v)`] if the [`Result`] is [`Ok(v)`], and like [`empty()`] if the
//! [`Result`] is [`Err`].
//!
//! [`Ok(v)`]: Ok
//! [`empty()`]: crate::iter::empty
//! [`once(v)`]: crate::iter::once
//!
//! Iterators over [`Result<T, E>`] come in three types:
//!
//! * [`into_iter`] consumes the [`Result`] and produces the contained
//!   value
//! * [`iter`] produces an immutable reference of type `&T` to the
//!   contained value
//! * [`iter_mut`] produces a mutable reference of type `&mut T` to the
//!   contained value
//!
//! See [Iterating over `Option`] for examples of how this can be useful.
//!
//! [Iterating over `Option`]: crate::option#iterating-over-option
//! [`into_iter`]: Result::into_iter
//! [`iter`]: Result::iter
//! [`iter_mut`]: Result::iter_mut
//!
//! You might want to use an iterator chain to do multiple instances of an
//! operation that can fail, but would like to ignore failures while
//! continuing to process the successful results. In this example, we take
//! advantage of the iterable nature of [`Result`] to select only the
//! [`Ok`] values using [`flatten`][Iterator::flatten].
//!
//! ```
//! # use std::str::FromStr;
//! let mut results = vec![];
//! let mut errs = vec![];
//! let nums: Vec<_> = ["17", "not a number", "99", "-27", "768"]
//!    .into_iter()
//!    .map(u8::from_str)
//!    // Save clones of the raw `Result` values to inspect
//!    .inspect(|x| results.push(x.clone()))
//!    // Challenge: explain how this captures only the `Err` values
//!    .inspect(|x| errs.extend(x.clone().err()))
//!    .flatten()
//!    .collect();
//! assert_eq!(errs.len(), 3);
//! assert_eq!(nums, [17, 99]);
//! println!("results {results:?}");
//! println!("errs {errs:?}");
//! println!("nums {nums:?}");
//! ```
//!
//! ## Collecting into `Result`
//!
//! [`Result`] implements the [`FromIterator`][impl-FromIterator] trait,
//! which allows an iterator over [`Result`] values to be collected into a
//! [`Result`] of a collection of each contained value of the original
//! [`Result`] values, or [`Err`] if any of the elements was [`Err`].
//!
//! [impl-FromIterator]: Result#impl-FromIterator%3CResult%3CT,+E%3E%3E-for-Result%3CV,+E%3E
//!
//! ```
//! let v = [Ok(2), Ok(4), Err("err!"), Ok(8)];
//! let res: Result<Vec<_>, &str> = v.into_iter().collect();
//! assert_eq!(res, Err("err!"));
//! let v = [Ok(2), Ok(4), Ok(8)];
//! let res: Result<Vec<_>, &str> = v.into_iter().collect();
//! assert_eq!(res, Ok(vec![2, 4, 8]));
//! ```
//!
//! [`Result`] also implements the [`Product`][impl-Product] and
//! [`Sum`][impl-Sum] traits, allowing an iterator over [`Result`] values
//! to provide the [`product`][Iterator::product] and
//! [`sum`][Iterator::sum] methods.
//!
//! [impl-Product]: Result#impl-Product%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E
//! [impl-Sum]: Result#impl-Sum%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E
//!
//! ```
//! let v = [Err("error!"), Ok(1), Ok(2), Ok(3), Err("foo")];
//! let res: Result<i32, &str> = v.into_iter().sum();
//! assert_eq!(res, Err("error!"));
//! let v = [Ok(1), Ok(2), Ok(21)];
//! let res: Result<i32, &str> = v.into_iter().product();
//! assert_eq!(res, Ok(42));
//! ```
//!
//! <div lang="ar" dir="rtl">
//!
//! معالجة الأخطاء بالنوع `Result`.
//!
//! `Result<T, E>` هو النوع المستخدم لإرجاع الأخطاء وتمريرها. هو enum له حالتان: `Ok(T)` تمثّل
//! النجاح وفيها قيمة، و`Err(E)` تمثّل الخطأ وفيها قيمة الخطأ.
//!
//! ترجع الدوال `Result` كلما كانت الأخطاء متوقعة ويمكن التعافي منها. في المكتبة `std` يُستخدم
//! `Result` بشكل بارز في الإدخال والإخراج (I/O).
//!
//! يمكن تعريف واستخدام دالة بسيطة ترجع `Result` هكذا (انظر المثال).
//!
//! مطابقة الأنماط على `Result` واضحة ومباشرة في الحالات البسيطة، لكن `Result` يأتي مع دوال مريحة
//! تجعل التعامل معه أقصر.
//!
//! # يجب استخدام النتائج
//!
//! مشكلة شائعة في استخدام قيم الإرجاع للدلالة على الأخطاء أنه من السهل تجاهل القيمة، فلا يُعالج
//! الخطأ. النوع `Result` معلَّم بالخاصية `#[must_use]`، فيُصدر المترجم تحذيرًا عند تجاهل قيمة
//! `Result`. هذا يجعل `Result` مفيدًا خاصة مع الدوال التي قد تواجه أخطاء لكنها لا ترجع قيمة مفيدة
//! غير ذلك.
//!
//! خذ مثلًا الدالة `write_all` المعرّفة لأنواع الإدخال والإخراج في الـ trait `Write`.
//!
//! *ملاحظة: التعريف الفعلي لـ `Write` يستخدم `io::Result` وهو مجرد اسم آخر لـ
//! `Result<T, io::Error>`.*
//!
//! هذه الدالة لا تنتج قيمة، لكن الكتابة قد تفشل. من المهم معالجة حالة الخطأ، و*عدم* كتابة شيء
//! يتجاهلها.
//!
//! إذا كتبت ذلك في Rust فسيعطيك المترجم تحذيرًا (افتراضيًا، يتحكم فيه الـ lint
//! `unused_must_use`).
//!
//! إذا لم ترد معالجة الخطأ، يمكنك بدلًا من ذلك تأكيد النجاح بـ `expect`. سيسبب هذا panic إذا فشلت
//! الكتابة، مع رسالة توضح لماذا كان متوقعًا أن تنجح.
//!
//! أو يمكنك تأكيد النجاح ببساطة، أو تمرير الخطأ إلى أعلى سلسلة الاستدعاءات بـ `?`.
//!
//! # عامل علامة الاستفهام `?`
//!
//! عند كتابة كود يستدعي دوالًا كثيرة ترجع `Result`، تصبح معالجة الأخطاء مملة. عامل علامة الاستفهام
//! `?` يخفي جزءًا من الكود المتكرر لتمرير الأخطاء إلى أعلى سلسلة الاستدعاءات.
//!
//! إنهاء التعبير بـ `?` يعطي القيمة داخل `Ok`، إلا إذا كانت النتيجة `Err`، فحينها تُرجع `Err`
//! مبكرًا
//! من الدالة المحيطة.
//!
//! يمكن استخدام `?` في الدوال التي ترجع `Result` بسبب الإرجاع المبكر لـ `Err` الذي يوفره.
//!
//! # التمثيل في الذاكرة
//!
//! في بعض الحالات يأتي `Result<T, E>` بضمانات للحجم والمحاذاة والـ ABI. تحديدًا: يجب أن يكون أحد
//! النوعين `T` أو `E` نوعًا تنطبق عليه ضمانات تمثيل `Option` (لنسمّه `I`)، والنوع *الآخر* نوعًا
//! بحجم صفر ومحاذاة 1.
//!
//! في هذه الحالة يكون لـ `Result<T, E>` نفس الحجم والمحاذاة واتفاقية استدعاء الدوال مثل `I`
//! (وبالتالي
//! مثل `Option<I>`). إذا كان `I` هو `T` فمن السليم تحويل قيمة `t` من النوع `I` إلى `Result<T, E>`
//! بـ transmute (فتنتج `Ok(t)`) والعكس. وإذا كان `I` هو `E` ينطبق نفس الشيء مع `Err` بدل `Ok`.
//!
//! مثلًا، `NonZeroI32` تنطبق عليه ضمانات تمثيل `Option`، و`()` نوع بحجم صفر ومحاذاة 1. إذًا
//! `Result<NonZeroI32, ()>` و`Result<(), NonZeroI32>` لهما نفس حجم ومحاذاة وABI النوع `NonZeroI32`
//! (و`Option<NonZeroI32>`). الفرق الوحيد بينها في المعنى:
//!
//! * `Option<NonZeroI32>` تعني "قد يوجد i32 غير صفري"
//! * `Result<NonZeroI32, ()>` تعني "نتيجة نجاح i32 غير صفري، إن وُجدت"
//! * `Result<(), NonZeroI32>` تعني "نتيجة خطأ i32 غير صفري، إن وُجدت"
//!
//! # نظرة عامة على الدوال
//!
//! إضافة إلى مطابقة الأنماط، يوفر `Result` دوالًا كثيرة متنوعة.
//!
//! ## معرفة نوع القيمة
//!
//! الدالتان `is_ok` و`is_err` ترجعان `true` إذا كان الـ `Result` هو `Ok` أو `Err` على الترتيب.
//!
//! الدالتان `is_ok_and` و`is_err_and` تطبّقان الدالة المعطاة على محتوى الـ `Result` لإنتاج قيمة
//! منطقية. إذا لم تكن الحالة هي المتوقعة تُرجع `false` بدون تنفيذ الدالة.
//!
//! ## محوّلات للتعامل مع المراجع
//!
//! * `as_ref` يحوّل من `&Result<T, E>` إلى `Result<&T, &E>`
//! * `as_mut` يحوّل من `&mut Result<T, E>` إلى `Result<&mut T, &mut E>`
//! * `as_deref` يحوّل من `&Result<T, E>` إلى `Result<&T::Target, &E>`
//! * `as_deref_mut` يحوّل من `&mut Result<T, E>` إلى `Result<&mut T::Target, &mut E>`
//!
//! ## استخراج القيم الموجودة
//!
//! هذه الدوال تستخرج القيمة الموجودة في `Result<T, E>` عندما يكون `Ok`. أما إذا كان `Err`:
//!
//! * `expect` يسبب panic برسالة مخصصة تعطيها
//! * `unwrap` يسبب panic برسالة عامة
//! * `unwrap_or` يرجع القيمة الافتراضية المعطاة
//! * `unwrap_or_default` يرجع القيمة الافتراضية للنوع `T` (يجب أن يطبّق `Default`)
//! * `unwrap_or_else` يرجع ناتج تنفيذ الدالة المعطاة
//! * `unwrap_unchecked` يسبب *سلوكًا غير معرّف*
//!
//! الدالتان `expect` و`unwrap` اللتان قد تسببان panic تتطلبان أن يطبّق `E` الـ trait `Debug`.
//!
//! هذه الدوال تستخرج القيمة الموجودة في `Result<T, E>` عندما يكون `Err`، وتتطلب أن يطبّق `T` الـ
//! trait `Debug`. أما إذا كان `Ok`:
//!
//! * `expect_err` يسبب panic برسالة مخصصة تعطيها
//! * `unwrap_err` يسبب panic برسالة عامة
//! * `unwrap_err_unchecked` يسبب *سلوكًا غير معرّف*
//!
//! ## تحويل القيم الموجودة
//!
//! هذه الدوال تحوّل `Result` إلى `Option`:
//!
//! * `err` يحوّل `Result<T, E>` إلى `Option<E>`: `Err(e)` يصبح `Some(e)` و`Ok(v)` يصبح `None`
//! * `ok` يحوّل `Result<T, E>` إلى `Option<T>`: `Ok(v)` يصبح `Some(v)` و`Err(e)` يصبح `None`
//! * `transpose` يقلب `Result` فيه `Option` إلى `Option` فيه `Result`
//!
//! هذه الدوال تحوّل القيمة الموجودة في حالة `Ok`:
//!
//! * `map` يحوّل `Result<T, E>` إلى `Result<U, E>` بتطبيق الدالة على القيمة داخل `Ok` ويترك `Err`
//!   كما هو
//! * `inspect` يأخذ ملكية الـ `Result` ويطبّق الدالة على القيمة بالمرجع ثم يرجع الـ `Result`
//!
//! هذه الدوال تحوّل القيمة الموجودة في حالة `Err`:
//!
//! * `map_err` يحوّل `Result<T, E>` إلى `Result<T, F>` بتطبيق الدالة على القيمة داخل `Err` ويترك
//!   `Ok` كما هو
//! * `inspect_err` يأخذ ملكية الـ `Result` ويطبّق الدالة على قيمة `Err` بالمرجع ثم يرجع الـ
//!   `Result`
//!
//! هذه الدوال تحوّل `Result<T, E>` إلى قيمة من نوع قد يكون مختلفًا `U`:
//!
//! * `map_or` يطبّق الدالة على القيمة داخل `Ok`، أو يرجع القيمة الافتراضية المعطاة إذا كان `Err`
//! * `map_or_else` يطبّق الدالة على القيمة داخل `Ok`، أو يطبّق الدالة البديلة على القيمة داخل `Err`
//!
//! ## العوامل المنطقية
//!
//! هذه الدوال تعامل الـ `Result` كقيمة منطقية: `Ok` مثل `true` و`Err` مثل `false`. هي نوعان: دوال
//! تأخذ `Result` كمدخل، ودوال تأخذ دالة كمدخل (تُنفَّذ عند الحاجة فقط).
//!
//! الدالتان `and` و`or` تأخذان `Result` آخر وترجعان `Result`. الدالة `and` يمكنها إرجاع
//! `Result<U, E>` بنوع داخلي `U` مختلف، والدالة `or` يمكنها إرجاع `Result<T, F>` بنوع خطأ `F`
//! مختلف.
//!
//! | الدالة | self     | المدخل    | الناتج   |
//! |--------|----------|-----------|----------|
//! | `and`  | `Err(e)` | (يُتجاهل) | `Err(e)` |
//! | `and`  | `Ok(x)`  | `Err(d)`  | `Err(d)` |
//! | `and`  | `Ok(x)`  | `Ok(y)`   | `Ok(y)`  |
//! | `or`   | `Err(e)` | `Err(d)`  | `Err(d)` |
//! | `or`   | `Err(e)` | `Ok(y)`   | `Ok(y)`  |
//! | `or`   | `Ok(x)`  | (يُتجاهل) | `Ok(x)`  |
//!
//! الدالتان `and_then` و`or_else` تأخذان دالة، ولا تنفذانها إلا عند الحاجة لإنتاج قيمة جديدة.
//! `and_then` يمكنها إرجاع `Result<U, E>` بنوع داخلي مختلف، و`or_else` يمكنها إرجاع `Result<T, F>`
//! بنوع خطأ مختلف.
//!
//! | الدالة     | self     | مدخل الدالة | ناتج الدالة  | الناتج   |
//! |------------|----------|-------------|--------------|----------|
//! | `and_then` | `Err(e)` | (لا يُعطى)  | (لا تُنفَّذ) | `Err(e)` |
//! | `and_then` | `Ok(x)`  | `x`         | `Err(d)`     | `Err(d)` |
//! | `and_then` | `Ok(x)`  | `x`         | `Ok(y)`      | `Ok(y)`  |
//! | `or_else`  | `Err(e)` | `e`         | `Err(d)`     | `Err(d)` |
//! | `or_else`  | `Err(e)` | `e`         | `Ok(y)`      | `Ok(y)`  |
//! | `or_else`  | `Ok(x)`  | (لا يُعطى)  | (لا تُنفَّذ) | `Ok(x)`  |
//!
//! ## عوامل المقارنة
//!
//! إذا كان `T` و`E` يطبّقان `PartialOrd` فإن `Result<T, E>` يشتق تطبيق `PartialOrd`. بهذا الترتيب
//! يكون `Ok` أصغر من أي `Err`، وتُقارن قيمتا `Ok` أو قيمتا `Err` كما تُقارن قيمهما في `T` أو `E`.
//! وإذا كان `T` و`E` يطبّقان `Ord` فكذلك `Result<T, E>`.
//!
//! ## التكرار على `Result`
//!
//! يمكن التكرار على `Result`. هذا مفيد إذا احتجت مكررًا قد يكون فارغًا حسب شرط. المكرر يعطي قيمة
//! واحدة (إذا كان `Ok`) أو لا يعطي شيئًا (إذا كان `Err`). مثلًا، `into_iter` يتصرف مثل `once(v)`
//! إذا كان `Ok(v)`، ومثل `empty()` إذا كان `Err`.
//!
//! المكررات على `Result<T, E>` ثلاثة أنواع:
//!
//! * `into_iter` يستهلك الـ `Result` ويعطي القيمة الموجودة
//! * `iter` يعطي مرجعًا غير قابل للتعديل `&T` إلى القيمة
//! * `iter_mut` يعطي مرجعًا قابلًا للتعديل `&mut T` إلى القيمة
//!
//! انظر قسم "التكرار على `Option`" لأمثلة على فائدة ذلك.
//!
//! قد تريد استخدام سلسلة مكررات لتنفيذ عملية قد تفشل عدة مرات، مع تجاهل حالات الفشل ومتابعة معالجة
//! النتائج الناجحة. في المثال نستفيد من أن `Result` قابل للتكرار لاختيار قيم `Ok` فقط باستخدام
//! `flatten`.
//!
//! ## الجمع في `Result`
//!
//! `Result` يطبّق الـ trait `FromIterator`، فيمكن جمع مكرر قيم `Result` في `Result` واحد فيه مجموعة
//! من كل القيم، أو `Err` إذا كان أي عنصر `Err`.
//!
//! `Result` يطبّق أيضًا `Product` و`Sum`، فيمكن استخدام `product` و`sum` على مكرر قيم `Result`.
//!
//! </div>

#![stable(feature = "rust1", since = "1.0.0")]

use crate::iter::{self, FusedIterator, TrustedLen};
use crate::marker::Destruct;
use crate::ops::{self, ControlFlow, Deref, DerefMut};
use crate::{fmt, hint};

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
///
/// See the [module documentation](self) for details.
///
/// <div lang="ar" dir="rtl">
///
/// `Result` نوع يمثّل إما النجاح (`Ok`) أو الفشل (`Err`).
///
/// انظر توثيق الوحدة للتفاصيل.
///
/// </div>
#[doc(search_unbox)]
#[derive(Copy, Debug, Hash)]
#[derive_const(PartialEq, PartialOrd, Eq, Ord)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
#[rustc_diagnostic_item = "Result"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Result<T, E> {
    /// Contains the success value
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحتوي قيمة النجاح.
    ///
    /// </div>
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحتوي قيمة الخطأ.
    ///
    /// </div>
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl<T, E> Result<T, E> {
    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the result is [`Ok`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<i32, &str> = Ok(-3);
    /// assert_eq!(x.is_ok(), true);
    ///
    /// let x: Result<i32, &str> = Err("Some error message");
    /// assert_eq!(x.is_ok(), false);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `true` إذا كانت النتيجة `Ok`.
    ///
    /// </div>
    #[must_use = "if you intended to assert that this is ok, consider `.unwrap()` instead"]
    #[rustc_const_stable(feature = "const_result_basics", since = "1.48.0")]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, Ok(_))
    }

    /// Returns `true` if the result is [`Ok`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.is_ok_and(|x| x > 1), true);
    ///
    /// let x: Result<u32, &str> = Ok(0);
    /// assert_eq!(x.is_ok_and(|x| x > 1), false);
    ///
    /// let x: Result<u32, &str> = Err("hey");
    /// assert_eq!(x.is_ok_and(|x| x > 1), false);
    ///
    /// let x: Result<String, &str> = Ok("ownership".to_string());
    /// assert_eq!(x.as_ref().is_ok_and(|x| x.len() > 1), true);
    /// println!("still alive {:?}", x);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `true` إذا كانت النتيجة `Ok` والقيمة بداخلها تحقق الشرط.
    ///
    /// </div>
    #[must_use]
    #[inline]
    #[stable(feature = "is_some_and", since = "1.70.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn is_ok_and<F>(self, f: F) -> bool
    where
        F: [const] FnOnce(T) -> bool + [const] Destruct,
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Err(_) => false,
            Ok(x) => f(x),
        }
    }

    /// Returns `true` if the result is [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<i32, &str> = Ok(-3);
    /// assert_eq!(x.is_err(), false);
    ///
    /// let x: Result<i32, &str> = Err("Some error message");
    /// assert_eq!(x.is_err(), true);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `true` إذا كانت النتيجة `Err`.
    ///
    /// </div>
    #[must_use = "if you intended to assert that this is err, consider `.unwrap_err()` instead"]
    #[rustc_const_stable(feature = "const_result_basics", since = "1.48.0")]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Returns `true` if the result is [`Err`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Error, ErrorKind};
    ///
    /// let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);
    ///
    /// let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
    ///
    /// let x: Result<u32, Error> = Ok(123);
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
    ///
    /// let x: Result<u32, String> = Err("ownership".to_string());
    /// assert_eq!(x.as_ref().is_err_and(|x| x.len() > 1), true);
    /// println!("still alive {:?}", x);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `true` إذا كانت النتيجة `Err` والقيمة بداخلها تحقق الشرط.
    ///
    /// </div>
    #[must_use]
    #[inline]
    #[stable(feature = "is_some_and", since = "1.70.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn is_err_and<F>(self, f: F) -> bool
    where
        F: [const] FnOnce(E) -> bool + [const] Destruct,
        E: [const] Destruct,
        T: [const] Destruct,
    {
        match self {
            Ok(_) => false,
            Err(e) => f(e),
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for each variant
    /////////////////////////////////////////////////////////////////////////

    /// Converts from `Result<T, E>` to [`Option<T>`].
    ///
    /// Converts `self` into an [`Option<T>`], consuming `self`,
    /// and converting the error to `None`, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.ok(), Some(2));
    ///
    /// let x: Result<u32, &str> = Err("Nothing here");
    /// assert_eq!(x.ok(), None);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `Result<T, E>` إلى `Option<T>`.
    ///
    /// يحوّل `self` إلى `Option<T>`، ويستهلك `self`، ويحوّل الخطأ (إن وُجد) إلى `None`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    #[rustc_diagnostic_item = "result_ok_method"]
    pub const fn ok(self) -> Option<T>
    where
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    /// Converts from `Result<T, E>` to [`Option<E>`].
    ///
    /// Converts `self` into an [`Option<E>`], consuming `self`,
    /// and discarding the success value, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.err(), None);
    ///
    /// let x: Result<u32, &str> = Err("Nothing here");
    /// assert_eq!(x.err(), Some("Nothing here"));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `Result<T, E>` إلى `Option<E>`.
    ///
    /// يحوّل `self` إلى `Option<E>`، ويستهلك `self`، ويتجاهل قيمة النجاح (إن وُجدت).
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn err(self) -> Option<E>
    where
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Ok(_) => None,
            Err(x) => Some(x),
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////

    /// Converts from `&Result<T, E>` to `Result<&T, &E>`.
    ///
    /// Produces a new `Result`, containing a reference
    /// into the original, leaving the original in place.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.as_ref(), Ok(&2));
    ///
    /// let x: Result<u32, &str> = Err("Error");
    /// assert_eq!(x.as_ref(), Err(&"Error"));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `&Result<T, E>` إلى `Result<&T, &E>`.
    ///
    /// ينتج `Result` جديدًا فيه مرجع إلى الأصلي، ويترك الأصلي في مكانه.
    ///
    /// </div>
    #[inline]
    #[rustc_const_stable(feature = "const_result_basics", since = "1.48.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub const fn as_ref(&self) -> Result<&T, &E> {
        match *self {
            Ok(ref x) => Ok(x),
            Err(ref x) => Err(x),
        }
    }

    /// Converts from `&mut Result<T, E>` to `Result<&mut T, &mut E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// fn mutate(r: &mut Result<i32, i32>) {
    ///     match r.as_mut() {
    ///         Ok(v) => *v = 42,
    ///         Err(e) => *e = 0,
    ///     }
    /// }
    ///
    /// let mut x: Result<i32, i32> = Ok(2);
    /// mutate(&mut x);
    /// assert_eq!(x.unwrap(), 42);
    ///
    /// let mut x: Result<i32, i32> = Err(13);
    /// mutate(&mut x);
    /// assert_eq!(x.unwrap_err(), 0);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `&mut Result<T, E>` إلى `Result<&mut T, &mut E>`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_result", since = "1.83.0")]
    pub const fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match *self {
            Ok(ref mut x) => Ok(x),
            Err(ref mut x) => Err(x),
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    /// Maps a `Result<T, E>` to `Result<U, E>` by applying a function to a
    /// contained [`Ok`] value, leaving an [`Err`] value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    ///
    /// # Examples
    ///
    /// Print the numbers on each line of a string multiplied by two.
    ///
    /// ```
    /// let line = "1\n2\n3\n4\n";
    ///
    /// for num in line.lines() {
    ///     match num.parse::<i32>().map(|i| i * 2) {
    ///         Ok(n) => println!("{n}"),
    ///         Err(..) => {}
    ///     }
    /// }
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<T, E>` إلى `Result<U, E>` بتطبيق دالة على القيمة داخل `Ok`، ويترك `Err` كما
    /// هو.
    ///
    /// يمكن استخدام هذه الدالة لتركيب نتائج دالتين.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: [const] FnOnce(T) -> U + [const] Destruct,
    {
        match self {
            Ok(t) => Ok(op(t)),
            Err(e) => Err(e),
        }
    }

    /// Returns the provided default (if [`Err`]), or
    /// applies a function to the contained value (if [`Ok`]).
    ///
    /// The `default` argument passed to `map_or` is eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which lazily evaluates `default`.
    ///
    /// [`map_or_else`]: Result::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة الافتراضية المعطاة (إذا كان `Err`)، أو يطبّق دالة على القيمة الموجودة (إذا كان
    /// `Ok`).
    ///
    /// المعامل `default` المعطى لـ `map_or` يُحسب فورًا؛ إذا كنت تمرر ناتج استدعاء دالة، يُنصح
    /// باستخدام
    /// `map_or_else` التي تحسب `default` عند الحاجة فقط.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_map_or", since = "1.41.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    #[must_use = "if you don't need the returned value, use `if let` instead"]
    pub const fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: [const] FnOnce(T) -> U + [const] Destruct,
        T: [const] Destruct,
        E: [const] Destruct,
        U: [const] Destruct,
    {
        match self {
            Ok(t) => f(t),
            Err(_) => default,
        }
    }

    /// Maps a `Result<T, E>` to `U` by applying fallback function `default` to
    /// a contained [`Err`] value, or function `f` to a contained [`Ok`] value.
    ///
    /// This function can be used to unpack a successful result
    /// while handling an error.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let k = 21;
    ///
    /// let x : Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
    ///
    /// let x : Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<T, E>` إلى `U` بتطبيق الدالة البديلة `default` على قيمة `Err`، أو الدالة `f`
    /// على
    /// قيمة `Ok`.
    ///
    /// يمكن استخدام هذه الدالة لفك نتيجة ناجحة مع معالجة الخطأ.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_map_or_else", since = "1.41.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: [const] FnOnce(E) -> U + [const] Destruct,
        F: [const] FnOnce(T) -> U + [const] Destruct,
    {
        match self {
            Ok(t) => f(t),
            Err(e) => default(e),
        }
    }

    /// Maps a `Result<T, E>` to a `U` by applying function `f` to the contained
    /// value if the result is [`Ok`], otherwise if [`Err`], returns the
    /// [default value] for the type `U`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<_, &str> = Ok("foo");
    /// let y: Result<&str, _> = Err("bar");
    ///
    /// assert_eq!(x.map_or_default(|x| x.len()), 3);
    /// assert_eq!(y.map_or_default(|y| y.len()), 0);
    /// ```
    ///
    /// [default value]: Default::default
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<T, E>` إلى `U` بتطبيق الدالة `f` على القيمة إذا كان `Ok`، وإلا إذا كان `Err`
    /// يرجع
    /// القيمة الافتراضية للنوع `U`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_option_map_or_default", since = "1.98.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn map_or_default<U, F>(self, f: F) -> U
    where
        F: [const] FnOnce(T) -> U + [const] Destruct,
        U: [const] Default,
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Ok(t) => f(t),
            Err(_) => U::default(),
        }
    }

    /// Maps a `Result<T, E>` to `Result<T, F>` by applying a function to a
    /// contained [`Err`] value, leaving an [`Ok`] value untouched.
    ///
    /// This function can be used to pass through a successful result while handling
    /// an error.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// fn stringify(x: u32) -> String { format!("error code: {x}") }
    ///
    /// let x: Result<u32, u32> = Ok(2);
    /// assert_eq!(x.map_err(stringify), Ok(2));
    ///
    /// let x: Result<u32, u32> = Err(13);
    /// assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<T, E>` إلى `Result<T, F>` بتطبيق دالة على القيمة داخل `Err`، ويترك `Ok` كما
    /// هو.
    ///
    /// يمكن استخدام هذه الدالة لتمرير نتيجة ناجحة مع معالجة الخطأ.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: [const] FnOnce(E) -> F + [const] Destruct,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(op(e)),
        }
    }

    /// Calls a function with a reference to the contained value if [`Ok`].
    ///
    /// Returns the original result.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: u8 = "4"
    ///     .parse::<u8>()
    ///     .inspect(|x| println!("original: {x}"))
    ///     .map(|x| x.pow(3))
    ///     .expect("literal `4` should parse as a `u8`");
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يستدعي دالة بمرجع إلى القيمة الموجودة إذا كان `Ok`.
    ///
    /// يرجع النتيجة الأصلية.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_option_inspect", since = "1.76.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn inspect<F>(self, f: F) -> Self
    where
        F: [const] FnOnce(&T) + [const] Destruct,
    {
        if let Ok(ref t) = self {
            f(t);
        }

        self
    }

    /// Calls a function with a reference to the contained value if [`Err`].
    ///
    /// Returns the original result.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{fs, io};
    ///
    /// fn read() -> io::Result<String> {
    ///     fs::read_to_string("address.txt")
    ///         .inspect_err(|e| eprintln!("failed to read file: {e}"))
    /// }
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يستدعي دالة بمرجع إلى القيمة الموجودة إذا كان `Err`.
    ///
    /// يرجع النتيجة الأصلية.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_option_inspect", since = "1.76.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn inspect_err<F>(self, f: F) -> Self
    where
        F: [const] FnOnce(&E) + [const] Destruct,
    {
        if let Err(ref e) = self {
            f(e);
        }

        self
    }

    /// Converts from `Result<T, E>` (or `&Result<T, E>`) to `Result<&<T as Deref>::Target, &E>`.
    ///
    /// Coerces the [`Ok`] variant of the original [`Result`] via [`Deref`](crate::ops::Deref)
    /// and returns the new [`Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<String, u32> = Ok("hello".to_string());
    /// let y: Result<&str, &u32> = Ok("hello");
    /// assert_eq!(x.as_deref(), y);
    ///
    /// let x: Result<String, u32> = Err(42);
    /// let y: Result<&str, &u32> = Err(&42);
    /// assert_eq!(x.as_deref(), y);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `Result<T, E>` (أو `&Result<T, E>`) إلى `Result<&<T as Deref>::Target, &E>`.
    ///
    /// يحوّل حالة `Ok` في الـ `Result` الأصلي عبر `Deref` ويرجع `Result` جديدًا.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "inner_deref", since = "1.47.0")]
    #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
    pub const fn as_deref(&self) -> Result<&T::Target, &E>
    where
        T: [const] Deref,
    {
        self.as_ref().map(Deref::deref)
    }

    /// Converts from `Result<T, E>` (or `&mut Result<T, E>`) to `Result<&mut <T as DerefMut>::Target, &mut E>`.
    ///
    /// Coerces the [`Ok`] variant of the original [`Result`] via [`DerefMut`](crate::ops::DerefMut)
    /// and returns the new [`Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = "HELLO".to_string();
    /// let mut x: Result<String, u32> = Ok("hello".to_string());
    /// let y: Result<&mut str, &mut u32> = Ok(&mut s);
    /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
    ///
    /// let mut i = 42;
    /// let mut x: Result<String, u32> = Err(42);
    /// let y: Result<&mut str, &mut u32> = Err(&mut i);
    /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `Result<T, E>` (أو `&mut Result<T, E>`) إلى
    /// `Result<&mut <T as DerefMut>::Target, &mut E>`.
    ///
    /// يحوّل حالة `Ok` في الـ `Result` الأصلي عبر `DerefMut` ويرجع `Result` جديدًا.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "inner_deref", since = "1.47.0")]
    #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
    pub const fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E>
    where
        T: [const] DerefMut,
    {
        self.as_mut().map(DerefMut::deref_mut)
    }

    /////////////////////////////////////////////////////////////////////////
    // Iterator constructors
    /////////////////////////////////////////////////////////////////////////

    /// Returns an iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is [`Result::Ok`], otherwise none.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(7);
    /// assert_eq!(x.iter().next(), Some(&7));
    ///
    /// let x: Result<u32, &str> = Err("nothing!");
    /// assert_eq!(x.iter().next(), None);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع مكررًا على القيمة الموجودة (إن وُجدت).
    ///
    /// المكرر يعطي قيمة واحدة إذا كانت النتيجة `Result::Ok`، وإلا لا شيء.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter { inner: self.as_ref().ok() }
    }

    /// Returns a mutable iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is [`Result::Ok`], otherwise none.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x: Result<u32, &str> = Ok(7);
    /// match x.iter_mut().next() {
    ///     Some(v) => *v = 40,
    ///     None => {},
    /// }
    /// assert_eq!(x, Ok(40));
    ///
    /// let mut x: Result<u32, &str> = Err("nothing!");
    /// assert_eq!(x.iter_mut().next(), None);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع مكررًا قابلًا للتعديل على القيمة الموجودة (إن وُجدت).
    ///
    /// المكرر يعطي قيمة واحدة إذا كانت النتيجة `Result::Ok`، وإلا لا شيء.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { inner: self.as_mut().ok() }
    }

    /////////////////////////////////////////////////////////////////////////
    // Extract a value
    /////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Result::unwrap_or
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    /// [`unwrap_or_default`]: Result::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message including the
    /// passed message, and the content of the [`Err`].
    ///
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.expect("Testing expect"); // panics with `Testing expect: emergency failure`
    /// ```
    ///
    /// # Recommended Message Style
    ///
    /// We recommend that `expect` messages are used to describe the reason you
    /// _expect_ the `Result` should be `Ok`.
    ///
    /// ```should_panic
    /// let path = std::env::var("IMPORTANT_PATH")
    ///     .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    /// ```
    ///
    /// **Hint**: If you're having trouble remembering how to phrase expect
    /// error messages remember to focus on the word "should" as in "env
    /// variable should be set by blah" or "the given binary should be available
    /// and executable by the current user".
    ///
    /// For more detail on expect message styles and the reasoning behind our recommendation please
    /// refer to the section on ["Common Message
    /// Styles"](../../std/error/index.html#common-message-styles) in the
    /// [`std::error`](../../std/error/index.html) module docs.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok`، ويستهلك `self`.
    ///
    /// لأن هذه الدالة قد تسبب panic، لا يُنصح باستخدامها عمومًا. بدلًا من ذلك، استخدم مطابقة
    /// الأنماط
    /// وتعامل مع حالة `Err` صراحة، أو استدعِ `unwrap_or` أو `unwrap_or_else` أو
    /// `unwrap_or_default`.
    ///
    /// # حالات الـ panic
    ///
    /// يسبب panic إذا كانت القيمة `Err`، برسالة فيها الرسالة المعطاة ومحتوى الـ `Err`.
    ///
    /// # أسلوب الرسالة المستحسن
    ///
    /// نوصي بأن تصف رسائل `expect` السبب الذي *تتوقع* لأجله أن يكون الـ `Result` هو `Ok`.
    ///
    /// **تلميح**: إذا صعب عليك صياغة رسائل `expect`، ركّز على كلمة "يجب" مثل "متغير البيئة يجب أن
    /// يُضبط بواسطة كذا" أو "الملف التنفيذي يجب أن يكون متاحًا وقابلًا للتنفيذ للمستخدم الحالي".
    ///
    /// لمزيد من التفاصيل عن أساليب رسائل `expect` وسبب توصيتنا، راجع قسم "Common Message Styles" في
    /// توثيق وحدة `std::error`.
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "result_expect", since = "1.4.0")]
    pub fn expect(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed(msg, e),
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Panics are meant for unrecoverable errors, and
    /// [may abort the entire program][panic-abort].
    ///
    /// Instead, prefer to use [the `?` (try) operator][try-operator], or pattern matching
    /// to handle the [`Err`] case explicitly, or call [`unwrap_or`],
    /// [`unwrap_or_else`], or [`unwrap_or_default`].
    ///
    /// [panic-abort]: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
    /// [try-operator]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    /// [`unwrap_or`]: Result::unwrap_or
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    /// [`unwrap_or_default`]: Result::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.unwrap(), 2);
    /// ```
    ///
    /// ```should_panic
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.unwrap(); // panics with `emergency failure`
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok`، ويستهلك `self`.
    ///
    /// لأن هذه الدالة قد تسبب panic، لا يُنصح باستخدامها عمومًا. الـ panic مخصص للأخطاء التي لا
    /// يمكن
    /// التعافي منها، وقد يوقف البرنامج كله.
    ///
    /// بدلًا من ذلك، استخدم العامل `?` أو مطابقة الأنماط للتعامل مع حالة `Err` صراحة، أو استدعِ
    /// `unwrap_or` أو `unwrap_or_else` أو `unwrap_or_default`.
    ///
    /// # حالات الـ panic
    ///
    /// يسبب panic إذا كانت القيمة `Err`، برسالة panic مأخوذة من قيمة الـ `Err`.
    ///
    /// </div>
    #[inline(always)]
    #[track_caller]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", e),
        }
    }

    /// Returns the contained [`Ok`] value or a default
    ///
    /// Consumes the `self` argument then, if [`Ok`], returns the contained
    /// value, otherwise if [`Err`], returns the default value for that
    /// type.
    ///
    /// # Examples
    ///
    /// Converts a string to an integer, turning poorly-formed strings
    /// into 0 (the default value for integers). [`parse`] converts
    /// a string to any other type that implements [`FromStr`], returning an
    /// [`Err`] on error.
    ///
    /// ```
    /// let good_year_from_input = "1909";
    /// let bad_year_from_input = "190blarg";
    /// let good_year = good_year_from_input.parse().unwrap_or_default();
    /// let bad_year = bad_year_from_input.parse().unwrap_or_default();
    ///
    /// assert_eq!(1909, good_year);
    /// assert_eq!(0, bad_year);
    /// ```
    ///
    /// [`parse`]: str::parse
    /// [`FromStr`]: crate::str::FromStr
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok` أو القيمة الافتراضية.
    ///
    /// يستهلك `self`، ثم إذا كان `Ok` يرجع القيمة الموجودة، وإذا كان `Err` يرجع القيمة الافتراضية
    /// لذلك
    /// النوع.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_unwrap_or_default", since = "1.16.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn unwrap_or_default(self) -> T
    where
        T: [const] Default + [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Ok(x) => x,
            Err(_) => Default::default(),
        }
    }

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a panic message including the
    /// passed message, and the content of the [`Ok`].
    ///
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// let x: Result<u32, &str> = Ok(10);
    /// x.expect_err("Testing expect_err"); // panics with `Testing expect_err: 10`
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Err`، ويستهلك `self`.
    ///
    /// # حالات الـ panic
    ///
    /// يسبب panic إذا كانت القيمة `Ok`، برسالة فيها الرسالة المعطاة ومحتوى الـ `Ok`.
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "result_expect_err", since = "1.17.0")]
    pub fn expect_err(self, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => unwrap_failed(msg, t),
            Err(e) => e,
        }
    }

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a custom panic message provided
    /// by the [`Ok`]'s value.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// let x: Result<u32, &str> = Ok(2);
    /// x.unwrap_err(); // panics with `2`
    /// ```
    ///
    /// ```
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// assert_eq!(x.unwrap_err(), "emergency failure");
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Err`، ويستهلك `self`.
    ///
    /// # حالات الـ panic
    ///
    /// يسبب panic إذا كانت القيمة `Ok`، برسالة panic مأخوذة من قيمة الـ `Ok`.
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn unwrap_err(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => unwrap_failed("called `Result::unwrap_err()` on an `Ok` value", t),
            Err(e) => e,
        }
    }

    /// Returns the contained [`Ok`] value, but never panics.
    ///
    /// Unlike [`unwrap`], this method is known to never panic on the
    /// result types it is implemented for. Therefore, it can be used
    /// instead of `unwrap` as a maintainability safeguard that will fail
    /// to compile if the error type of the `Result` is later changed
    /// to an error that can actually occur.
    ///
    /// [`unwrap`]: Result::unwrap
    ///
    /// # Examples
    ///
    /// ```
    /// fn only_good_news() -> Result<String, !> {
    ///     Ok("this is fine".into())
    /// }
    ///
    /// let s: String = only_good_news().into_ok();
    /// println!("{s}");
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok`، ولا يسبب panic أبدًا.
    ///
    /// بخلاف `unwrap`، هذه الدالة معروف أنها لا تسبب panic أبدًا على أنواع النتائج المطبقة لها.
    /// لذلك
    /// يمكن استخدامها بدل `unwrap` كضمانة للصيانة: سيفشل الكود في الترجمة إذا تغيّر نوع الخطأ في
    /// الـ `Result` لاحقًا إلى خطأ يمكن أن يحدث فعلًا.
    ///
    /// </div>
    #[stable(feature = "unwrap_infallible", since = "CURRENT_RUSTC_VERSION")]
    #[inline]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
    pub const fn into_ok(self) -> T
    where
        E: [const] Into<!>,
    {
        match self {
            Ok(x) => x,
            Err(e) => e.into(),
        }
    }

    /// Returns the contained [`Err`] value, but never panics.
    ///
    /// Unlike [`unwrap_err`], this method is known to never panic on the
    /// result types it is implemented for. Therefore, it can be used
    /// instead of `unwrap_err` as a maintainability safeguard that will fail
    /// to compile if the ok type of the `Result` is later changed
    /// to a type that can actually occur.
    ///
    /// [`unwrap_err`]: Result::unwrap_err
    ///
    /// # Examples
    ///
    /// ```
    /// fn only_bad_news() -> Result<!, String> {
    ///     Err("Oops, it failed".into())
    /// }
    ///
    /// let error: String = only_bad_news().into_err();
    /// println!("{error}");
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Err`، ولا يسبب panic أبدًا.
    ///
    /// بخلاف `unwrap_err`، هذه الدالة معروف أنها لا تسبب panic أبدًا على أنواع النتائج المطبقة لها.
    /// لذلك يمكن استخدامها بدل `unwrap_err` كضمانة للصيانة: سيفشل الكود في الترجمة إذا تغيّر نوع
    /// النجاح
    /// في الـ `Result` لاحقًا إلى نوع يمكن أن يحدث فعلًا.
    ///
    /// </div>
    #[stable(feature = "unwrap_infallible", since = "CURRENT_RUSTC_VERSION")]
    #[inline]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    #[rustc_const_unstable(feature = "const_convert", issue = "143773")]
    pub const fn into_err(self) -> E
    where
        T: [const] Into<!>,
    {
        match self {
            Ok(x) => x.into(),
            Err(e) => e,
        }
    }

    ////////////////////////////////////////////////////////////////////////
    // Boolean operations on the values, eager and lazy
    /////////////////////////////////////////////////////////////////////////

    /// Returns `res` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    ///
    /// Arguments passed to `and` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`and_then`], which is
    /// lazily evaluated.
    ///
    /// [`and_then`]: Result::and_then
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// let y: Result<&str, &str> = Err("late error");
    /// assert_eq!(x.and(y), Err("late error"));
    ///
    /// let x: Result<u32, &str> = Err("early error");
    /// let y: Result<&str, &str> = Ok("foo");
    /// assert_eq!(x.and(y), Err("early error"));
    ///
    /// let x: Result<u32, &str> = Err("not a 2");
    /// let y: Result<&str, &str> = Err("late error");
    /// assert_eq!(x.and(y), Err("not a 2"));
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// let y: Result<&str, &str> = Ok("different result type");
    /// assert_eq!(x.and(y), Ok("different result type"));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `res` إذا كانت النتيجة `Ok`، وإلا يرجع قيمة `Err` الخاصة بـ `self`.
    ///
    /// المعاملات المعطاة لـ `and` تُحسب فورًا؛ إذا كنت تمرر ناتج استدعاء دالة، يُنصح باستخدام
    /// `and_then`
    /// التي تحسب عند الحاجة فقط.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn and<U>(self, res: Result<U, E>) -> Result<U, E>
    where
        T: [const] Destruct,
        E: [const] Destruct,
        U: [const] Destruct,
    {
        match self {
            Ok(_) => res,
            Err(e) => Err(e),
        }
    }

    /// Calls `op` if the result is [`Ok`], otherwise returns the [`Err`] value of `self`.
    ///
    ///
    /// This function can be used for control flow based on `Result` values.
    ///
    /// # Examples
    ///
    /// ```
    /// fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
    ///     x.checked_mul(x).map(|sq| sq.to_string()).ok_or("overflowed")
    /// }
    ///
    /// assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    /// assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    /// assert_eq!(Err("not a number").and_then(sq_then_to_string), Err("not a number"));
    /// ```
    ///
    /// Often used to chain fallible operations that may return [`Err`].
    ///
    /// ```
    /// use std::{io::ErrorKind, path::Path};
    ///
    /// // Note: on Windows "/" maps to "C:\"
    /// let root_modified_time = Path::new("/").metadata().and_then(|md| md.modified());
    /// assert!(root_modified_time.is_ok());
    ///
    /// let should_fail = Path::new("/bad/path").metadata().and_then(|md| md.modified());
    /// assert!(should_fail.is_err());
    /// assert_eq!(should_fail.unwrap_err().kind(), ErrorKind::NotFound);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يستدعي `op` إذا كانت النتيجة `Ok`، وإلا يرجع قيمة `Err` الخاصة بـ `self`.
    ///
    /// يمكن استخدام هذه الدالة للتحكم في سير البرنامج حسب قيم `Result`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    #[rustc_confusables("flat_map", "flatmap")]
    pub const fn and_then<U, F>(self, op: F) -> Result<U, E>
    where
        F: [const] FnOnce(T) -> Result<U, E> + [const] Destruct,
    {
        match self {
            Ok(t) => op(t),
            Err(e) => Err(e),
        }
    }

    /// Returns `res` if the result is [`Err`], otherwise returns the [`Ok`] value of `self`.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`or_else`], which is
    /// lazily evaluated.
    ///
    /// [`or_else`]: Result::or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// let y: Result<u32, &str> = Err("late error");
    /// assert_eq!(x.or(y), Ok(2));
    ///
    /// let x: Result<u32, &str> = Err("early error");
    /// let y: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.or(y), Ok(2));
    ///
    /// let x: Result<u32, &str> = Err("not a 2");
    /// let y: Result<u32, &str> = Err("late error");
    /// assert_eq!(x.or(y), Err("late error"));
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// let y: Result<u32, &str> = Ok(100);
    /// assert_eq!(x.or(y), Ok(2));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع `res` إذا كانت النتيجة `Err`، وإلا يرجع قيمة `Ok` الخاصة بـ `self`.
    ///
    /// المعاملات المعطاة لـ `or` تُحسب فورًا؛ إذا كنت تمرر ناتج استدعاء دالة، يُنصح باستخدام
    /// `or_else`
    /// التي تحسب عند الحاجة فقط.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn or<F>(self, res: Result<T, F>) -> Result<T, F>
    where
        T: [const] Destruct,
        E: [const] Destruct,
        F: [const] Destruct,
    {
        match self {
            Ok(v) => Ok(v),
            Err(_) => res,
        }
    }

    /// Calls `op` if the result is [`Err`], otherwise returns the [`Ok`] value of `self`.
    ///
    /// This function can be used for control flow based on result values.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
    /// fn err(x: u32) -> Result<u32, u32> { Err(x) }
    ///
    /// assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    /// assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    /// assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    /// assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يستدعي `op` إذا كانت النتيجة `Err`، وإلا يرجع قيمة `Ok` الخاصة بـ `self`.
    ///
    /// يمكن استخدام هذه الدالة للتحكم في سير البرنامج حسب قيم النتائج.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn or_else<F, O>(self, op: O) -> Result<T, F>
    where
        O: [const] FnOnce(E) -> Result<T, F> + [const] Destruct,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => op(e),
        }
    }

    /// Returns the contained [`Ok`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let default = 2;
    /// let x: Result<u32, &str> = Ok(9);
    /// assert_eq!(x.unwrap_or(default), 9);
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.unwrap_or(default), default);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok` أو القيمة الافتراضية المعطاة.
    ///
    /// المعاملات المعطاة لـ `unwrap_or` تُحسب فورًا؛ إذا كنت تمرر ناتج استدعاء دالة، يُنصح باستخدام
    /// `unwrap_or_else` التي تحسب عند الحاجة فقط.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn unwrap_or(self, default: T) -> T
    where
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            Ok(t) => t,
            Err(_) => default,
        }
    }

    /// Returns the contained [`Ok`] value or computes it from a closure.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// fn count(x: &str) -> usize { x.len() }
    ///
    /// assert_eq!(Ok(2).unwrap_or_else(count), 2);
    /// assert_eq!(Err("foo").unwrap_or_else(count), 3);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok` أو يحسبها من دالة مغلقة (closure).
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_result_trait_fn", issue = "144211")]
    pub const fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: [const] FnOnce(E) -> T + [const] Destruct,
    {
        match self {
            Ok(t) => t,
            Err(e) => op(e),
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value,
    /// without checking that the value is not an [`Err`].
    ///
    /// # Safety
    ///
    /// Calling this method on an [`Err`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(unsafe { x.unwrap_unchecked() }, 2);
    /// ```
    ///
    /// ```no_run
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// unsafe { x.unwrap_unchecked() }; // Undefined behavior!
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Ok` ويستهلك `self`، بدون التحقق من أن القيمة ليست `Err`.
    ///
    /// # الأمان
    ///
    /// استدعاء هذه الدالة على `Err` *سلوك غير معرّف*.
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "option_result_unwrap_unchecked", since = "1.58.0")]
    #[rustc_const_unstable(feature = "const_result_unwrap_unchecked", issue = "148714")]
    pub const unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                // FIXME(const-hack): to avoid E: const Destruct bound
                super::mem::forget(e);
                // SAFETY: the safety contract must be upheld by the caller.
                unsafe { hint::unreachable_unchecked() }
            }
        }
    }

    /// Returns the contained [`Err`] value, consuming the `self` value,
    /// without checking that the value is not an [`Ok`].
    ///
    /// # Safety
    ///
    /// Calling this method on an [`Ok`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let x: Result<u32, &str> = Ok(2);
    /// unsafe { x.unwrap_err_unchecked() }; // Undefined behavior!
    /// ```
    ///
    /// ```
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع القيمة داخل `Err` ويستهلك `self`، بدون التحقق من أن القيمة ليست `Ok`.
    ///
    /// # الأمان
    ///
    /// استدعاء هذه الدالة على `Ok` *سلوك غير معرّف*.
    ///
    /// </div>
    #[inline]
    #[track_caller]
    #[stable(feature = "option_result_unwrap_unchecked", since = "1.58.0")]
    #[rustc_const_unstable(feature = "const_result_unwrap_unchecked", issue = "148714")]
    pub const unsafe fn unwrap_err_unchecked(self) -> E
    where
        T: [const] Destruct,
        E: [const] Destruct,
    {
        match self {
            // SAFETY: the safety contract must be upheld by the caller.
            Ok(_) => unsafe { hint::unreachable_unchecked() },
            Err(e) => e,
        }
    }
}

impl<T, E> Result<&T, E> {
    /// Maps a `Result<&T, E>` to a `Result<T, E>` by copying the contents of the
    /// `Ok` part.
    ///
    /// # Examples
    ///
    /// ```
    /// let val = 12;
    /// let x: Result<&i32, i32> = Ok(&val);
    /// assert_eq!(x, Ok(&12));
    /// let copied = x.copied();
    /// assert_eq!(copied, Ok(12));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<&T, E>` إلى `Result<T, E>` بنسخ (copy) محتوى جزء `Ok`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_copied", since = "1.59.0")]
    #[rustc_const_stable(feature = "const_result", since = "1.83.0")]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    pub const fn copied(self) -> Result<T, E>
    where
        T: Copy,
    {
        // FIXME(const-hack): this implementation, which sidesteps using `Result::map` since it's not const
        // ready yet, should be reverted when possible to avoid code repetition
        match self {
            Ok(&v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Maps a `Result<&T, E>` to a `Result<T, E>` by cloning the contents of the
    /// `Ok` part.
    ///
    /// # Examples
    ///
    /// ```
    /// let val = 12;
    /// let x: Result<&i32, i32> = Ok(&val);
    /// assert_eq!(x, Ok(&12));
    /// let cloned = x.cloned();
    /// assert_eq!(cloned, Ok(12));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<&T, E>` إلى `Result<T, E>` باستنساخ (clone) محتوى جزء `Ok`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_cloned", since = "1.59.0")]
    #[expect(clippy::map_clone, reason = "implements Result::cloned")]
    pub fn cloned(self) -> Result<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> Result<&mut T, E> {
    /// Maps a `Result<&mut T, E>` to a `Result<T, E>` by copying the contents of the
    /// `Ok` part.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut val = 12;
    /// let x: Result<&mut i32, i32> = Ok(&mut val);
    /// assert_eq!(x, Ok(&mut 12));
    /// let copied = x.copied();
    /// assert_eq!(copied, Ok(12));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<&mut T, E>` إلى `Result<T, E>` بنسخ (copy) محتوى جزء `Ok`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_copied", since = "1.59.0")]
    #[rustc_const_stable(feature = "const_result", since = "1.83.0")]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    pub const fn copied(self) -> Result<T, E>
    where
        T: Copy,
    {
        // FIXME(const-hack): this implementation, which sidesteps using `Result::map` since it's not const
        // ready yet, should be reverted when possible to avoid code repetition
        match self {
            Ok(&mut v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Maps a `Result<&mut T, E>` to a `Result<T, E>` by cloning the contents of the
    /// `Ok` part.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut val = 12;
    /// let x: Result<&mut i32, i32> = Ok(&mut val);
    /// assert_eq!(x, Ok(&mut 12));
    /// let cloned = x.cloned();
    /// assert_eq!(cloned, Ok(12));
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل `Result<&mut T, E>` إلى `Result<T, E>` باستنساخ (clone) محتوى جزء `Ok`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_cloned", since = "1.59.0")]
    pub fn cloned(self) -> Result<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> Result<Option<T>, E> {
    /// Transposes a `Result` of an `Option` into an `Option` of a `Result`.
    ///
    /// `Ok(None)` will be mapped to `None`.
    /// `Ok(Some(_))` and `Err(_)` will be mapped to `Some(Ok(_))` and `Some(Err(_))`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct SomeErr;
    ///
    /// let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
    /// let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
    /// assert_eq!(x.transpose(), y);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يقلب `Result` فيه `Option` إلى `Option` فيه `Result`.
    ///
    /// `Ok(None)` يصبح `None`. و`Ok(Some(_))` و`Err(_)` يصبحان `Some(Ok(_))` و`Some(Err(_))`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "transpose_result", since = "1.33.0")]
    #[rustc_const_stable(feature = "const_result", since = "1.83.0")]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    pub const fn transpose(self) -> Option<Result<T, E>> {
        match self {
            Ok(Some(x)) => Some(Ok(x)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<T, E> Result<Result<T, E>, E> {
    /// Converts from `Result<Result<T, E>, E>` to `Result<T, E>`
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<Result<&'static str, u32>, u32> = Ok(Ok("hello"));
    /// assert_eq!(Ok("hello"), x.flatten());
    ///
    /// let x: Result<Result<&'static str, u32>, u32> = Ok(Err(6));
    /// assert_eq!(Err(6), x.flatten());
    ///
    /// let x: Result<Result<&'static str, u32>, u32> = Err(6);
    /// assert_eq!(Err(6), x.flatten());
    /// ```
    ///
    /// Flattening only removes one level of nesting at a time:
    ///
    /// ```
    /// let x: Result<Result<Result<&'static str, u32>, u32>, u32> = Ok(Ok(Ok("hello")));
    /// assert_eq!(Ok(Ok("hello")), x.flatten());
    /// assert_eq!(Ok("hello"), x.flatten().flatten());
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يحوّل من `Result<Result<T, E>, E>` إلى `Result<T, E>`.
    ///
    /// </div>
    #[inline]
    #[stable(feature = "result_flattening", since = "1.89.0")]
    #[rustc_allow_const_fn_unstable(const_precise_live_drops)]
    #[rustc_const_stable(feature = "result_flattening", since = "1.89.0")]
    pub const fn flatten(self) -> Result<T, E> {
        // FIXME(const-hack): could be written with `and_then`
        match self {
            Ok(inner) => inner,
            Err(e) => Err(e),
        }
    }
}

// This is a separate function to reduce the code size of the methods
//
// Take `error` by value so that `unwrap` doesn't need to include a landing pad to drop `error`
// during unwinding. This reduces the size of `unwrap` and allows it to be MIR inlined.
#[cfg(not(panic = "immediate-abort"))]
#[rustc_no_mir_inline]
#[cold]
#[track_caller]
fn unwrap_failed<T: fmt::Debug>(msg: &str, error: T) -> ! {
    // Do the actual formatting with a `&dyn Debug` to reduce code size.
    #[inline(never)]
    #[cold]
    #[track_caller]
    fn unwrap_failed_inner(msg: &str, error: &dyn fmt::Debug) -> ! {
        panic!("{msg}: {error:?}")
    }
    unwrap_failed_inner(msg, &error)
}

// This is a separate function to avoid constructing a `dyn Debug`
// that gets immediately thrown away, since vtables don't get cleaned up
// by dead code elimination if a trait object is constructed even if it goes
// unused
#[cfg(panic = "immediate-abort")]
#[inline]
#[cold]
#[track_caller]
const fn unwrap_failed<T>(_msg: &str, _error: T) -> ! {
    panic!()
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementations
/////////////////////////////////////////////////////////////////////////////

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, E> Clone for Result<T, E>
where
    T: Clone,
    E: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Ok(x) => Ok(x.clone()),
            Err(x) => Err(x.clone()),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Ok(to), Ok(from)) => to.clone_from(from),
            (Err(to), Err(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

#[unstable(feature = "ergonomic_clones", issue = "132290")]
impl<T, E> crate::clone::UseCloned for Result<T, E>
where
    T: crate::clone::UseCloned,
    E: crate::clone::UseCloned,
{
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, E> IntoIterator for Result<T, E> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Returns a consuming iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is [`Result::Ok`], otherwise none.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<u32, &str> = Ok(5);
    /// let v: Vec<u32> = x.into_iter().collect();
    /// assert_eq!(v, [5]);
    ///
    /// let x: Result<u32, &str> = Err("nothing!");
    /// let v: Vec<u32> = x.into_iter().collect();
    /// assert_eq!(v, []);
    /// ```
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع مكررًا مستهلكًا على القيمة الموجودة (إن وُجدت).
    ///
    /// المكرر يعطي قيمة واحدة إذا كانت النتيجة `Result::Ok`، وإلا لا شيء.
    ///
    /// </div>
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { inner: self.ok() }
    }
}

#[stable(since = "1.4.0", feature = "result_iter")]
impl<'a, T, E> IntoIterator for &'a Result<T, E> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[stable(since = "1.4.0", feature = "result_iter")]
impl<'a, T, E> IntoIterator for &'a mut Result<T, E> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

/////////////////////////////////////////////////////////////////////////////
// The Result Iterators
/////////////////////////////////////////////////////////////////////////////

/// An iterator over a reference to the [`Ok`] variant of a [`Result`].
///
/// The iterator yields one value if the result is [`Ok`], otherwise none.
///
/// Created by [`Result::iter`].
///
/// <div lang="ar" dir="rtl">
///
/// مكرر على مرجع إلى حالة `Ok` في `Result`.
///
/// المكرر يعطي قيمة واحدة إذا كانت النتيجة `Ok`، وإلا لا شيء.
///
/// تنشئه `Result::iter`.
///
/// </div>
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, T: 'a> {
    inner: Option<&'a T>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.inner.take()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Iter<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Iter<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for Iter<'_, A> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { inner: self.inner }
    }
}

/// An iterator over a mutable reference to the [`Ok`] variant of a [`Result`].
///
/// Created by [`Result::iter_mut`].
///
/// <div lang="ar" dir="rtl">
///
/// مكرر على مرجع قابل للتعديل إلى حالة `Ok` في `Result`.
///
/// تنشئه `Result::iter_mut`.
///
/// </div>
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut T>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IterMut<'_, T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IterMut<'_, T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for IterMut<'_, A> {}

/// An iterator over the value in a [`Ok`] variant of a [`Result`].
///
/// The iterator yields one value if the result is [`Ok`], otherwise none.
///
/// This struct is created by the [`into_iter`] method on
/// [`Result`] (provided by the [`IntoIterator`] trait).
///
/// [`into_iter`]: IntoIterator::into_iter
///
/// <div lang="ar" dir="rtl">
///
/// مكرر على القيمة في حالة `Ok` من `Result`.
///
/// المكرر يعطي قيمة واحدة إذا كانت النتيجة `Ok`، وإلا لا شيء.
///
/// هذه البنية تنشئها الدالة `into_iter` على `Result` (من الـ trait `IntoIterator`).
///
/// </div>
#[derive(Clone, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IntoIter<T> {
    inner: Option<T>,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.inner.take()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IntoIter<T> {}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IntoIter<T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A> TrustedLen for IntoIter<A> {}

/////////////////////////////////////////////////////////////////////////////
// FromIterator
/////////////////////////////////////////////////////////////////////////////

#[stable(feature = "rust1", since = "1.0.0")]
impl<T, E, V: FromIterator<T>> FromIterator<Result<T, E>> for Result<V, E> {
    /// Takes each element in the `Iterator`: if it is an `Err`, no further
    /// elements are taken, and the `Err` is returned. Should no `Err` occur, a
    /// container with the values of each `Result` is returned.
    ///
    /// Here is an example which increments every integer in a vector,
    /// checking for overflow:
    ///
    /// ```
    /// let v = vec![1, 2];
    /// let res: Result<Vec<u32>, &'static str> = v.iter().map(|x: &u32|
    ///     x.checked_add(1).ok_or("Overflow!")
    /// ).collect();
    /// assert_eq!(res, Ok(vec![2, 3]));
    /// ```
    ///
    /// Here is another example that tries to subtract one from another list
    /// of integers, this time checking for underflow:
    ///
    /// ```
    /// let v = vec![1, 2, 0];
    /// let res: Result<Vec<u32>, &'static str> = v.iter().map(|x: &u32|
    ///     x.checked_sub(1).ok_or("Underflow!")
    /// ).collect();
    /// assert_eq!(res, Err("Underflow!"));
    /// ```
    ///
    /// Here is a variation on the previous example, showing that no
    /// further elements are taken from `iter` after the first `Err`.
    ///
    /// ```
    /// let v = vec![3, 2, 1, 10];
    /// let mut shared = 0;
    /// let res: Result<Vec<u32>, &'static str> = v.iter().map(|x: &u32| {
    ///     shared += x;
    ///     x.checked_sub(2).ok_or("Underflow!")
    /// }).collect();
    /// assert_eq!(res, Err("Underflow!"));
    /// assert_eq!(shared, 6);
    /// ```
    ///
    /// Since the third element caused an underflow, no further elements were taken,
    /// so the final value of `shared` is 6 (= `3 + 2 + 1`), not 16.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يأخذ كل عنصر من المكرر: إذا كان `Err` لا يأخذ عناصر أخرى ويرجع الـ `Err`. وإذا لم يظهر أي
    /// `Err`
    /// يرجع حاوية فيها قيم كل `Result`.
    ///
    /// المثال الأول يزيد كل عدد صحيح في متجه مع التحقق من الفيضان. والثاني يحاول طرح واحد من قائمة
    /// أخرى
    /// مع التحقق من النقص تحت الحد. والثالث يبين أنه لا تؤخذ عناصر أخرى من `iter` بعد أول `Err`.
    ///
    /// لأن العنصر الثالث سبب نقصًا تحت الحد، لم تؤخذ عناصر أخرى، فالقيمة النهائية لـ `shared` هي 6
    /// (= `3 + 2 + 1`) وليست 16.
    ///
    /// </div>
    #[inline]
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Result<V, E> {
        iter::try_process(iter.into_iter(), |i| i.collect())
    }
}

#[unstable(feature = "try_trait_v2", issue = "84277", old_name = "try_trait")]
#[rustc_const_unstable(feature = "const_try", issue = "74935")]
const impl<T, E> ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<!, E>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

#[unstable(feature = "try_trait_v2", issue = "84277", old_name = "try_trait")]
#[rustc_const_unstable(feature = "const_try", issue = "74935")]
const impl<T, E, F: [const] From<E>> ops::FromResidual<Result<!, E>> for Result<T, F> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: Result<!, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
#[diagnostic::do_not_recommend]
#[unstable(feature = "try_trait_v2_yeet", issue = "96374")]
#[rustc_const_unstable(feature = "const_try", issue = "74935")]
const impl<T, E, F: [const] From<E>> ops::FromResidual<ops::Yeet<E>> for Result<T, F> {
    #[inline]
    fn from_residual(ops::Yeet(e): ops::Yeet<E>) -> Self {
        Err(From::from(e))
    }
}

#[unstable(feature = "try_trait_v2_residual", issue = "91285")]
#[rustc_const_unstable(feature = "const_try", issue = "74935")]
const impl<T, E> ops::Residual<T> for Result<!, E> {
    type TryType = Result<T, E>;
}
