# `--version` | The Compiler Says Rodet

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

The compiler tells you it is Rodet, so you always know which compiler you
are running.

### Before (Rust)

```text
> rustc --version
rustc 1.101.0-dev
```

### After (Rodet)

```text
> rustc --version
rustc 1.101.0-dev (Rodet)
```

**Update 2026-10-23:** releases add their build number, `rustc 1.101.0-dev
(Rodet #19)`, so Cargo knows a new Rodet is a different compiler and never
reuses files built by an older one.

<details><summary>Where in the compiler</summary>

- `src/bootstrap/src/core/session.rs`: sets the description to `Rodet`
- `.github/workflows/build-rodet.yml`: adds the build number (`build.description`)

</details>
