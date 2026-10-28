# `rodet_init` | One-Click Installers

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

Rodet installs with one file, without `rustup`. After installing, `rustc`
and `cargo` in your terminal are Rodet.

### Before (Rust)

```text
Install rustup → rustup toolchain link ... → rustup default ...
```

### After (Rodet)

Windows: download `rodet_init.exe` and double-click it.

Linux / macOS:

```sh
curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh
```

```text
> rustc --version
rustc 1.101.0-dev (Rodet)
```

### Rules

- Installs into `%USERPROFILE%\.rodet` (Windows) or `~/.rodet` (Linux/macOS)
  and puts its `bin` first in your PATH.
- Run it again to update. `--uninstall` removes Rodet.
- Windows: offers to install the Visual Studio C++ Build Tools if they are
  missing (needed to link programs).

<details><summary>Where</summary>

- `rodet/installer/main.rs`: `rodet_init.exe`
- `rodet/installer/install.sh`: `rodet_init.sh`

</details>
