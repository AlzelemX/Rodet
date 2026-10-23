# `rust++-analyzer` | A Language Server That Speaks Rust++

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Tooling

rust-analyzer (the brain behind errors, completion and "go to definition"
in editors) now understands Rust++. It ships inside every Rust++ release as
`rust++-analyzer`, built from the same commit as the compiler, so a new
Rust++ version always comes with an analyzer that knows its new syntax.

### Before (Rust's rust-analyzer)

```text
fn longest(x: &str, y: &str) -> &{x, y} str   ← red: "expected type"
struct Parser<_a> { input: &_a str }           ← red: "expected type"
area(width = 3, height = 4)                    ← red: "expected u32, found ()"
fn open(path: &str, write: bool = false)       ← red: "expected `,`"
```

### After (rust++-analyzer)

```text
no false errors, and:
hover   fn longest(x: &str, y: &str) -> &{x, y} str
hover   fn open(path: &str, read: bool = true, write: bool = false) -> String
hover   `width` in area(width = 3)  →  width: u32  (and F12 jumps to it)
```

### New things only Rust++ has

- **↩ return marks**: a mark after the value a function gives back without
  `return` (`inlayHints.returnValueHints.enable`).
- **Speed score**: the `rustpp/speedScore` request gives a file a score from
  0 to 100, with each slow pattern found and how to fix it.
- Named arguments get no misleading `width:` hints; `_` completes to
  `_static`.

<details><summary>Where</summary>

- `src/tools/rust-analyzer/crates/parser/`: `_a` lifetimes, `&{x}`, `= default`
- `src/tools/rust-analyzer/crates/hir-def/src/expr_store/lower.rs`, `hir-ty/src/infer/rustpp.rs`: named arguments and defaults
- `src/tools/rust-analyzer/crates/ide/src/inlay_hints/return_value.rs`: ↩ marks
- `src/tools/rust-analyzer/crates/ide/src/speed_score.rs`: speed score
- `.github/workflows/build-rustpp.yml`: built and shipped as `bin/rust++-analyzer`

</details>
