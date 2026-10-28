# The DEVR+ Editor

**DEVR+** is the editor made for Rodet. It looks like VS Code, so
everything is where you expect it, but it understands every Rodet feature.

## Install

Download DEVR+ from the
[releases named “DEVR+”](https://github.com/AlzelemX/rodet/releases):

| System | File |
|---|---|
| Windows | `DEVR+_…_x64-setup.exe` |
| Linux (glibc 2.39+: Ubuntu 24.04+, Debian 13+) | `.deb` or `.AppImage` |
| macOS (Apple Silicon / Intel) | `DEVR+_…_aarch64.dmg` / `DEVR+_…_x64.dmg` |

DEVR+ uses the Rodet you installed with `rodet_init`. If it's missing,
the welcome page has an **Install Rodet** button.

> macOS: the app isn't signed by Apple yet. The first time, right-click it
> and choose **Open**.

## Arabic

In the settings (`Ctrl+,`) choose **Language: العربية**. The whole editor
switches to Arabic: menus and commands, compiler messages, quick fixes, speed
tips, the documentation of the standard library, and this book. Code itself
stays left to right.

The Arabic lives in Rodet itself, not only in the editor:

- **Compiler messages.** `rodetc` and `rodet` speak Arabic when the
  environment variable `RODET_LANG` is `ar` (DEVR+ sets it for you, also in
  its terminal). From any terminal: `RODET_LANG=ar rodet build`
  (PowerShell: `$env:RODET_LANG="ar"; rodet build`). Names, types and code
  in the messages stay as they are.
- **Standard library docs.** Every documented item of `std`, `core` and
  `alloc` has an Arabic explanation next to the English one, in the same doc
  comment. `Option`, `Result`, `String`, `Vec`, `HashMap` and the print
  macros have the whole text in Arabic, the rest a short summary for now.
  The analyzer shows the language you chose (setting `rodet.language`:
  `en` or `ar`); code examples are shown in both.

## A first project

1. **New Rodet Project…** on the welcome page (or `Ctrl+Shift+P` → *Rodet: New Project*).
2. Press **F5**: Silk builds and runs it in the terminal at the bottom.
3. Errors appear as you type; a red file:line in the terminal opens the file
   at that line.

## What you see

![The DEVR+ editor](devr.png)

- **↩ return marks**: after the value a function gives back.
- **⚡ Speed score**: next to the Run button, from 0 to 100. Point at it to see
  each slow spot and how to fix it. Slow spots are underlined with dots.
- **▶ Run** above `fn main`, and **N implementations** above types.
- Gray **hints**: the types Rodet works out (`: u32`), and parameter names
  for calls that don't use [named arguments](../basics/named-arguments.md).
- **Doc Book** (the book icon on the left): this book inside the editor, in
  English or Arabic. It comes with the installed Rodet, so it always matches
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

## The Silk view

The package icon on the left shows your project: **Run**, **Build**,
**Check**, **Test**, **Run fast** and **Clean**, the programs in it, and its
libraries. A library with a newer version on crates.io shows it in green;
**+** adds a library (`rodet add`).

## In a browser

`devr-server` serves the same editor to a web browser:

```text
devr-server --ui path/to/devr/ui/dist
```

Then open <http://127.0.0.1:7878>. It only listens on your own computer.
