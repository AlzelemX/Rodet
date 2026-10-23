# Rust++ لمبرمجي Rust

إذا كنت تعرف Rust، فهذه الصفحة كل ما تحتاجه. Rust++ هي Rust 1.101.0-dev مع
هذه التغييرات؛ وكل شيء آخر، بما فيه crates.io، يعمل كما في Rust.

| Rust | Rust++ |
|---|---|
| `println!("{x}")` | `out!("{x}")` (و`println!` ما زالت تعمل) |
| `area(3, 4)` | `area(width = 3, height = 4)`، بأي ترتيب، بعد القيم بالترتيب |
| `fn open(p: &str, w: bool)` + `open(p, false)` | `fn open(p: &str, w: bool = false)` + `open(p)` |
| `fn f<'a>(x: &'a str, y: &str) -> &'a str` | `fn f(x: &str, y: &str) -> &{x} str` |
| `fn f(&self) -> &str` | نفسها، أو `-> &{self} str` |
| `struct P<'a> { s: &'a str }` | `struct P<_a> { s: &_a str }` |
| `&'static str`، `T: 'static` | `&_static str`، `T: _static` |
| `rustup`، `rustc --version` ← `rustc 1.101.0-dev` | `rustup++_init`، `rustc 1.101.0-dev (Rust++ #19)` |
| rust-analyzer | `rust++-analyzer` ومحرر [DEVR+](../start/devr.md) |

## كيف تُترجم الكتابة الجديدة

| الميزة | تصبح | التكلفة |
|---|---|---|
| `out!` | `println!` | لا شيء |
| `&{x, y} T` | عمر واحد مشترك بين `x` و`y` والنتيجة | لا شيء |
| `_a` | `'_a` (و`_static` ← `'static`) | لا شيء |
| الوسائط بالاسم | استدعاء بالترتيب حسب ترتيب المعاملات | لا شيء |
| القيم الافتراضية | `const` مخفي لكل معامل، يُستخدم للوسائط المتروكة | لا شيء |

## الحدود اليوم

- القيم الافتراضية: تعابير ثابتة فقط؛ ليست في الـ trait.
- الوسائط بالاسم: على الدوال والدوال المرتبطة التي تُستدعى مباشرة، وليس على
  الإغلاقات (closures) أو مؤشرات الدوال.
- rustfmt لا يعرف الكتابة الجديدة بعد.

لكل تغيير بطاقة في
[NoteBook](https://github.com/AlzelemX/Rust-/tree/main/notebook) فيها التاريخ
والكود قبل وبعد.
