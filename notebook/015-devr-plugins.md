# `Ctrl+Shift+X` | DEVR+ Plugins

> 📅 **2026-10-24** · 🟢 Done · 🏷️ Tooling

DEVR+ has plugins, and they can change everything: commands and shortcuts,
side views, panels, status bar items, pages, colors, the whole look, the
code editor, and they can use the language server, the terminal and files.

### Before

```text
DEVR+ did what it shipped with. New colors or a new panel = a new DEVR+ release.
```

### After

```text
Extensions (Ctrl+Shift+X)
├── Create a Plugin   → a folder from a template, opened in the editor
├── Install from a Folder…
└── each plugin: on/off · Reload · Edit · Remove

my-plugin/
├── plugin.json   { "id", "name": { "en", "ar" }, "main", "styles", "themes" }
├── main.js       export function activate(devr) { … }
├── style.css     CSS for the whole editor
└── devr.d.ts     the whole API, explained
```

```js
export function activate(devr) {
  devr.commands.register({ id: 'me.hi', title: 'Say Hi', key: 'Ctrl+Alt+H',
    run: () => devr.ui.toast(devr.t('Hi!')) });
  devr.theme.setColors({ '--accent': '#2f9e44' });
}
```

### Rules

- Save a file of a plugin and it reloads; everything it added is undone when
  it stops (on/off, reload, remove) — no restart.
- A plugin can be only a theme and CSS, with no code.
- Plugins are bilingual: `name` / `description` as `{ "en", "ar" }`, texts
  through `devr.t` and `devr.addTranslations`.
- A plugin runs with the editor's full power: install only trusted ones.
  A plugin that fails to start is turned off and shows why.

<details><summary>Where</summary>

- `devr/core/src/plugins.rs`: list, read, install, create, remove; saved data
- `devr/ui/src/plugins/host.ts`: starts and stops plugins, styles, themes
- `devr/ui/src/plugins/api.ts`: the `devr` object
- `devr/ui/src/views/plugins.ts`: the Extensions view
- `devr/ui/src/theme.ts`: themes from plugins (workbench, code, terminal)
- `devr/plugin-api/devr.d.ts`: the API for plugin authors; `examples/`
- Doc Book: *The DEVR+ Editor → Plugins*

</details>
