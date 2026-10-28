//! Standard library macros
//!
//! This module contains a set of macros which are exported from the standard
//! library. Each macro is available for use when linking against the standard
//! library.
//!
//! <div lang="ar" dir="rtl">
//!
//! ماكروهات المكتبة القياسية.
//!
//! هذه الوحدة فيها مجموعة ماكروهات تصدّرها المكتبة القياسية. كل ماكرو متاح للاستخدام عند الربط مع
//! المكتبة القياسية.
//!
//! </div>
// ignore-tidy-file-dbg

#[cfg(test)]
mod tests;

#[doc = include_str!("../../core/src/macros/panic.md")]
#[macro_export]
#[rustc_builtin_macro(std_panic)]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable(edition_panic)]
#[cfg_attr(not(test), rustc_diagnostic_item = "std_panic_macro")]
macro_rules! panic {
    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`
    // depending on the edition of the caller.
    ($($arg:tt)*) => {
        /* compiler built-in */
    };
}

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use [`io::stdout().flush()`][flush] to ensure the output is emitted
/// immediately.
///
/// The `print!` macro will lock the standard output on each call. If you call
/// `print!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// write!(lock, "hello world").unwrap();
/// ```
///
/// Use `print!` only for the primary output of your program. Use
/// [`eprint!`] instead to print error and progress messages.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// [flush]: crate::io::Write::flush
/// [`println!`]: crate::println
/// [`eprint!`]: crate::eprint
/// [lock]: crate::io::Stdout
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// use std::io::{self, Write};
///
/// print!("this ");
/// print!("will ");
/// print!("be ");
/// print!("on ");
/// print!("the ");
/// print!("same ");
/// print!("line ");
///
/// io::stdout().flush().unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// io::stdout().flush().unwrap();
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// يطبع إلى الإخراج القياسي.
///
/// مثل الماكرو `println!` إلا أنه لا يطبع سطرًا جديدًا في نهاية الرسالة.
///
/// لاحظ أن الإخراج القياسي غالبًا مخزن سطرًا بسطر افتراضيًا، فقد تحتاج استخدام
/// `io::stdout().flush()`
/// لضمان ظهور الإخراج فورًا.
///
/// الماكرو `print!` يقفل الإخراج القياسي في كل استدعاء. إذا استدعيت `print!` داخل حلقة كثيفة فقد
/// يكون
/// هذا عنق الزجاجة فيها. لتجنب ذلك اقفل الإخراج بـ `io::stdout().lock()`.
///
/// استخدم `print!` فقط للإخراج الأساسي لبرنامجك، واستخدم `eprint!` لطباعة رسائل الأخطاء والتقدم.
///
/// انظر توثيق التنسيق في `std::fmt` لتفاصيل صيغة معاملات الماكرو.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stdout()`.
///
/// الكتابة إلى إخراج قياسي لا ينتظر (non-blocking) قد تسبب خطأ، فيسبب هذا الماكرو panic.
///
/// </div>
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "print_macro")]
#[allow_internal_unstable(print_internals)]
#[rustc_diagnostic_opaque]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args!($($arg)*));
    }};
}

/// Prints to the standard output, with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// This macro uses the same syntax as [`format!`], but writes to the standard output instead.
/// See [`std::fmt`] for more information.
///
/// The `println!` macro will lock the standard output on each call. If you call
/// `println!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// writeln!(lock, "hello world").unwrap();
/// ```
///
/// Use `println!` only for the primary output of your program. Use
/// [`eprintln!`] instead to print error and progress messages.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// [`std::fmt`]: crate::fmt
/// [`eprintln!`]: crate::eprintln
/// [lock]: crate::io::Stdout
///
/// # Panics
///
/// Panics if writing to [`io::stdout`] fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// [`io::stdout`]: crate::io::stdout
///
/// # Examples
///
/// ```
/// println!(); // prints just a newline
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// let local_variable = "some";
/// println!("format {local_variable} arguments");
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// يطبع إلى الإخراج القياسي، مع سطر جديد.
///
/// على كل المنصات، السطر الجديد هو حرف LINE FEED (`\n`/`U+000A`) وحده (بدون CARRIAGE RETURN `\r`).
///
/// يستخدم هذا الماكرو نفس صيغة `format!` لكنه يكتب إلى الإخراج القياسي. انظر `std::fmt` لمزيد من
/// المعلومات.
///
/// الماكرو `println!` يقفل الإخراج القياسي في كل استدعاء. إذا استدعيت `println!` داخل حلقة كثيفة
/// فقد
/// يكون هذا عنق الزجاجة فيها. لتجنب ذلك اقفل الإخراج بـ `io::stdout().lock()`.
///
/// استخدم `println!` فقط للإخراج الأساسي لبرنامجك، واستخدم `eprintln!` لطباعة رسائل الأخطاء
/// والتقدم.
///
/// انظر توثيق التنسيق في `std::fmt` لتفاصيل صيغة معاملات الماكرو.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stdout`.
///
/// الكتابة إلى إخراج قياسي لا ينتظر (non-blocking) قد تسبب خطأ، فيسبب هذا الماكرو panic.
///
/// </div>
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "println_macro")]
#[allow_internal_unstable(print_internals, format_args_nl)]
#[rustc_diagnostic_opaque]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}

