# DEVR+ Plugins

A plugin can change anything in DEVR+: add commands and shortcuts, views,
panels, status bar items and pages, change the colors and the whole look,
teach the code editor new tricks, and talk to the language server, the
terminal and your files.

## Use plugins

Open **Extensions** (`Ctrl+Shift+X`, the puzzle icon). Each plugin has a
switch to turn it on or off, **Reload**, **Edit** and **Remove**.

- **Install from a Folder…**: a folder with a `plugin.json`.
- **Create a Plugin**: makes a new plugin from a template and opens it.

> A plugin can do anything DEVR+ can, including reading and changing your
> files and running programs. Install only plugins you trust.

Plugins are kept in the DEVR+ settings folder, in `plugins/`.

## Write a plugin

**Create a Plugin** makes a folder like this:

```text
my-plugin/
├── plugin.json   what the plugin is
├── main.js       its code
├── style.css     CSS for the whole editor
└── devr.d.ts     the whole API, with explanations (for completion)
```

`plugin.json`:

```json
{
  "id": "my-plugin",
  "name": { "en": "My Plugin", "ar": "إضافتي" },
  "version": "0.1.0",
  "description": { "en": "What it does.", "ar": "ماذا تفعل." },
  "main": "main.js",
  "styles": ["style.css"],
  "themes": ["themes/night.json"],
  "devr": "1"
}
```

Only `id` is required. A plugin can be only a theme and CSS, with no code.

`main.js` exports `activate`, which gets the editor as `devr`:

```js
// @ts-check
/** @param {import('./devr').Devr} devr */
export function activate(devr) {
  devr.addTranslations({ 'Hello!': 'مرحبًا!' });

  devr.commands.register({
    id: 'my-plugin.hello',
    title: 'My Plugin: Say Hello',
    key: 'Ctrl+Alt+H',
    run: () => devr.ui.toast(devr.t('Hello!'), 'success'),
  });

  const lines = devr.ui.addStatusItem({ icon: 'list-ordered', align: 'right' });
  devr.editor.onChange(() => lines.set(String(devr.editor.model?.getLineCount() ?? '')));
}
```

**Save a file of the plugin and it reloads**: change, save, see. Everything
the plugin added is removed when it stops or reloads; for things you make
with Monaco directly, add them to `devr.subscriptions`.

## What `devr` has

| Part | For |
|---|---|
| `devr.commands`, `devr.keys` | commands in the palette, shortcuts (also for DEVR+'s own commands) |
| `devr.ui` | toasts, dialogs, quick picks, menus, side views, panels, status items, pages |
| `devr.layout` | every part of the window (activity bar, side bar, tabs, panel, status bar) |
| `devr.theme` | color themes, CSS, colors (`setColors({ '--accent': '#2f9e44' })`) |
| `devr.editor` | the code editor and the whole Monaco API |
| `devr.workspace` | open, read, write and save files; events when files open, save, close |
| `devr.terminal` | run programs and `cargo` |
| `devr.lsp` | requests to rodet-analyzer, errors and warnings |
| `devr.settings`, `devr.storage` | the editor's settings; your plugin's own saved data |
| `devr.backend` | everything the DEVR+ backend can do |
| `devr.t`, `devr.addTranslations` | your texts in English and Arabic |

## Themes

A theme file sets the workbench colors (CSS variables), and if you want the
code editor's and the terminal's:

```json
{
  "name": { "en": "Night", "ar": "الليل" },
  "base": "dark",
  "colors": { "--bg": "#0b1622", "--accent": "#38bdf8" },
  "editor": { "rules": [{ "token": "keyword", "foreground": "38bdf8" }] },
  "terminal": { "background": "#08111b" }
}
```

It then shows in **Settings → Theme**.

## Examples

In the Rodet repository, `devr/plugin-api/examples/`:

- **ocean**: a theme and a rounded look, no code.
- **todos**: a side view with every TODO and FIXME, a command and a status item.
