# `rustup++_init` | One-Click Installers

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

Rust++ installs with one file, without `rustup`. After installing, `rustc`
and `cargo` in your terminal are Rust++.

### Before (Rust)

```text
Install rustup → rustup toolchain link ... → rustup default ...
```

### After (Rust++)

Windows: download `rustup++_init.exe` and double-click it.

Linux / macOS:

```sh
curl -fsSL https://github.com/AlzelemX/Rust-/releases/latest/download/rustup++_init.sh | sh
```

```text
> rustc --version
rustc 1.101.0-dev (Rust++)
```

### Rules

- Installs into `%USERPROFILE%\.rustpp` (Windows) or `~/.rustpp` (Linux/macOS)
  and puts its `bin` first in your PATH.
- Run it again to update. `--uninstall` removes Rust++.
- Windows: offers to install the Visual Studio C++ Build Tools if they are
  missing (needed to link programs).

<details><summary>Where</summary>

- `rustpp/installer/main.rs`: `rustup++_init.exe`
- `rustpp/installer/install.sh`: `rustup++_init.sh`

</details>