/// Prints to the standard output, with a newline. **(Rodet only)**
///
/// `out!` is the Rodet name for [`println!`]: it takes exactly the same
/// arguments and uses the same format syntax, so anything you can write with
/// `println!` you can write with `out!`.
///
/// This macro does not exist in upstream Rust; code that uses it only
/// compiles with the Rodet compiler.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// # Panics
///
/// Panics if writing to [`io::stdout`] fails.
///
/// [`io::stdout`]: crate::io::stdout
///
/// # Examples
///
/// ```
/// out!(); // prints just a newline
/// out!("hello from Rodet!");
/// let name = "out!";
/// out!("{name} works like println!: {} + {} = {}", 1, 2, 1 + 2);
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// يطبع إلى الإخراج القياسي، مع سطر جديد. **(خاص بـ Rodet)**
///
/// `out!` هو اسم Rodet لـ `println!`: يأخذ نفس المعاملات تمامًا ويستخدم نفس صيغة التنسيق، فكل ما
/// تكتبه
/// بـ `println!` يمكنك كتابته بـ `out!`.
///
/// هذا الماكرو غير موجود في Rust الأصلية؛ الكود الذي يستخدمه يُترجم فقط بمترجم Rodet.
///
/// انظر توثيق التنسيق في `std::fmt` لتفاصيل صيغة معاملات الماكرو.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stdout`.
///
/// </div>
#[macro_export]
#[stable(feature = "rodet_out_macro", since = "CURRENT_RUSTC_VERSION")]
#[allow_internal_unstable(print_internals, format_args_nl)]
#[rustc_diagnostic_opaque]
macro_rules! out {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}

/// Prints to the standard error.
///
/// Equivalent to the [`print!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`print!`] for
/// example usage.
///
/// Use `eprint!` only for error and progress messages. Use `print!`
/// instead for the primary output of your program.
///
/// [`io::stderr`]: crate::io::stderr
/// [`io::stdout`]: crate::io::stdout
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// eprint!("Error: Could not complete task");
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// يطبع إلى مجرى الأخطاء القياسي.
///
/// مثل الماكرو `print!` إلا أن الإخراج يذهب إلى `io::stderr` بدل `io::stdout`. انظر `print!` لمثال
/// على الاستخدام.
///
/// استخدم `eprint!` فقط لرسائل الأخطاء والتقدم، واستخدم `print!` للإخراج الأساسي لبرنامجك.
///
/// انظر توثيق التنسيق في `std::fmt` لتفاصيل صيغة معاملات الماكرو.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stderr`.
///
/// الكتابة إلى مجرى أخطاء لا ينتظر (non-blocking) قد تسبب خطأ، فيسبب هذا الماكرو panic.
///
/// </div>
#[macro_export]
#[stable(feature = "eprint", since = "1.19.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "eprint_macro")]
#[allow_internal_unstable(print_internals)]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::io::_eprint($crate::format_args!($($arg)*));
    }};
}

