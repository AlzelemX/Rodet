# Rust++

**Rust++** is a personal, experimental fork of the Rust compiler and standard
library by [AlzelemX](https://github.com/AlzelemX).

> Rust++ is **not** affiliated with, endorsed by, or supported by the Rust
> Project or the Rust Foundation. "Rust" is a trademark of the Rust
> Foundation. The original upstream README is kept in
> [`README.rust-lang.md`](README.rust-lang.md).

The Rust++ changes in this repository were written with the help of an AI
assistant (Claude) for personal use and experimentation. They are not intended
to be contributed to `rust-lang/rust`.


# 🚀 [Rust ++ ] - Roadmap & Progress

<div align="center">

### 📊 Project Status Overview

| 🔴 To Do | 🟡 In Progress | 🟢 Completed | 🟣 Backlog |
| :--- | :--- | :--- | :--- |
| ![](https://img.shields.io/badge/Status-Pending-red?style=for-the-badge) | ![](https://img.shields.io/badge/Status-Active-yellow?style=for-the-badge) | ![](https://img.shields.io/badge/Status-Done-brightgreen?style=for-the-badge) | ![](https://img.shields.io/badge/Status-Future-purple?style=for-the-badge) |

</div>


### 📌 Task Breakdown & Checklists

#### 🔴 Pending (To Do)
- [ ] on_rady() | Declarative Constructor
- [ ] Set up end-to-end integration tests

#### 🟡 In Progress
- [x] Platform support | Linux , Mac , ... 
- [x] Convert 'static to 'global
- [ ] Edit Style Liftime To [] Style

#### 🟢 Completed


---

<div align="center">

💡 **Note:** This status board is updated regularly to reflect current project progress.

</div>


## What is different from Rust?

| Change | Where |
|---|---|
| New `out!` macro: prints a line, exactly like `println!` | `library/std/src/macros.rs`, `library/std/src/prelude/v1.rs` |
| Borrow sources instead of lifetimes in return types: `fn longest(x: &str, y: &str) -> &{x, y} str` | `compiler/rustc_parse/src/parser/rustpp.rs` |
| Named lifetimes written `_a` / `_static`: `struct Parser<_a> { input: &_a str }` | `compiler/rustc_parse/src/parser/rustpp.rs` |
| Named arguments: `area(width = 3, height = 4)`, any order after positional ones | `compiler/rustc_hir/src/rustpp.rs`, `compiler/rustc_hir_typeck/src/fn_ctxt/rustpp.rs` |
| Parameter default values: `fn open(path: &str, write: bool = false)`, then `open("a")` | `compiler/rustc_parse/src/parser/rustpp.rs`, `compiler/rustc_middle/src/ty/rustpp.rs` |
| `rustc --version` reports `(Rust++)` | `src/bootstrap/src/core/session.rs` |
| Test for `out!` | `tests/ui/rustpp/out-macro.rs` |
| Automatic Windows, Linux and macOS builds and release on every push | `.github/workflows/build-rustpp.yml` |
| Installers `rustup++_init.exe` (Windows) and `rustup++_init.sh` (Linux/macOS) | `rustpp/installer/` |
| Builds without rust-lang servers | `rustpp/setup-deps.py`, `src/ci-llvm-sha`, `src/bootstrap/src/core/download.rs` |

```rust
fn main() {
    let lang = "Rust++";
    out!("Hello from {lang}!");          // new in Rust++
    out!("{} + {} = {}", 1, 2, 1 + 2);   // same format syntax as println!
}
```

## Install

### Windows: `rustup++_init.exe`

1. Download **`rustup++_init.exe`** from the latest
   [release](../../releases/latest) and double-click it.
2. It installs Rust++ (`rustc`, `cargo`, `rustdoc` and the standard library)
   into `%USERPROFILE%\.rustpp`, puts `%USERPROFILE%\.rustpp\bin` first in your
   PATH, and offers to install the Visual Studio C++ Build Tools if they are
   missing (Windows programs are linked with Microsoft's linker).
3. Open a **new** terminal:

```powershell
rustc --version      # rustc 1.101.0-dev (Rust++)
cargo new hello
cd hello
cargo run
```

There is no toolchain switching: `rustc` and `cargo` are Rust++. The official
`rustup` is not needed. Run the installer again to update, or
`rustup++_init.exe --uninstall` to remove Rust++.

### Linux and macOS: `rustup++_init.sh`

```sh
curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh
```

It detects your system, installs Rust++ into `~/.rustpp`, and adds
`~/.rustpp/bin` to your PATH (in `~/.profile`, `~/.bashrc`, `~/.zshrc`).
Open a new terminal and run `rustc --version`. You also need a C linker:
`xcode-select --install` on macOS, `build-essential` (or `gcc`) on Linux.

Uninstall:

```sh
curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh -s -- --uninstall
```

### Supported platforms

| Platform | Package | Installer |
|---|---|---|
| Windows x86_64 | `rustpp-x86_64-pc-windows-msvc.zip` | `rustup++_init.exe` |
| Linux x86_64 (glibc 2.39+) | `rustpp-x86_64-unknown-linux-gnu.tar.xz` | `rustup++_init.sh` |
| Linux ARM64 (glibc 2.39+) | `rustpp-aarch64-unknown-linux-gnu.tar.xz` | `rustup++_init.sh` |
| macOS Apple Silicon | `rustpp-aarch64-apple-darwin.tar.xz` | `rustup++_init.sh` |
| macOS Intel | `rustpp-x86_64-apple-darwin.tar.xz` | `rustup++_init.sh` |

To add another platform, run the *Snapshot dependencies* workflow with its
host triple, then add it to the matrix in `.github/workflows/build-rustpp.yml`.

## Independent from Rust's servers

Rust++ is based on upstream `rust-lang/rust` commit
`c303e414dd813ab090055754d0be647054fce70a` (2026-10-22), and it no longer
downloads anything from rust-lang servers to build:

| Needed to build | Where it comes from now |
|---|---|
| stage0 compiler (rustc, std, cargo beta 2026-09-29) | `deps-snapshot` release of this repository |
| Prebuilt LLVM (pinned in `src/ci-llvm-sha`) | `deps-snapshot` release |
| crates.io dependencies | `deps-snapshot` release (`vendor.tar.xz`) |
| `library/backtrace` (was a git submodule) | committed directly in this repository |

The snapshot was made once by the *Snapshot dependencies* workflow
(`.github/workflows/snapshot-deps.yml`). `rustpp/setup-deps.py` downloads it
and puts every file where the build expects it.

## Build it yourself

Needs Python 3, Git, a C/C++ toolchain (Visual Studio Build Tools with
"Desktop development with C++" on Windows), and about 30 GB of disk.

```bash
git clone https://github.com/AlzelemX/Rust- rustpp
cd rustpp
python rustpp/setup-deps.py      # python3 on Linux
python x.py build library src/tools/rustdoc
# the toolchain is now in build/host/stage1 (add cargo from build/host/stage0/bin)
```

## License

Same as upstream Rust: dual-licensed under MIT and Apache 2.0. See
[`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`COPYRIGHT`](COPYRIGHT).
