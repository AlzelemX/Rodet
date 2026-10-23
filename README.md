# Rust++

A copy of **Rust 1.101.0-dev** (upstream commit
[`c303e41`](https://github.com/rust-lang/rust/commit/c303e414dd813ab090055754d0be647054fce70a),
22 Oct 2026), rebuilt to make its syntax cleaner, and the language faster,
more precise and easier to read and learn.

<!-- STATUS:START -->
<!-- Generated from STATUS.md: edit that file, not this table. -->

## Status

| Feature | Status | NoteBook |
|---|---|---|
| `out!` print macro | 🟢 Done | [001](notebook/001-out-macro.md) |
| `rustc --version` says Rust++ | 🟢 Done | [002](notebook/002-rustpp-name.md) |
| Build without Rust's servers | 🟢 Done | [003](notebook/003-own-servers.md) |
| Windows, Linux, macOS builds | 🟢 Done | [004](notebook/004-platforms.md) |
| `rustup++_init` installers | 🟢 Done | [005](notebook/005-installers.md) |
| Borrow sources `-> &{x, y} T` | 🟢 Done | [006](notebook/006-borrow-sources.md) |
| Lifetime style `_a`, `_static` | 🟢 Done | [007](notebook/007-lifetime-style.md) |
| Named arguments `f(x = 1)` | 🟢 Done | [008](notebook/008-named-arguments.md) |
| Default values `fn f(x: T = 1)` | 🟢 Done | [009](notebook/009-default-values.md) |
| Clean repository + docs system | 🟢 Done | [010](notebook/010-clean-repo.md) |
| `data`: one type for struct + enum | 🔵 Planned | |
| Named `match` | 🔵 Planned | |
| Remove `'a` completely | 🔵 Planned | |
| Error messages in Rust++ style | 🔵 Planned | |
| `on_ready()` declarative constructor | 🔵 Planned | |
| End-to-end integration tests | 🔵 Planned | |
| Android and iOS | 🟣 Idea | |
| Borrow sources inside structs `&{text}` | 🟣 Idea | |

🟢 Done · 🟡 In progress · 🔵 Planned · 🟣 Idea · 🔴 Blocked

<!-- STATUS:END -->

## A taste

```rust
fn open(path: &str, write: bool = false) -> &{path} str {
    out!("opening {path}, write = {write}");
    path
}

fn main() {
    open("notes.txt");
    open(path = "log.txt", write = true);
}
```

## Learn

| | |
|---|---|
| 📘 **[Doc Book](https://alzelemx.github.io/Rust-/)** | How to write Rust++, from Hello World to traits. Source in [`docbook/`](docbook/). |
| 📒 **[NoteBook](notebook/)** | One card for every change: what it is, before and after, and when. |

## Install

**Windows:** download [`rustup++_init.exe`](../../releases/latest) and double-click it.

**Linux / macOS:**

```sh
curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh
```

Then, in a new terminal: `rustc --version` → `rustc 1.101.0-dev (Rust++)`.

## Build it yourself

```sh
git clone https://github.com/AlzelemX/Rust- rustpp && cd rustpp
python rustpp/setup-deps.py
python x.py build library src/tools/rustdoc
```

## License

MIT or Apache 2.0, like Rust: [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE), [`COPYRIGHT`](COPYRIGHT). Rust++ is not
affiliated with the Rust Project or the Rust Foundation; "Rust" is their
trademark. The Rust++ changes are written with the help of an AI assistant.
