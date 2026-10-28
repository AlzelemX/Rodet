# الوحدات (modules)

عندما يكبر البرنامج، قسّمه إلى **وحدات**. الوحدة مجموعة لها اسم من الدوال
والأنواع والوحدات الأخرى.

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }

    const PI: f64 = 3.14159; // خاص: يُستخدم داخل `shapes` فقط
}

use shapes::Circle;

fn main() {
    let c = Circle { radius: 2.0 };
    out!("{:.2}", c.area());
}
```

- كل شيء **خاص (private)** في الأصل. و`pub` تجعله قابلًا للاستخدام خارج
  الوحدة.
- `use` تجلب اسمًا إلى النطاق فلا تحتاج كتابة المسار كاملًا.
- `::` تفصل أجزاء المسار: `shapes::Circle`، `std::collections::HashMap`.

## وحدة لكل ملف

في مشروع Rodet، `mod shapes;` (مع `;`) تحمّل الوحدة من ملف:

```text
src/
├── main.rd        mod shapes;  use shapes::Circle;
└── shapes.rd      pub struct Circle { ... }
```

المجلد يعمل أيضًا: `src/shapes/mod.rd`، أو `src/shapes.rd` بجانب مجلد
`src/shapes/` لوحداتها الفرعية.

## المسارات

| المسار | معناه |
|---|---|
| `crate::shapes::Circle` | من أعلى مشروعك |
| `super::helper` | من الوحدة الأم |
| `self::inner` | من هذه الوحدة |
| `std::fs` | من المكتبة القياسية |
| `rand::random` | من مكتبة في `Silk.toml` |
