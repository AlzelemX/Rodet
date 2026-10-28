# `RODET_LANG=ar` | Arabic Inside The Language

> 📅 **2026-10-24** · 🟢 Done · 🏷️ Compiler, Std, Tooling

The Arabic moved from the editor into Rodet itself: the compiler prints its
messages in Arabic, and every function of the standard library is explained
in Arabic next to the English, in the same file.

### Before

```text
$ cargo build
error[E0425]: cannot find value `x` in this scope

hover on Vec::push: English only (DEVR+ had Arabic for ~100 items)
```

### After

```text
$ RODET_LANG=ar cargo build
error[E0425]: لا يوجد قيمة بالاسم `x` هنا

hover on Vec::push (rodet.language = "ar"):
  يضيف عنصرًا إلى نهاية المجموعة.
  # حالات الـ panic …
  (the English code example below it)
```

### How the docs look in the source

```rust
/// Appends an element to the back of a collection.
///
/// <div lang="ar" dir="rtl">
///
/// يضيف عنصرًا إلى نهاية المجموعة.
///
/// </div>
pub fn push(&mut self, value: T) { … }
```

rustdoc shows both languages; rodet-analyzer shows the one you chose and
keeps the code examples.

### Coverage

| Part | Arabic |
|---|---|
| Compiler messages | 5583 / 7935 (70%): parser, type checking, borrow checker, resolve, lints, traits; the rest is mostly codegen and options |
| `std` / `core` / `alloc` items | 7488 / 7551 have an Arabic summary (99%) |
| Whole docs in Arabic | `Option`, `Result`, `String`, `Vec`, `HashMap`, `print!`, `println!`, `out!`, `eprint!`, `eprintln!`, `dbg!`, `vec!`, `format!`, `hash_map!` |

### Rules

- `RODET_LANG=ar` (or `ar_SA`, …) turns Arabic on; anything else, or unset,
  is English. A message without a translation stays English.
- Names, types, paths and code inside messages are never translated.
- Doc examples stay one: the English one, shown in both languages.

<details><summary>Where</summary>

- `compiler/rustc_errors/src/i18n.rs`, `i18n/ar.txt`: the message table and
  how it is matched (Fluent and `format!` messages, words in their holes)
- `rodet/i18n/compiler_messages.py`: finds untranslated messages, coverage
- `rodet/i18n/std_docs.py`: Arabic blocks in the std docs (`coverage`,
  `batch`, `apply`, `full`, `apply-full`)
- `src/tools/rust-analyzer/crates/ide-db/src/documentation.rs`: shows one
  language; setting `rodet.language`
- `devr/core/src/toolchain.rs`, `lsp.rs`, `terminal.rs`: DEVR+ passes
  `RODET_LANG` and `rodet.language` from its Language setting

</details>
