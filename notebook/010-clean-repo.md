# `cleanup` | Clean Repository And Own Docs

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Project

The repository now holds only what Rodet uses, and documents itself in its
own way.

### Before

```text
README.md            upstream's README + notes
AGENTS.md            rust-lang's rules for contributors
CODE_OF_CONDUCT.md   CONTRIBUTING.md   INSTALL.md   RELEASES.md
triagebot.toml       rust-bors.toml    typos.toml   .mailmap ...
.github/ISSUE_TEMPLATE/  FUNDING.yml (rustfoundation)  renovate.json5
src/ci/              upstream's CI (Docker images, scripts)
src/doc/             upstream's books (rustc book, style guide, ...)
```

### After

```text
README.md     short description + status board
STATUS.md     the status board (edit this one)
notebook/     one card per change (this file)
docbook/      the Doc Book: how to write Rodet
rodet/       setup, installers, scripts
compiler/  library/  src/  tests/   the language itself
LICENSE-*  COPYRIGHT  LICENSES/  REUSE.toml   kept: required by the license
```

### Rules

- **Status:** edit `STATUS.md`; the table at the top of `README.md` updates
  itself after the push.
- **NoteBook:** for every change, copy `notebook/TEMPLATE.md` to
  `notebook/NNN-name.md`, fill it in, and add a row to `notebook/README.md`.
- **Doc Book:** edit the pages in `docbook/src/`; the site updates itself.

<details><summary>Where</summary>

- `rodet/sync-status.py`, `.github/workflows/readme-status.yml`: status board
- `.github/workflows/docbook.yml`: publishes the Doc Book
- `src/bootstrap/src/core/builder/mod.rs`: lets the build run without the removed files

</details>