/// Prints to the standard error, with a newline.
///
/// Equivalent to the [`println!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`println!`] for
/// example usage.
///
/// Use `eprintln!` only for error and progress messages. Use `println!`
/// instead for the primary output of your program.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// [`io::stderr`]: crate::io::stderr
/// [`io::stdout`]: crate::io::stdout
/// [`println!`]: crate::println
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// eprintln!("Error: Could not complete task");
/// ```
///
/// <div lang="ar" dir="rtl">
///
/// يطبع إلى مجرى الأخطاء القياسي، مع سطر جديد.
///
/// مثل الماكرو `println!` إلا أن الإخراج يذهب إلى `io::stderr` بدل `io::stdout`. انظر `println!`
/// لمثال
/// على الاستخدام.
///
/// استخدم `eprintln!` فقط لرسائل الأخطاء والتقدم، واستخدم `println!` للإخراج الأساسي لبرنامجك.
///
/// انظر توثيق التنسيق في `std::fmt` لتفاصيل صيغة معاملات الماكرو.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stderr`.
///
/// الكتابة إلى مجرى أخطاء لا ينتظر (non-blocking) قد تسبب خطأ، فيسبب هذا الماكرو panic.
///
/// </div>
#[macro_export]
#[stable(feature = "eprint", since = "1.19.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "eprintln_macro")]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_eprint($crate::format_args_nl!($($arg)*));
    }};
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// An example:
///
/// ```rust
/// let a = 2;
/// let b = dbg!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2:9] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// The macro works by using the `Debug` implementation of the type of
/// the given expression to print the value to [stderr] along with the
/// source location of the macro invocation as well as the source code
/// of the expression.
///
/// Invoking the macro on an expression moves and takes ownership of it
/// before returning the evaluated expression unchanged. If the type
/// of the expression does not implement `Copy` and you don't want
/// to give up ownership, you can instead borrow with `dbg!(&expr)`
/// for some expression `expr`.
///
/// The `dbg!` macro works exactly the same in release builds.
/// This is useful when debugging issues that only occur in release
/// builds or when debugging in release mode is significantly faster.
///
/// Note that the macro is intended as a debugging tool and therefore you
/// should avoid having uses of it in version control for long periods
/// (other than in tests and similar).
/// Debug output from production code is better done with other facilities
/// such as the [`debug!`] macro from the [`log`] crate.
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon
/// and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Further examples
///
/// With a method call:
///
/// ```rust
/// fn foo(n: usize) {
///     if let Some(_) = dbg!(n.checked_sub(4)) {
///         // ...
///     }
/// }
///
/// foo(3)
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:2:22] n.checked_sub(4) = None
/// ```
///
/// Naive factorial implementation:
///
/// ```rust
/// fn factorial(n: u32) -> u32 {
///     if dbg!(n <= 1) {
///         dbg!(1)
///     } else {
///         dbg!(n * factorial(n - 1))
///     }
/// }
///
/// dbg!(factorial(4));
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:2:8] n <= 1 = false
/// [src/main.rs:2:8] n <= 1 = false
/// [src/main.rs:2:8] n <= 1 = false
/// [src/main.rs:2:8] n <= 1 = true
/// [src/main.rs:3:9] 1 = 1
/// [src/main.rs:7:9] n * factorial(n - 1) = 2
/// [src/main.rs:7:9] n * factorial(n - 1) = 6
/// [src/main.rs:7:9] n * factorial(n - 1) = 24
/// [src/main.rs:9:1] factorial(4) = 24
/// ```
///
/// The `dbg!(..)` macro moves the input:
///
/// ```compile_fail
/// /// A wrapper around `usize` which importantly is not Copyable.
/// #[derive(Debug)]
/// struct NoCopy(usize);
///
/// let a = NoCopy(42);
/// let _ = dbg!(a); // <-- `a` is moved here.
/// let _ = dbg!(a); // <-- `a` is moved again; error!
/// ```
///
/// You can also use `dbg!()` without a value to just print the
/// file and line whenever it's reached.
///
/// Finally, if you want to `dbg!(..)` multiple values, it will treat them as
/// a tuple (and return it, too):
///
/// ```
/// assert_eq!(dbg!(1usize, 2u32), (1, 2));
/// ```
///
/// However, a single argument with a trailing comma will still not be treated
/// as a tuple, following the convention of ignoring trailing commas in macro
/// invocations. You can use a 1-tuple directly if you need one:
///
/// ```
/// assert_eq!(1, dbg!(1u32,)); // trailing comma ignored
/// assert_eq!((1,), dbg!((1u32,))); // 1-tuple
/// ```
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log
///
/// <div lang="ar" dir="rtl">
///
/// يطبع قيمة التعبير المعطى ويرجعها، للتنقيح السريع.
///
/// يعمل الماكرو باستخدام تطبيق `Debug` لنوع التعبير المعطى لطباعة القيمة إلى stderr، مع مكان
/// استدعاء
/// الماكرو في الكود المصدري ونص التعبير نفسه.
///
/// استدعاء الماكرو على تعبير ينقله ويأخذ ملكيته قبل إرجاع قيمته بدون تغيير. إذا كان نوع التعبير لا
/// يطبّق `Copy` ولا تريد التخلي عن الملكية، يمكنك الاستعارة بـ `dbg!(&expr)`.
///
/// يعمل الماكرو `dbg!` بنفس الطريقة تمامًا في بناء الإصدار (release). هذا مفيد عند تنقيح مشاكل تحدث
/// فقط
/// في بناء الإصدار، أو عندما يكون التنقيح في وضع الإصدار أسرع بكثير.
///
/// لاحظ أن الماكرو أداة تنقيح، لذلك تجنب إبقاء استخداماته في نظام التحكم بالإصدارات لفترات طويلة
/// (ما عدا
/// الاختبارات وما يشبهها). الأفضل لإخراج التنقيح في كود الإنتاج استخدام أدوات أخرى مثل الماكرو
/// `debug!`
/// من الحزمة `log`.
///
/// # الاستقرار
///
/// لا تعتمد على الإخراج المطبوع بالضبط من هذا الماكرو، فقد يتغير مستقبلًا.
///
/// # حالات الـ panic
///
/// يسبب panic إذا فشلت الكتابة إلى `io::stderr`.
///
/// # أمثلة إضافية
///
/// يمكن استخدامه مع استدعاء دالة، ومع دالة مضروب بسيطة، وهو ينقل المدخل. يمكنك أيضًا استخدام
/// `dbg!()`
/// بدون قيمة لطباعة الملف والسطر فقط عند الوصول إليه.
///
/// وأخيرًا، إذا أعطيت `dbg!(..)` عدة قيم فسيعاملها كـ tuple (ويرجعها أيضًا). لكن معاملًا واحدًا مع
/// فاصلة
/// في النهاية لن يُعامل كـ tuple، اتباعًا لقاعدة تجاهل الفواصل الأخيرة في استدعاءات الماكرو. يمكنك
/// استخدام tuple من عنصر واحد مباشرة إذا احتجته.
///
/// </div>
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "dbg_macro")]
#[stable(feature = "dbg_macro", since = "1.32.0")]
#[rustc_diagnostic_opaque]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::eprintln!("[{}:{}:{}]", $crate::file!(), $crate::line!(), $crate::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
                    $crate::file!(),
                    $crate::line!(),
                    $crate::column!(),
                    $crate::stringify!($val),
                    // The `&T: Debug` check happens here (not in the format literal desugaring)
                    // to avoid format literal related messages and suggestions.
                    &&tmp as &dyn $crate::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

