# `x: T = v` | Default Parameter Values

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Syntax

A parameter can have a default value. Leave it out when calling, and the
default is used. Together with named arguments, there is no need for
builder types or many `new_with_...` functions.

### Before (Rust)

```rust
fn open(path: &str, read: bool, write: bool) -> String {
    format!("{path} r={read} w={write}")
}

fn main() {
    println!("{}", open("a.txt", true, false));
    println!("{}", open("b.txt", true, true));
}
```

### After (Rodet)

```rust
fn open(path: &str, read: bool = true, write: bool = false) -> String {
    format!("{path} r={read} w={write}")
}

fn main() {
    out!("{}", open("a.txt"));
    out!("{}", open("b.txt", write = true));
}
```

### Rules

- The default must be a constant value: numbers, `"text"`, `true`, `'c'`,
  constants and `const fn` calls.
- Allowed on functions in a module and in `impl Type { ... }`; not yet in
  traits.
- Skip any parameter that has a default; give the others by position or by
  name.
- No runtime cost: the default is a hidden constant.

<details><summary>Where in the compiler</summary>

- `compiler/rustc_parse/src/parser/rodet.rs`: reads `= value` and makes a hidden constant
- `compiler/rustc_middle/src/ty/rodet.rs`: finds the constant for a parameter
- `compiler/rustc_hir_typeck/src/fn_ctxt/rodet.rs`, `compiler/rustc_mir_build/src/thir/cx/expr.rs`: fill in left-out parameters
- `tests/ui/rodet/param-defaults.rs`, `param-defaults-errors.rs`: tests

</details>
