# Types

Every value has a type. The type tells the compiler how much memory the
value needs and what you can do with it.

## Numbers

| Type | Holds | Example |
|---|---|---|
| `i32` | whole numbers, −2 147 483 648 to 2 147 483 647 (the default) | `-7` |
| `i64` | bigger whole numbers | `9_000_000_000` |
| `u8`, `u32`, `u64` | whole numbers, never negative | `255` |
| `usize` | sizes and positions in lists | `v.len()` |
| `f64` | numbers with a point (the default) | `3.14` |
| `f32` | smaller, less precise `f64` | `1.5` |

`i` = can be negative, `u` = unsigned (0 and up), the number = bits.

```rust
fn main() {
    let a = 10;           // i32
    let b = 3.0;          // f64
    let big = 1_000_000;  // _ makes big numbers readable

    out!("{}", a / 3);    // 3   (whole-number division)
    out!("{}", a % 3);    // 1   (remainder)
    out!("{}", b / 2.0);  // 1.5
    out!("{}", a as f64 / 4.0); // 2.5  (`as` converts)
    out!("{big}");
}
```

Numbers of different types don't mix by accident: `a + b` above is an
error until you convert one of them with `as`. No silent precision loss.

## true and false

```rust
fn main() {
    let ready: bool = true;
    let big = 10 > 5; // true
    out!("{} {} {}", ready && big, ready || false, !ready);
}
```

## Characters and text

```rust
fn main() {
    let letter: char = 'R';     // one character, single quotes
    let arabic = 'ع';           // any language, any emoji
    let fixed: &str = "hello";  // text written in the program
    let mut owned = String::from("hello"); // text you can grow
    owned.push_str(", world");

    out!("{letter} {arabic} {fixed} {owned}");
    out!("{}", owned.len()); // length in bytes: 12
}
```

`&str` vs `String` is explained in [Ownership](../ownership/ownership.md).
For now: `"..."` is a `&str`; use `String` when the text changes.

## Tuples and arrays

```rust
fn main() {
    let point: (i32, i32) = (3, 4);   // tuple: fixed group, any types
    let (x, y) = point;               // take it apart
    out!("x={x} y={y} first={}", point.0);

    let days = ["Sat", "Sun", "Mon"]; // array: fixed length, one type
    out!("{} of {}", days[0], days.len());

    let zeros = [0; 5];               // [0, 0, 0, 0, 0]
    out!("{zeros:?}");
}
```

Reading `days[10]` stops the program with a clear error instead of reading
random memory. For lists that grow, use `Vec` ([Collections](../everyday/collections.md)).
