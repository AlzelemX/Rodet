# Enums and match

A struct is "this **and** that". An **enum** is "this **or** that": a value
that is one of several kinds.

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

## Kinds can carry data

Each kind can hold its own values:

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

`match` must handle every kind. Add a new kind to `Shape` and the compiler
shows you every `match` that needs updating: no case is forgotten.

## Option: a value or nothing

Rust++ has no `null`. A value that may be missing is an `Option`:

```rust
fn find_age(name: &str) -> Option<u32> {
    if name == "Sara" { Some(25) } else { None }
}

fn main() {
    match find_age("Sara") {
        Some(age) => out!("age {age}"),
        None => out!("unknown"),
    }

    // shorter forms
    if let Some(age) = find_age("Ali") {
        out!("Ali is {age}");
    }
    let age = find_age("Ali").unwrap_or(0);
    out!("{age}");
}
```

You can't use an `Option<u32>` as a `u32` by accident: you must handle
`None` first. That removes a whole family of crashes.

## Patterns in match

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

`if` after a pattern is a *guard*: an extra condition.

> **Coming to Rust++:** `data`, one type that can be a struct, an enum, or
> both, and named `match` functions. See [Coming Soon](../appendix/coming-soon.md).
