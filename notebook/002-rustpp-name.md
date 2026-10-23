# `--version` | The Compiler Says Rust++

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

The compiler tells you it is Rust++, so you always know which compiler you
are running.

### Before (Rust)

```text
> rustc --version
rustc 1.101.0-dev
```

### After (Rust++)

```text
> rustc --version
rustc 1.101.0-dev (Rust++)
```

<details><summary>Where in the compiler</summary>

- `src/bootstrap/src/core/session.rs`: sets the description to `Rust++`

</details>
