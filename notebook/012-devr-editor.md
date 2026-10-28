# `DEVR+` | The Editor For Rodet

> 📅 **2026-10-23** · 🟢 Done (v0.1) · 🏷️ Tooling

DEVR+ (DEV + R+) is a light editor made only for Rodet. It looks and works
like VS Code, so there is nothing new to learn, but everything in it is
about the language: about 10 MB instead of hundreds.

### Before

```text
VS Code + rust-analyzer: red lines under every Rodet feature,
nothing knows about out!, &{x}, _a, named arguments or default values.
```

### After

```text
DEVR+
├── Explorer, tabs, Search in files, Go to File (Ctrl+P), Commands (Ctrl+Shift+P)
├── Find Symbols: struct / enum / trait / fn / impl … (Ctrl+T, Ctrl+Shift+T)
├── Errors and warnings as you type, compiler errors on save, quick fixes (Ctrl+.)
├── Completion, hover, go to definition (F12), references, rename (F2)
├── ↩ marks after returned values
├── ⚡ Speed score 0–100 next to Run, with tips
├── Cargo: Run (F5), Build, Test, Run fast, libraries and their new versions
└── Terminal, with clickable file:line errors
```

### Download

From the [releases](https://github.com/AlzelemX/rodet/releases) named
**DEVR+**: `.exe` for Windows, `.deb` / `.AppImage` for Linux, `.dmg` for
macOS. DEVR+ uses the Rodet installed by `rodet_init`, and offers to
install it when it's missing.

### Rules

- Built with Tauri (Rust backend, system web view) and Monaco (the code
  editor inside VS Code): both MIT licensed.
- The same editor runs in a browser with `devr-server`, the first step
  towards phones and tablets.
- Extensions are planned for later (for extras such as dependency graphs).

<details><summary>Where</summary>

- `devr/core/`: files, terminals, search, Cargo, the language server
- `devr/ui/`: the editor (TypeScript + Monaco + xterm.js)
- `devr/app/`: the desktop app (Tauri)
- `devr/server/`: the editor in a browser
- `.github/workflows/devr.yml`: builds the installers with Rodet itself

</details>
