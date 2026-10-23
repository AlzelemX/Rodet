# الهياكل (struct)

**الـ struct** يجمع قيمًا مترابطة تحت اسم واحد.

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let mut user = User {
        name: String::from("Sara"),
        age: 25,
        active: true,
    };

    user.age += 1;
    out!("{} is {} (active: {})", user.name, user.age, user.active);
}
```

- عرّفه بـ `struct Name { field: Type, ... }`.
- أنشئ واحدًا بـ `Name { field: value, ... }`.
- اقرأ حقلًا بـ `.field`. لتغيير الحقول، يحتاج المتغير `mut`.

## اختصارات

```rust
struct Point {
    x: i32,
    y: i32,
}

fn make(x: i32, y: i32) -> Point {
    Point { x, y }                   // مثل Point { x: x, y: y }
}

fn main() {
    let a = make(1, 2);
    let b = Point { y: 10, ..a };    // باقي الحقول من `a`
    let Point { x, y } = b;          // فكّه إلى أجزاء
    out!("{x} {y}");
}
```

## struct على شكل صف، و struct بدون حقول

```rust
struct Meters(f64);          // حقل واحد بلا اسم: رقم له معنى
struct Color(u8, u8, u8);
struct Marker;               // بدون حقول

fn main() {
    let d = Meters(5.5);
    let red = Color(255, 0, 0);
    let _m = Marker;
    out!("{} {}", d.0, red.0);
}
```

`Meters(5.5)` و`5.5` العادي نوعان مختلفان، فلا يمكنك أن تعطي ثوانيَ حيث
يُنتظر أمتار.

## طباعة struct

أضف `#[derive(Debug)]` واطبعه بـ `{:?}` (أو `{:#?}` لحقل في كل سطر):

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    out!("{p:?}");   // Point { x: 3, y: 4 }
}
```

`derive` يكتب لك كودًا شائعًا. ومن المفيد أيضًا: `Clone` و`Copy`
و`PartialEq` (لـ `==`) و`Default`.
