# Introduction

> 🌐 [اقرأ هذا الكتاب بالعربية](https://alzelemx.github.io/rodet/ar/)

**Rodet** is a programming language built from Rust 1.101. It keeps
everything that makes Rust good — speed like C, no garbage collector, no
crashes from bad memory use — and changes the parts that make Rust hard to
read and slow to learn.

```rust
fn greet(name: &str = "world", times: u32 = 1) {
    for _ in 0..times {
        out!("Hello, {name}!");
    }
}

fn main() {
    greet();
    greet("Rodet", times = 2);
}
```

```text
Hello, world!
Hello, Rodet!
Hello, Rodet!
```

## What is different from Rust

| Rust | Rodet | Chapter |
|---|---|---|
| `println!("...")` | `out!("...")` | [Hello, World!](start/hello.md) |
| `area(3, 4)` | `area(width = 3, height = 4)` | [Named Arguments](basics/named-arguments.md) |
| many `new_with_...` functions | `fn open(path: &str, write: bool = false)` | [Default Values](basics/default-values.md) |
| `fn f<'a>(x: &'a str) -> &'a str` | `fn f(x: &str) -> &{x} str` | [Borrow Sources](ownership/borrow-sources.md) |
| `struct P<'a> { s: &'a str }` | `struct P<_a> { s: &_a str }` | [Named Lifetimes](ownership/named-lifetimes.md) |
| `&'static str` | `&_static str` | [Named Lifetimes](ownership/named-lifetimes.md) |

Everything else is the same as Rust, so Rust libraries (crates) work in
Rodet too.

## How to read this book

Read it in order the first time: each chapter uses what the chapters before
it taught. Boxes like this one mark what is new in Rodet:

> **New in Rodet:** something Rust doesn't have.

Every example in this book is a complete program. Copy it into
`src/main.rs` of a Cargo project and run it with `cargo run`.
