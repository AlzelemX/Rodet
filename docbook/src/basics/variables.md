# Variables

Make a variable with `let`:

```rust
fn main() {
    let x = 5;
    out!("x = {x}");
}
```

## Variables can't change unless you say so

```rust,compile_fail
fn main() {
    let x = 5;
    x = 6; // error: cannot assign twice to immutable variable `x`
}
```

Add `mut` (mutable) to let a variable change:

```rust
fn main() {
    let mut count = 0;
    count = count + 1;
    count += 1;
    out!("count = {count}"); // 2
}
```

Why not `mut` by default? When a value can't change, you never have to
check who changed it. Most variables never change, so the few with `mut`
stand out.

## Giving the type yourself

Rust++ works out the type for you, but you can write it after `:`:

```rust
fn main() {
    let age: u32 = 30;
    let price: f64 = 9.99;
    let name: &str = "Ali";
    out!("{name}, {age}, {price}");
}
```

Remember: **`:` means "has the type"**, **`=` means "gets the value"**.
Rust++ keeps these two meanings everywhere, also in
[named arguments](named-arguments.md) and [default values](default-values.md).

## Shadowing

A new `let` with the same name makes a new variable that hides the old one.
It can even have a different type:

```rust
fn main() {
    let input = "42";
    let input: i32 = input.parse().unwrap();
    out!("{}", input + 1); // 43
}
```

## Constants

Values that never change in the whole program. The type is required, and
the name is in CAPITALS:

```rust
const MAX_PLAYERS: u32 = 4;
const GAME: &_static str = "Space";

fn main() {
    out!("{GAME}: up to {MAX_PLAYERS} players");
}
```

`&_static str` means "text that lives for the whole program"; see
[Named Lifetimes](../ownership/named-lifetimes.md).
