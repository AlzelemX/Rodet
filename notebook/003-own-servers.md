# `deps-snapshot` | Build Without Rust's Servers

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

Building the compiler used to download the starting compiler, LLVM and
crates from rust-lang servers. Now everything comes from this repository's
`deps-snapshot` release, so Rodet builds even if those servers change or
go away.

### Before (Rust)

```text
python x.py build
  downloading https://static.rust-lang.org/dist/.../rustc-beta.tar.xz
  downloading https://ci-artifacts.rust-lang.org/.../rust-dev.tar.xz
  downloading crates from crates.io
```

### After (Rodet)

```text
python rodet/setup-deps.py
  downloading github.com/AlzelemX/rodet/releases/download/deps-snapshot/...
python x.py build library
  (nothing downloaded from rust-lang)
```

### What comes from where

| Needed to build | Comes from |
|---|---|
| Starting compiler (stage0 rustc, std, cargo) | `deps-snapshot` release |
| Prebuilt LLVM (pinned in `src/ci-llvm-sha`) | `deps-snapshot` release |
| crates.io dependencies | `deps-snapshot` release (`vendor.tar.xz`) |
| `library/backtrace` | committed directly in the repository |

To add a platform, run the *Snapshot dependencies* workflow with its target.

<details><summary>Where in the compiler</summary>

- `rodet/setup-deps.py`: downloads the snapshot and writes `bootstrap.toml`
- `src/ci-llvm-sha`: the LLVM version to use
- `src/bootstrap/src/core/download.rs`, `src/bootstrap/src/core/config/config.rs`: read it
- `.github/workflows/snapshot-deps.yml`: makes the snapshot

</details>
