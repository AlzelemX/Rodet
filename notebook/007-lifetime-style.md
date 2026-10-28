# `_a` | Edit LifeTime Style

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Syntax

When a lifetime is still needed (mostly in structs), write it like a normal
name starting with `_`, not with a quote. `'static` becomes `_static`.

### Before (Rust)

```rust
struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn rest(&self) -> &'a str {
        self.input
    }
}

const NAME: &'static str = "Rust";
```

### After (Rodet)

```rust
struct Parser<_a> {
    input: &_a str,
}

impl<_a> Parser<_a> {
    fn rest(&self) -> &_a str {
        self.input
    }
}

const NAME: &_static str = "Rodet";
```

### Rules

- `_a`, `_x`, `_input`: `_` then a small letter, in `<...>`, after `&`, and
  in bounds (`T: _static`).
- A variable called `_a` is still a normal variable.
- The old `'a` still works for now; removing it completely is planned.

<details><summary>Where in the compiler</summary>

- `compiler/rustc_parse/src/parser/rodet.rs`: `is_named_lifetime`, `named_lifetime`
- `tests/ui/rodet/named-lifetimes.rs`: test

</details>
