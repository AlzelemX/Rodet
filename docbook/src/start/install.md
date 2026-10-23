# Install Rust++

## Windows

1. Download **`rustup++_init.exe`** from the
   [latest release](https://github.com/AlzelemX/Rust-/releases/latest).
2. Double-click it. It installs Rust++ into `%USERPROFILE%\.rustpp` and adds
   it to your PATH.
3. If the Visual Studio C++ Build Tools are missing, it offers to install
   them. Say yes: Windows programs are linked with Microsoft's linker.

## Linux and macOS

```sh
curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh
```

You also need a C linker: `xcode-select --install` on macOS, or
`build-essential` (Ubuntu/Debian) / `gcc` on Linux.

## Check it works

Open a **new** terminal:

```text
> rustc --version
rustc 1.101.0-dev (Rust++ #19)
> cargo --version
cargo 1.100.0-beta.1 (3d7cf6e93 2026-09-25)
```

If `(Rust++ #…)` is there, you’re ready. The number is the build: a newer release has a bigger number.

## Update and uninstall

- **Update:** run the installer again.
- **Uninstall:** `rustup++_init.exe --uninstall` on Windows, or

  ```sh
  curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh -s -- --uninstall
  ```

## Editor

Use **[DEVR+](devr.md)**, the editor made for Rust++: it knows all the new
syntax, shows errors as you type and runs your project with `F5`.

Rust++ also comes with **`rust++-analyzer`**, the language server DEVR+ uses.
Other editors can use it too: in VS Code, install the *rust-analyzer*
extension and set `rust-analyzer.server.path` to the `rust++-analyzer` in
`~/.rustpp/bin` (`%USERPROFILE%\.rustpp\bin` on Windows).
