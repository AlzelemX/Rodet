# The DEVR+ Editor

**DEVR+** is the editor made for Rust++. It looks like VS Code, so
everything is where you expect it, but it understands every Rust++ feature.

## Install

Download DEVR+ from the
[releases named “DEVR+”](https://github.com/AlzelemX/Rust-/releases):

| System | File |
|---|---|
| Windows | `DEVR+_…_x64-setup.exe` |
| Linux (glibc 2.39+: Ubuntu 24.04+, Debian 13+) | `.deb` or `.AppImage` |
| macOS (Apple Silicon / Intel) | `DEVR+_…_aarch64.dmg` / `DEVR+_…_x64.dmg` |

DEVR+ uses the Rust++ you installed with `rustup++_init`. If it's missing,
the welcome page has an **Install Rust++** button.

> macOS: the app isn't signed by Apple yet. The first time, right-click it
> and choose **Open**.

## Arabic

In the settings (`Ctrl+,`) choose **Language: العربية**. The whole editor
switches to Arabic: menus and commands, the common compiler messages, quick
fixes, speed tips, the descriptions of the most used standard library items,
and this book. Code itself stays left to right.

## A first project

1. **New Rust++ Project…** on the welcome page (or `Ctrl+Shift+P` → *Cargo: New*).
2. Press **F5**: Cargo builds and runs it in the terminal at the bottom.
3. Errors appear as you type; a red file:line in the terminal opens the file
   at that line.

## What you see

![The DEVR+ editor](devr.png)

- **↩ return marks**: after the value a function gives back.
- **⚡ Speed score**: next to the Run button, from 0 to 100. Point at it to see
  each slow spot and how to fix it. Slow spots are underlined with dots.
- **▶ Run** above `fn main`, and **N implementations** above types.
- Gray **hints**: the types Rust++ works out (`: u32`), and parameter names
  for calls that don't use [named arguments](../basics/named-arguments.md).
- **Doc Book** (the book icon on the left): this book inside the editor, in
  English or Arabic. It comes with the installed Rust++, so it always matches
  your version of the language.

## Keys

| Key | Does |
|---|---|
| `Ctrl+P` | open a file by name |
| `Ctrl+Shift+P` | all commands |
| `Ctrl+T` | go to a struct, trait, function… in the project |
| `Ctrl+Shift+T` | **Find Symbols** panel with filters: struct, enum, trait, fn, impl… |
| `Ctrl+Shift+F` | search in all files |
| `F12` / `Shift+F12` | go to definition / all references |
| `Ctrl+F12` | go to the `impl` blocks of a type or trait |
| `F2` | rename everywhere |
| `Ctrl+.` | quick fix |
| `F8` | next error |
| `F5` / `Ctrl+F5` | run / run fast (`--release`) |
| `Ctrl+Shift+B` | build |
| `` Ctrl+` `` | new terminal |
| `Ctrl+B` / `Ctrl+J` | show or hide the side bar / the bottom panel |
| `Ctrl+,` | settings (language, theme, font, hints, speed score…) |

On macOS use `⌘` instead of `Ctrl`.

## The Cargo view

The package icon on the left shows your project: **Run**, **Build**,
**Check**, **Test**, **Run fast** and **Clean**, the programs in it, and its
libraries. A library with a newer version on crates.io shows it in green;
**+** adds a library (`cargo add`).

## In a browser

`devr-server` serves the same editor to a web browser:

```text
devr-server --ui path/to/devr/ui/dist
```

Then open <http://127.0.0.1:7878>. It only listens on your own computer.
