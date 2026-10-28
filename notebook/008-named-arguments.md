# `x = 1` | Named Arguments

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Syntax

Write the parameter name when you call a function, in any order. The call
reads like a sentence, and two numbers can't be swapped by mistake.

`=` is used, not `:`, because `=` already means "give a value" and `:`
means "has the type".

### Before (Rust)

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let a = area(3, 4); // which one is the width?
    println!("{a}");
}
```

### After (Rodet)

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    out!("{}", area(width = 3, height = 4));
    out!("{}", area(height = 4, width = 3)); // any order
    out!("{}", area(3, height = 4));         // positional first, then named
}
```

### Rules

- Positional arguments first; after the first named one, all must be named.
- Each parameter can be given once.
- Works for functions, `Type::new(...)` and methods: `win.resize(height = 1080, width = 1920)`.
- No runtime cost: the compiler puts the values back in order.

<details><summary>Where in the compiler</summary>

- `compiler/rustc_hir/src/rodet.rs`: matches names to parameters
- `compiler/rustc_hir_typeck/src/fn_ctxt/rodet.rs`: type checking and errors
- `compiler/rustc_mir_build/src/thir/cx/expr.rs`: builds the call in order
- `tests/ui/rodet/named-args.rs`, `named-args-errors.rs`: tests

</details>
