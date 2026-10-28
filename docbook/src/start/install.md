# Install Rodet

## Windows

1. Download **`rodet_init.exe`** from the
   [latest release](https://github.com/AlzelemX/rodet/releases/latest).
2. Double-click it. It installs Rodet into `%USERPROFILE%\.rodet` and adds
   it to your PATH.
3. If the Visual Studio C++ Build Tools are missing, it offers to install
   them. Say yes: Windows programs are linked with Microsoft's linker.

## Linux and macOS

```sh
curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh
```

You also need a C linker: `xcode-select --install` on macOS, or
`build-essential` (Ubuntu/Debian) / `gcc` on Linux.

## Check it works

Open a **new** terminal:

```text
> rodet --version
rodet 0.1.0 (Silk)
> rodetc --version
rodetc 0.1.0 (#23, 2026-10-28)
```

If both answer, you’re ready. The `#` number is the build: a newer release has a bigger number.

## Update and uninstall

- **Update:** run the installer again.
- **Uninstall:** `rodet_init.exe --uninstall` on Windows, or

  ```sh
  curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh -s -- --uninstall
  ```

## Editor

Use **[DEVR+](devr.md)**, the editor made for Rodet: it knows all the new
syntax, shows errors as you type and runs your project with `F5`.

Rodet also comes with **`rodet-analyzer`**, the language server DEVR+ uses.
Other editors can use it too: in VS Code, install the *rust-analyzer*
extension and set `rust-analyzer.server.path` to the `rodet-analyzer` in
`~/.rodet/bin` (`%USERPROFILE%\.rodet\bin` on Windows).
