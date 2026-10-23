# التحكم في سير البرنامج

## if

```rust
fn main() {
    let temp = 38;

    if temp > 40 {
        out!("very hot");
    } else if temp > 30 {
        out!("hot");
    } else {
        out!("nice");
    }

    // `if` تعطي قيمة
    let wear = if temp > 30 { "t-shirt" } else { "jacket" };
    out!("{wear}");
}
```

الشرط يجب أن يكون `bool`. كتابة `if count { ... }` مع رقم خطأ: اكتب
`if count != 0`.

## loop و while و for

```rust
fn main() {
    // loop: للأبد، حتى `break`. و`break value` يعطي قيمة.
    let mut n = 1;
    let first_big = loop {
        n *= 2;
        if n > 100 { break n; }
    };
    out!("{first_big}"); // 128

    // while: ما دام الشرط صحيحًا
    let mut left = 3;
    while left > 0 {
        out!("{left}...");
        left -= 1;
    }

    // for: على مدى أو قائمة
    for i in 0..3 {          // 0، 1، 2
        out!("i = {i}");
    }
    for i in 1..=3 {         // 1، 2، 3
        out!("i = {i}");
    }
    for name in ["Ali", "Sara"] {
        out!("hi {name}");
    }
}
```

استخدم `for` كلما أمكنك: لا يمكن أن تتجاوز نهاية القائمة.
و`continue` تنتقل إلى الدورة التالية.

## match

`match` تقارن القيمة بأنماط، من الأعلى إلى الأسفل، وتنفذ أول نمط يطابق:

```rust
fn main() {
    let code = 404;
    let text = match code {
        200 => "OK",
        301 | 302 => "moved",          // واحد من الاثنين
        400..=499 => "your mistake",   // مدى
        _ => "something else",         // أي شيء متبقٍّ
    };
    out!("{text}");
}
```

يجب أن تغطي `match` **كل** القيم الممكنة؛ و`_` تلتقط الباقي. إذا نسيت حالة،
لا يُترجم البرنامج. هذا مفيد جدًا مع [التعدادات](../types/enums.md).
