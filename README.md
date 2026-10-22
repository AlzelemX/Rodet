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

## What is different from Rust?

| Change | Where |
|---|---|
| New `out!` macro: prints a line, exactly like `println!` | `library/std/src/macros.rs`, `library/std/src/prelude/v1.rs` |
| `rustc --version` reports `(Rust++)` | `src/bootstrap/src/core/session.rs` |
| Test for `out!` | `tests/ui/rustpp/out-macro.rs` |
| Automatic Windows + Linux build and release on every push | `.github/workflows/build-rustpp.yml` |
| Builds without rust-lang servers | `rustpp/setup-deps.py`, `src/ci-llvm-sha`, `src/bootstrap/src/core/download.rs` |

```rust
fn main() {
    let lang = "Rust++";
    out!("Hello from {lang}!");          // new in Rust++
    out!("{} + {} = {}", 1, 2, 1 + 2);   // same format syntax as println!
}
```

## Download and use a prebuilt compiler

Every push to `main` builds the compiler on GitHub Actions and publishes it on
the [Releases](../../releases) page, for **Windows** and **Linux** (x86_64).

You need [rustup](https://rustup.rs) installed (it provides `cargo`).

### Windows (PowerShell)

1. Download `rustpp-...-x86_64-pc-windows-msvc.zip` from Releases and extract
   it, for example to `C:\rustpp`.
2. Register it with rustup and use it:

```powershell
rustup toolchain link rustpp C:\rustpp
rustc +rustpp --version          # rustc 1.101.0-dev (Rust++)
rustc +rustpp C:\rustpp\hello.rs
.\hello.exe
cargo +rustpp run                # inside a cargo project
```

### Linux

```bash
tar -xJf rustpp-*-x86_64-unknown-linux-gnu.tar.xz
rustup toolchain link rustpp "$PWD/rustpp"
rustc +rustpp rustpp/hello.rs && ./hello
```

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
python x.py build library
rustup toolchain link rustpp build/host/stage1
```

## License

Same as upstream Rust: dual-licensed under MIT and Apache 2.0. See
[`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`COPYRIGHT`](COPYRIGHT).
