# الأخطاء

في Rust++ نوعان من الأخطاء:

- **العيوب (bugs)** التي يجب ألا تحدث أبدًا: يتوقف البرنامج بـ `panic!`.
- **الفشل المتوقع**، مثل ملف غير موجود أو مدخل خاطئ: الدالة ترجع `Result`
  ومن يستدعيها يقرر ماذا يفعل.

## Result

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("can't divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 4.0) {
        Ok(v) => out!("result {v}"),
        Err(e) => out!("error: {e}"),
    }
    match divide(1.0, 0.0) {
        Ok(v) => out!("result {v}"),
        Err(e) => out!("error: {e}"),
    }
}
```

`Result<T, E>` إما `Ok(value)` أو `Err(error)`. مثل `Option`، لا يمكنك استخدام
القيمة بدون معالجة الخطأ أولًا.

## علامة ? : مرّر الخطأ للأعلى

داخل دالة ترجع `Result`، علامة `?` بعد استدعاء تعني: *إذا كان هذا خطأ، أرجعه
الآن؛ وإلا أعطني القيمة*.

```rust
fn parse_pair(text: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let mut parts = text.split(',');
    let a = parts.next().unwrap_or("").trim().parse::<i32>()?;
    let b = parts.next().unwrap_or("").trim().parse::<i32>()?;
    Ok((a, b))
}

fn main() {
    out!("{:?}", parse_pair("3, 4"));   // Ok((3, 4))
    out!("{:?}", parse_pair("3, x"));   // Err(ParseIntError { kind: InvalidDigit })
}
```

## طرق سريعة وبسيطة

| الاستدعاء | عندما يكون خطأ |
|---|---|
| `.unwrap()` | يوقف البرنامج |
| `.expect("message")` | يوقف البرنامج برسالتك |
| `.unwrap_or(value)` | يستخدم `value` بدلًا منه |
| `.unwrap_or_default()` | يستخدم `0` أو `""` أو `false`… |
| `?` | يرجع الخطأ لمن استدعى الدالة |

استخدم `unwrap` في البرامج الصغيرة والاختبارات. في الكود الحقيقي، فضّل `?`
أو `match`.

## main يمكن أن ترجع Result

```rust
use std::fs;

fn main() -> Result<(), std::io::Error> {
    fs::write("hello.txt", "salam")?;
    let text = fs::read_to_string("hello.txt")?;
    out!("{text}");
    fs::remove_file("hello.txt")?;
    Ok(())
}
```

إذا وصل خطأ إلى `main`، يطبعه البرنامج وينتهي برمز خطأ.
