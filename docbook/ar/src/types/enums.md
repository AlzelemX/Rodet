# التعدادات (enum) و match

الـ struct يعني "هذا **و** ذاك". والـ **enum** يعني "هذا **أو** ذاك": قيمة
تكون واحدة من عدة أنواع.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn(d: Direction) -> &_static str {
    match d {
        Direction::Up => "going up",
        Direction::Down => "going down",
        Direction::Left | Direction::Right => "going sideways",
    }
}

fn main() {
    out!("{}", turn(Direction::Left));
}
```

## كل نوع يمكن أن يحمل بيانات

كل نوع يمكن أن يحمل قيمه الخاصة:

```rust
enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Point,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14159 * radius * radius,
            Shape::Rect { width, height } => width * height,
            Shape::Point => 0.0,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::Rect { width: 2.0, height: 3.0 },
        Shape::Point,
    ];
    for s in &shapes {
        out!("{:.2}", s.area());
    }
}
```

يجب أن تعالج `match` كل نوع. أضف نوعًا جديدًا إلى `Shape` وسيعرض لك المترجم
كل `match` تحتاج تحديثًا: لا تُنسى أي حالة.

## Option: قيمة أو لا شيء

Rust++ ليس فيها `null`. القيمة التي قد تكون غير موجودة نوعها `Option`:

```rust
fn find_age(name: &str) -> Option<u32> {
    if name == "Sara" { Some(25) } else { None }
}

fn main() {
    match find_age("Sara") {
        Some(age) => out!("age {age}"),
        None => out!("unknown"),
    }

    // صيغ أقصر
    if let Some(age) = find_age("Ali") {
        out!("Ali is {age}");
    }
    let age = find_age("Ali").unwrap_or(0);
    out!("{age}");
}
```

لا يمكنك استخدام `Option<u32>` كأنه `u32` بالخطأ: يجب أن تعالج `None` أولًا.
هذا يزيل عائلة كاملة من الانهيارات.

## الأنماط في match

```rust
fn describe(point: (i32, i32)) -> String {
    match point {
        (0, 0) => String::from("center"),
        (x, 0) => format!("on the x line at {x}"),
        (0, y) => format!("on the y line at {y}"),
        (x, y) if x == y => String::from("on the diagonal"),
        _ => String::from("somewhere"),
    }
}

fn main() {
    out!("{}", describe((3, 3)));
}
```

`if` بعد النمط *حارس*: شرط إضافي.

> **قريبًا في Rust++:** `data`، نوع واحد يمكن أن يكون struct أو enum أو
> الاثنين، ودوال `match` المسماة. انظر [قريبًا](../appendix/coming-soon.md).
