# `&{x, y}` | Borrow Sources Instead Of Lifetimes

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Syntax

When a function returns a reference, say **which parameters it borrows
from**, by name. No lifetime letters to invent and repeat.

### Before (Rust)

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn pick<'a>(v: &'a Vec<i32>, n: &usize) -> &'a i32 {
    &v[*n]
}
```

### After (Rodet)

```rust
fn longest(x: &str, y: &str) -> &{x, y} str {
    if x.len() >= y.len() { x } else { y }
}

fn pick(v: &Vec<i32>, n: &usize) -> &{v} i32 {
    &v[*n]
}
```

Read `&{x, y} str` as "a `&str` borrowed from `x` or `y`".

### Rules

- Works anywhere in the return type: `Option<&{items} T>`, `(&{text} str, usize)`.
- `self` works too: `fn first(&self) -> &{self} str`.
- Each name must be a parameter that is a reference.
- Same speed and same checks as lifetimes: it is turned into a lifetime by
  the compiler.

<details><summary>Where in the compiler</summary>

- `compiler/rustc_parse/src/parser/rodet.rs`: reads `&{...}` and turns it into lifetimes
- `tests/ui/rodet/borrow-sources.rs`, `borrow-sources-errors.rs`: tests

</details>
