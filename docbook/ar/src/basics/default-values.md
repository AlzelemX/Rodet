# القيم الافتراضية

> **جديد في Rodet:** يمكن أن يكون للمعامل قيمة افتراضية: `name: Type = value`.
> اتركه عند الاستدعاء فتُستخدم القيمة الافتراضية.

```rust
fn connect(host: &str, port: u16 = 443, secure: bool = true) {
    out!("{host}:{port} secure={secure}");
}

fn main() {
    connect("example.com");                       // المنفذ 443، آمن
    connect("example.com", 8080);                 // آمن
    connect("localhost", port = 3000, secure = false);
    connect("localhost", secure = false);         // المنفذ 443
}
```

```text
example.com:443 secure=true
example.com:8080 secure=true
localhost:3000 secure=false
localhost:443 secure=false
```

مع [الوسائط بالاسم](named-arguments.md) يمكنك ترك أي معامل له قيمة افتراضية،
وليس الأخيرة فقط: `connect("localhost", secure = false)` يبقي المنفذ
الافتراضي.

## داخل الأنواع

القيم الافتراضية تعمل في كتل `impl` أيضًا، فلا تحتاج دوالًا كثيرة مثل
`new_with_...`:

```rust
struct Counter {
    value: i64,
    step: i64,
}

impl Counter {
    fn new(start: i64 = 0, step: i64 = 1) -> Counter {
        Counter { value: start, step }
    }

    fn tick(&mut self, times: i64 = 1) {
        self.value += self.step * times;
    }
}

fn main() {
    let mut a = Counter::new();
    a.tick();
    a.tick(times = 5);
    out!("{}", a.value); // 6

    let b = Counter::new(step = 10);
    out!("{}", b.value + b.step); // 10
}
```

## القواعد

- **القيمة الافتراضية يجب أن تكون ثابتة**: أرقام، `"نص"`، `true`، `'c'`،
  `None`، ثوابت، وعمليات حسابية عليها (`MAX / 2`).

  ```rust,compile_fail
  fn hello(name: String = String::from("world")) {} // خطأ: ليست ثابتة

  fn main() {}
  ```

  لقيمة افتراضية نصية، خذ `&str` بدلًا من `String`: `fn hello(name: &str = "world")`.

- مسموحة في الدوال وفي `impl Type { ... }`. ليست في الـ trait بعد.
- المعاملات التي ليس لها قيمة افتراضية يجب إعطاؤها.

## التكلفة

لا شيء. كل قيمة افتراضية تُحفظ مرة واحدة كثابت مخفي، والاستدعاء الذي يترك
المعامل يستخدم ذلك الثابت كأنك كتبته بنفسك.
