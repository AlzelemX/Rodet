# Projects with Cargo

`rustc` compiles one file. For real programs use **Cargo**: it makes the
project, builds it, runs it, and downloads libraries.

```text
> cargo new hello
> cd hello
```

```text
hello/
├── Cargo.toml      the project: name, version, libraries
└── src/
    └── main.rs     your code
```

Change `src/main.rs`:

```rust
fn main() {
    out!("Hello from Cargo!");
}
```

## Commands

| Command | Does |
|---|---|
| `cargo run` | build and run |
| `cargo build` | build only (into `target/debug/`) |
| `cargo build --release` | build optimized, fast program (into `target/release/`) |
| `cargo check` | check for errors without building — fastest |
| `cargo test` | run the tests |

Always measure speed with `--release`: debug builds can be 10–50 times
slower.

## Libraries (crates)

Libraries are called *crates*. Add one to `Cargo.toml` under
`[dependencies]`:

```toml
[dependencies]
rand = "0.8"
```

Cargo downloads it the next time you build. Crates written for Rust work in
Rodet.
