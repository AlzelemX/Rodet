# `rodet` | Rodet's Own Tools: rodetc, Silk, `.rd`

> 📅 **2026-10-28** · 🟢 Done · 🏷️ Tooling

Every tool, file and folder of Rodet now has a Rodet name. The compiler is
`rodetc`, the package manager is **Silk** (the `rodet` command), source files
end in `.rd`, projects are described by `Silk.toml`, and everything lives in
`~/.rodet`. The logo is the spider 🕷️: Rodet is about threads.

### Before (Rust)

```text
> cargo new hello          hello/Cargo.toml, hello/src/main.rs
> cargo run
> rustc --version          rustc 1.101.0-dev (Rodet #22)
~/.cargo/                  downloaded crates
compiler/rustc_driver/     the compiler's crates
```

### After (Rodet)

```text
> rodet new hello          hello/Silk.toml, hello/src/main.rd
> rodet run                Silk.lock is written next to Silk.toml
> rodetc --version         rodetc 0.1.0 (#23, 2026-10-28)
~/.rodet/                  Rodet itself and downloaded libraries
compiler/rodetc_driver/    the compiler's crates
```

| Rust | Rodet |
|---|---|
| `rustc`, `rustdoc` | `rodetc`, `rodetdoc` |
| `cargo` | `rodet` (Silk) |
| `Cargo.toml`, `Cargo.lock` | `Silk.toml`, `Silk.lock` |
| `.rs`, `src/main.rs`, `build.rs` | `.rd`, `src/main.rd`, `build.rd` |
| `~/.cargo`, `CARGO_HOME`, `.cargo/config.toml` | `~/.rodet`, `RODET_HOME`, `.rodet/config.toml` |
| `compiler/rustc_*` | `compiler/rodetc_*` |

### Rules

- A crate whose root file is `.rd` loads `mod x;` from `x.rd` or `x/mod.rd`;
  a crate whose root is `.rs` keeps loading `.rs` files.
- Libraries from crates.io are Rust packages: Silk still reads their
  `Cargo.toml` and `.rs` files, so all of crates.io works in Rodet. A Rodet
  package published to crates.io gets its `Silk.toml` as `Cargo.toml`.
- Tools that need the classic answers get them with `RODET_COMPAT=1`:
  `rodetc --version` then prints `rustc 1.101.0-dev (Rodet #23)` and
  `rodet --version` prints `cargo 1.101.0`. Silk sets it for build scripts
  (crates like `rustversion` read `$RUSTC --version`) and rodet-analyzer
  sets it for its own questions.
- Build scripts still get the `CARGO_*` environment variables, and
  `cargo::` lines from build scripts still work: crates.io packages use them.

<details><summary>Where in the code</summary>

- `compiler/rodetc_*`: all compiler crates renamed (`rustc_*` → `rodetc_*`)
- `compiler/rodetc_driver_impl/src/lib.rs`: `rodetc --version`, `RODET_COMPAT`
- `compiler/rodetc_expand/src/module.rs`: `.rd` modules
- `compiler/rodetc_interface/src/util.rs`: rodetdoc finds `rodetc`
- `src/bootstrap/src/core/builder/cargo.rs`: `RODET_COMPAT=1` while building
- Silk (the `silk` branch, a fork of Cargo): `src/util/important_paths.rs`
  (`Silk.toml`), `src/ops/lockfile.rs` (`Silk.lock`),
  `src/workspace/parser/targets.rs` (`.rd` targets), `src/context/mod.rs`
  (`~/.rodet`, finding `rodetc`), `src/ops/cargo_new.rs` (new projects)
- `src/tools/rust-analyzer/crates/toolchain`: `rodet`/`rodetc` first;
  `project-model`: `Silk.toml`; `hir-def/src/nameres/mod_resolution.rs`: `.rd`
- `devr/`: DEVR+ runs `rodet`, shows the Silk view, opens `.rd` files
- `.github/workflows/build-rodet.yml`: builds Silk and packages
  `rodetc`, `rodetdoc`, `rodet`

</details>
