# Projects with Silk

`rodetc` compiles one file. For real programs use **Silk**, Rodet's package
manager: its command is `rodet`. It makes the project, builds it, runs it,
and downloads libraries.

```text
> rodet new hello
> cd hello
```

```text
hello/
├── Silk.toml       the project: name, version, libraries
└── src/
    └── main.rd     your code
```

Rodet files end in `.rd`. Change `src/main.rd`:

```rust
fn main() {
    out!("Hello from Silk!");
}
```

## Commands

| Command | Does |
|---|---|
| `rodet run` | build and run |
| `rodet build` | build only (into `target/debug/`) |
| `rodet build --release` | build optimized, fast program (into `target/release/`) |
| `rodet check` | check for errors without building — fastest |
| `rodet test` | run the tests |
| `rodet add rand` | add a library |

Always measure speed with `--release`: debug builds can be 10–50 times
slower.

## Libraries

Add a library with `rodet add`, or write it in `Silk.toml` under
`[dependencies]`:

```toml
[dependencies]
rand = "0.8"
```

Silk downloads it the next time you build, and writes the exact versions in
`Silk.lock`. Libraries come from crates.io, so every library written for Rust
works in Rodet too (Silk reads their `Cargo.toml` and `.rs` files).

## Files and folders

| | |
|---|---|
| `Silk.toml` | your project |
| `Silk.lock` | the exact library versions (keep it in git for programs) |
| `src/main.rd` | a program; `src/lib.rd` is a library |
| `src/bin/*.rd`, `examples/*.rd`, `tests/*.rd` | more programs, examples, tests |
| `build.rd` | runs before the build, if it exists |
| `~/.rodet` | Rodet itself and the downloaded libraries (`RODET_HOME` changes it) |
| `.rodet/config.toml` | settings for one project |
