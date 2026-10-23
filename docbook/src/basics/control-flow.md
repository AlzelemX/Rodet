# Control Flow

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

    // `if` gives a value
    let wear = if temp > 30 { "t-shirt" } else { "jacket" };
    out!("{wear}");
}
```

The condition must be a `bool`. `if count { ... }` with a number is an
error: write `if count != 0`.

## loop, while, for

```rust
fn main() {
    // loop: forever, until `break`. `break value` gives a value.
    let mut n = 1;
    let first_big = loop {
        n *= 2;
        if n > 100 { break n; }
    };
    out!("{first_big}"); // 128

    // while: as long as the condition is true
    let mut left = 3;
    while left > 0 {
        out!("{left}...");
        left -= 1;
    }

    // for: over a range or a list
    for i in 0..3 {          // 0, 1, 2
        out!("i = {i}");
    }
    for i in 1..=3 {         // 1, 2, 3
        out!("i = {i}");
    }
    for name in ["Ali", "Sara"] {
        out!("hi {name}");
    }
}
```

Use `for` whenever you can: it can't go past the end of a list.
`continue` skips to the next round.

## match

`match` compares a value against patterns, top to bottom, and runs the
first one that fits:

```rust
fn main() {
    let code = 404;
    let text = match code {
        200 => "OK",
        301 | 302 => "moved",          // either one
        400..=499 => "your mistake",   // a range
        _ => "something else",         // anything left
    };
    out!("{text}");
}
```

A `match` must cover **every** possible value; `_` catches the rest. If you
forget a case, the program doesn't compile. This becomes very useful with
[enums](../types/enums.md).
