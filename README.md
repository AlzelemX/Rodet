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
| New `out!` macro: prints a line, exactly like `println!` | `library/std/src/macros.rs` |
| `rustc --version` reports `(Rust++)` | `src/bootstrap/src/core/session.rs` |
| Test for `out!` | `tests/ui/rustpp/out-macro.rs` |
| Automatic build + release on every push | `.github/workflows/build-rustpp.yml` |

```rust
fn main() {
    let lang = "Rust++";
    out!("Hello from {lang}!");          // new in Rust++
    out!("{} + {} = {}", 1, 2, 1 + 2);   // same format syntax as println!
}
```

## Download and use a prebuilt compiler

Every push to `main` builds the compiler on GitHub Actions and publishes it on
the [Releases](../../releases) page (Linux x86_64).

```bash
# 1. download rustpp-*-x86_64-unknown-linux-gnu.tar.xz from Releases, then:
tar -xJf rustpp-*-x86_64-unknown-linux-gnu.tar.xz

# 2. register it with rustup under the name "rustpp"
rustup toolchain link rustpp "$PWD/rustpp"

# 3. use it
rustc +rustpp --version          # rustc 1.101.0-dev (Rust++)
rustc +rustpp rustpp/examples/hello.rs && ./hello
cargo +rustpp run                # cargo comes from your normal toolchain
```

## Build it yourself

```bash
git clone https://github.com/AlzelemX/Rust- rustpp && cd rustpp

# Let bootstrap see the upstream commit, so it can download prebuilt LLVM.
UPSTREAM_SHA=c303e414dd813ab090055754d0be647054fce70a
git fetch --depth 1 https://github.com/rust-lang/rust "$UPSTREAM_SHA"
git replace --graft "$(git rev-list --max-parents=0 HEAD)" "$UPSTREAM_SHA"

./x setup compiler
./x build library
rustup toolchain link rustpp build/host/stage1
```

Rust++ is based on upstream `rust-lang/rust` commit
`c303e414dd813ab090055754d0be647054fce70a` (2026-10-22). The build workflow
links this repository's history to that upstream commit so the build can
download the matching prebuilt LLVM instead of compiling LLVM from source.

## License

Same as upstream Rust: dual-licensed under MIT and Apache 2.0. See
[`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`COPYRIGHT`](COPYRIGHT).