#[doc(hidden)]
#[macro_export]
#[allow_internal_unstable(hash_map_internals)]
#[unstable(feature = "hash_map_internals", issue = "none")]
macro_rules! repetition_utils {
    (@count $($tokens:tt),*) => {{
        [$($crate::repetition_utils!(@replace $tokens => ())),*].len()
    }};

    (@replace $x:tt => $y:tt) => { $y }
}

/// Creates a [`HashMap`] containing the arguments.
///
/// `hash_map!` allows specifying the entries that make
/// up the [`HashMap`] where the key and value are separated by a `=>`.
///
/// The entries are separated by commas with a trailing comma being allowed.
///
/// It is semantically equivalent to using repeated [`HashMap::insert`]
/// on a newly created hashmap.
///
/// `hash_map!` will attempt to avoid repeated reallocations by
/// using [`HashMap::with_capacity`].
///
/// # Examples
///
/// ```rust
/// #![feature(hash_map_macro)]
/// use std::hash_map;
///
/// let map = hash_map! {
///     "key" => "value",
///     "key1" => "value1"
/// };
///
/// assert_eq!(map.get("key"), Some(&"value"));
/// assert_eq!(map.get("key1"), Some(&"value1"));
/// assert!(map.get("brrrrrrooooommm").is_none());
/// ```
///
/// And with a trailing comma
///
///```rust
/// #![feature(hash_map_macro)]
/// use std::hash_map;
///
/// let map = hash_map! {
///     "key" => "value", // notice the ,
/// };
///
/// assert_eq!(map.get("key"), Some(&"value"));
/// ```
///
/// The key and value are moved into the HashMap.
///
/// [`HashMap`]: crate::collections::HashMap
/// [`HashMap::insert`]: crate::collections::HashMap::insert
/// [`HashMap::with_capacity`]: crate::collections::HashMap::with_capacity
///
/// <div lang="ar" dir="rtl">
///
/// ينشئ `HashMap` فيه المعاملات المعطاة.
///
/// `hash_map!` يسمح بتحديد مدخلات الـ `HashMap` حيث يفصل `=>` بين المفتاح والقيمة.
///
/// تُفصل المدخلات بفواصل، ويُسمح بفاصلة في النهاية.
///
/// معناه مثل استدعاء `HashMap::insert` مرارًا على جدول جديد.
///
/// يحاول `hash_map!` تجنب إعادة الحجز المتكررة باستخدام `HashMap::with_capacity`.
///
/// </div>
#[macro_export]
#[allow_internal_unstable(hash_map_internals)]
#[unstable(feature = "hash_map_macro", issue = "144032")]
macro_rules! hash_map {
    () => {{
        $crate::collections::HashMap::new()
    }};

    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = $crate::collections::HashMap::with_capacity(
            const { $crate::repetition_utils!(@count $($key),*) }
        );
        $( map.insert($key, $value); )*
        map
    }}
}
