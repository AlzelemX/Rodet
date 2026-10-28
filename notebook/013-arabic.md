# `العربية` | DEVR+ And The Doc Book In Arabic

> 📅 **2026-10-23** · 🟢 Done · 🏷️ Tooling, Docs

DEVR+ speaks Arabic, and the Doc Book has an Arabic edition that ships with
every Rodet release and opens inside the editor.

### Before

```text
DEVR+ and the Doc Book: English only.
The Doc Book: only on the website, not matched to the installed version.
```

### After

```text
Settings (Ctrl+,) → Language: العربية
├── the whole editor: menus, commands, panels, dialogs, the code editor's own menus
├── compiler and analyzer messages: the common errors, notes and quick fixes
├── speed tips, Run / implementations lenses
├── hover: an Arabic description of the most used std items (Vec::push, Option::unwrap…)
└── Doc Book tab (book icon): Arabic or English, from the installed Rodet
```

Code stays left to right; only text around it is right to left.

### The Doc Book follows the language

Every Rodet release now carries its book in
`share/doc/rodet/book/{en,ar}`. DEVR+ reads the book of the Rodet that is
installed, so updating the language updates the book. Online:
<https://alzelemx.github.io/rodet/ar/>.

### Also in this update

- Errors clear as soon as the code is fixed (auto save is on, and compiler
  errors on edited lines disappear right away).
- The completion box no longer opens empty: messages to the language server
  now always arrive in order.
- `Cargo.toml` is no longer read as Rust (it showed "Syntax Error" on every line).
- The speed score is a small ⚡ item next to Run; point at it for the tips.
- The welcome page lost its "Rodet in one minute" list (the Doc Book
  explains the language and stays up to date).

<details><summary>Where</summary>

- `devr/ui/src/i18n/`: `ar.ts` (interface), `messages.ts` (compiler messages),
  `std-ar.ts` (std descriptions), `monaco-ar.json` (the code editor's texts)
- `devr/ui/src/views/book.ts`, `devr/core/src/book.rs`: the Doc Book tab
- `docbook/ar/`: the Arabic Doc Book
- `.github/workflows/build-rodet.yml`: puts both books in every release

</details>
