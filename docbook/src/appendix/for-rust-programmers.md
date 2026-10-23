# Rust++ for Rust Programmers

If you already know Rust, this page is all you need. Rust++ is Rust
1.101.0-dev with these changes; everything else, including crates.io, works
as in Rust.

| Rust | Rust++ |
|---|---|
| `println!("{x}")` | `out!("{x}")` (`println!` still works) |
| `area(3, 4)` | `area(width = 3, height = 4)`, any order, after positional ones |
| `fn open(p: &str, w: bool)` + `open(p, false)` | `fn open(p: &str, w: bool = false)` + `open(p)` |
| `fn f<'a>(x: &'a str, y: &str) -> &'a str` | `fn f(x: &str, y: &str) -> &{x} str` |
| `fn f(&self) -> &str` | the same, or `-> &{self} str` |
| `struct P<'a> { s: &'a str }` | `struct P<_a> { s: &_a str }` |
| `&'static str`, `T: 'static` | `&_static str`, `T: _static` |
| `rustup`, `rustc --version` → `rustc 1.101.0-dev` | `rustup++_init`, `rustc 1.101.0-dev (Rust++ #19)` |
| rust-analyzer | `rust++-analyzer` and the [DEVR+](../start/devr.md) editor |

## How the new syntax is compiled

| Feature | Becomes | Cost |
|---|---|---|
| `out!` | `println!` | none |
| `&{x, y} T` | one lifetime shared by `x`, `y` and the result | none |
| `_a` | `'_a` (`_static` → `'static`) | none |
| named arguments | positional call in parameter order | none |
| default values | hidden `const` per parameter, used for left-out arguments | none |

## Limits today

- Default values: constant expressions only; not in traits.
- Named arguments: on functions and methods called directly, not on
  closures or function pointers.
- rustfmt doesn't know the new syntax yet.

Every change has a card in the
[NoteBook](https://github.com/AlzelemX/Rust-/tree/main/notebook) with the
date and before/after code.
