# Structs

A **struct** groups values that belong together under one name.

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

- Define it with `struct Name { field: Type, ... }`.
- Make one with `Name { field: value, ... }`.
- Read a field with `.field`. To change fields, the variable needs `mut`.

## Shortcuts

```rust
struct Point {
    x: i32,
    y: i32,
}

fn make(x: i32, y: i32) -> Point {
    Point { x, y }                   // same as Point { x: x, y: y }
}

fn main() {
    let a = make(1, 2);
    let b = Point { y: 10, ..a };    // the rest of the fields from `a`
    let Point { x, y } = b;          // take it apart
    out!("{x} {y}");
}
```

## Tuple structs and unit structs

```rust
struct Meters(f64);          // one unnamed field: a number with a meaning
struct Color(u8, u8, u8);
struct Marker;               // no fields

fn main() {
    let d = Meters(5.5);
    let red = Color(255, 0, 0);
    let _m = Marker;
    out!("{} {}", d.0, red.0);
}
```

`Meters(5.5)` and a plain `5.5` are different types, so you can't pass
seconds where meters are expected.

## Printing a struct

Add `#[derive(Debug)]` and print it with `{:?}` (or `{:#?}` for one field
per line):

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

`derive` writes common code for you. Other useful ones: `Clone`, `Copy`,
`PartialEq` (for `==`), `Default`.
