# Rodet for Rust Programmers

If you already know Rust, this page is all you need. Rodet is Rust
1.101.0-dev with these changes; everything else, including crates.io, works
as in Rust.

| Rust | Rodet |
|---|---|
| `println!("{x}")` | `out!("{x}")` (`println!` still works) |
| `area(3, 4)` | `area(width = 3, height = 4)`, any order, after positional ones |
| `fn open(p: &str, w: bool)` + `open(p, false)` | `fn open(p: &str, w: bool = false)` + `open(p)` |
| `fn f<'a>(x: &'a str, y: &str) -> &'a str` | `fn f(x: &str, y: &str) -> &{x} str` |
| `fn f(&self) -> &str` | the same, or `-> &{self} str` |
| `struct P<'a> { s: &'a str }` | `struct P<_a> { s: &_a str }` |
| `&'static str`, `T: 'static` | `&_static str`, `T: _static` |
| `rustup` | `rodet_init` (one toolchain, in `~/.rodet`) |
| `rustc`, `rustdoc` | `rodetc`, `rodetdoc` (`RODET_COMPAT=1 rodetc --version` prints `rustc 1.101.0-dev (Rodet #N)`) |
| `cargo`, `Cargo.toml`, `Cargo.lock` | `rodet` (Silk), `Silk.toml`, `Silk.lock`; crates.io packages keep their `Cargo.toml` |
| `.rs` files, `src/main.rs` | `.rd` files, `src/main.rd` |
| `~/.cargo`, `.cargo/config.toml` | `~/.rodet`, `.rodet/config.toml` |
| rust-analyzer | `rodet-analyzer` and the [DEVR+](../start/devr.md) editor |

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
[NoteBook](https://github.com/AlzelemX/rodet/tree/main/notebook) with the
date and before/after code.
