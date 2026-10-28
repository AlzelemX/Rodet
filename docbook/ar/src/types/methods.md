# الدوال المرتبطة (methods)

الدوال التي تنتمي لنوع تُكتب في كتلة `impl`:

```rust
struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    // بدون `self`: تُستدعى هكذا Rect::new(...)
    fn new(width: f64, height: f64 = 1.0) -> Rect {
        Rect { width, height }
    }

    fn square(size: f64) -> Rect {
        Rect::new(width = size, height = size)
    }

    // &self: تقرأ القيمة
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // &mut self: تغيّر القيمة
    fn scale(&mut self, by: f64 = 2.0) {
        self.width *= by;
        self.height *= by;
    }

    // self: تأخذ القيمة (تنتقل إلى الدالة)
    fn into_square(self) -> Rect {
        Rect::square(self.width.max(self.height))
    }
}

fn main() {
    let mut r = Rect::new(width = 3.0, height = 4.0);
    out!("{}", r.area());        // 12
    r.scale();
    out!("{}", r.area());        // 48
    r.scale(by = 0.5);
    let s = r.into_square();
    out!("{}", s.area());        // 16
    // لا يمكن استخدام r بعد الآن: into_square أخذته
}
```

| المعامل الأول | معناه | الاستدعاء |
|---|---|---|
| (لا شيء) | تنتمي للنوع | `Rect::new(...)` |
| `&self` | تقرأ | `r.area()` |
| `&mut self` | تغيّر | `r.scale()` |
| `self` | تأخذ | `r.into_square()` |

اختر الأضعف الذي يكفي: `&self` إذا كنت تقرأ فقط.

`Self` (بحرف S كبير) اسم مختصر لنوع الـ `impl`:
`fn new(...) -> Self` تعني `-> Rect`.

> **Rodet:** [الوسائط بالاسم](../basics/named-arguments.md)
> و[القيم الافتراضية](../basics/default-values.md) تعمل مع الدوال المرتبطة:
> `Rect::new(width = 3.0)`، `r.scale(by = 0.5)`.
