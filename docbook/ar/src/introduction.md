# المقدمة

> 🌐 [Read this book in English](https://alzelemx.github.io/rodet/)

**Rodet** لغة برمجة مبنية من Rust 1.101. تحتفظ بكل ما يجعل Rust جيدة —
سرعة مثل C، بدون جامع نفايات (garbage collector)، وبدون انهيارات بسبب
الاستخدام الخاطئ للذاكرة — وتغيّر الأجزاء التي تجعل Rust صعبة القراءة وبطيئة
التعلم.

```rust
fn greet(name: &str = "world", times: u32 = 1) {
    for _ in 0..times {
        out!("Hello, {name}!");
    }
}

fn main() {
    greet();
    greet("Rodet", times = 2);
}
```

```text
Hello, world!
Hello, Rodet!
Hello, Rodet!
```

## ما الذي يختلف عن Rust

| Rust | Rodet | الفصل |
|---|---|---|
| `println!("...")` | `out!("...")` | [مرحبا يا عالم!](start/hello.md) |
| `area(3, 4)` | `area(width = 3, height = 4)` | [الوسائط بالاسم](basics/named-arguments.md) |
| دوال كثيرة مثل `new_with_...` | `fn open(path: &str, write: bool = false)` | [القيم الافتراضية](basics/default-values.md) |
| `fn f<'a>(x: &'a str) -> &'a str` | `fn f(x: &str) -> &{x} str` | [مصادر الاستعارة](ownership/borrow-sources.md) |
| `struct P<'a> { s: &'a str }` | `struct P<_a> { s: &_a str }` | [الأعمار المسماة](ownership/named-lifetimes.md) |
| `&'static str` | `&_static str` | [الأعمار المسماة](ownership/named-lifetimes.md) |

كل شيء آخر مثل Rust تمامًا، لذلك مكتبات Rust (crates) تعمل في Rodet أيضًا.

## كيف تقرأ هذا الكتاب

اقرأه بالترتيب في المرة الأولى: كل فصل يستخدم ما علّمته الفصول التي قبله.
مربعات مثل هذا تدل على الجديد في Rodet:

> **جديد في Rodet:** شيء غير موجود في Rust.

كل مثال في هذا الكتاب برنامج كامل. انسخه إلى `src/main.rd` في مشروع Rodet
وشغّله بـ `rodet run`.
