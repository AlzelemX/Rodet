# `out!` | New Print Macro

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Std

A shorter name for printing a line. `out!` works exactly like `println!`:
same format strings, same speed. `println!` still works.

### Before (Rust)

```rust
fn main() {
    let lang = "Rust";
    println!("Hello from {lang}!");
    println!("{} + {} = {}", 1, 2, 1 + 2);
}
```

### After (Rust++)

```rust
fn main() {
    let lang = "Rust++";
    out!("Hello from {lang}!");
    out!("{} + {} = {}", 1, 2, 1 + 2);
}
```

### Rules

- Available everywhere without `use`, like `println!`.
- `out!()` with nothing inside prints an empty line.

<details><summary>Where in the compiler</summary>

- `library/std/src/macros.rs`: the macro
- `library/std/src/prelude/v1.rs`: makes it available without `use`
- `tests/ui/rustpp/out-macro.rs`: test

</details>
