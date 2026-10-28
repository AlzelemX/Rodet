# Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(2, 3);
    out!("{sum}");
}
```

- `fn add` — the name.
- `(a: i32, b: i32)` — the parameters, each with its type. Types are
  required here: they are the function's promise to its callers.
- `-> i32` — the type it gives back. No `->` means it gives back nothing.
- `a + b` — **the last expression, without `;`, is the result.**

## The last line is the result

```rust
fn square(n: i32) -> i32 {
    n * n        // no `;` → this is returned
}

fn square_too(n: i32) -> i32 {
    return n * n; // `return` also works; use it to leave early
}

fn main() {
    out!("{} {}", square(4), square_too(5));
}
```

A `;` turns an expression into a statement that gives nothing. Writing
`n * n;` in `square` is an error: the function promised an `i32` and gives
nothing.

## Blocks are expressions too

```rust
fn main() {
    let grade = 85;
    let label = if grade >= 90 { "excellent" } else { "good" };

    let area = {
        let w = 3;
        let h = 4;
        w * h
    };
    out!("{label} {area}");
}
```

## Two Rodet extras

Functions in Rodet can be called with names, and parameters can have
default values:

```rust
fn rect(width: u32, height: u32 = 1) -> u32 {
    width * height
}

fn main() {
    out!("{}", rect(width = 3, height = 4)); // 12
    out!("{}", rect(5));                     // 5, height = 1
}
```

The next two chapters explain them.
