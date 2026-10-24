// The `devr` object a plugin gets in `activate(devr)`: the whole editor.
//
// Everything that adds something (a command, a key, a view, a style, a
// theme…) is recorded on the plugin, and undone when the plugin stops.
// Plugins also get the raw parts (Monaco, the layout's elements, the
// backend), so nothing in DEVR+ is out of their reach.
// The types for plugin authors: devr/plugin-api/devr.d.ts (keep it in sync).
import { backend } from '../backend';
import * as monaco from '../editor/monaco';
import { addTranslations, arabic, language, t } from '../i18n';
import { diagnostics, lsp, onDiagnostics } from '../lsp/features';
import { onSettings, settings, updateSettings, type Settings } from '../settings';
import { currentTheme, onThemeApplied, registerTheme, themeName, themes, type Theme } from '../theme';
import { contextMenu, layout, type MenuItem, type PanelView, type SideView } from '../ui/layout';
import { confirm, prompt, showDialog, toast, type Action, type Level } from '../ui/notify';
import { bindKey, commands, keybindings, pick, registerCommand, runCommand, type Command, type PickItem } from '../ui/palette';
import { h, icon } from '../util';
import { openBook } from '../views/book';
import { newTerminal, runCargo, runTask } from '../views/terminal';
import { pathKey, workspace, type Reveal } from '../workspace';
import { API_VERSION, type Plugin } from './host';

export interface Disposable {
  dispose(): void;
}

// ---- editor options from plugins (kept when the settings change) ----

const editorOptionLayers = new Map<symbol, monaco.editor.IEditorOptions>();

/** The editor options plugins set, applied over DEVR+'s own. */
export function pluginEditorOptions(): monaco.editor.IEditorOptions {
  return Object.assign({}, ...editorOptionLayers.values());
}

// ---- pages: tabs in the editor area with a plugin's own content ----

const PAGE_PREFIX = 'devr-plugin:';
const pageElements = new Map<string, HTMLElement>();
let pagesWired = false;

function wirePages() {
  if (pagesWired) return;
  pagesWired = true;
  workspace.onActive.on((path) => {
    for (const [key, el] of pageElements) el.classList.toggle('hidden', key !== path);
  });
}

export function createApi(plugin: Plugin) {
  /** Records an undo function; returns it as a Disposable (and calls it once). */
  const track = (undo: () => void): Disposable => {
    let done = false;
    const dispose = () => {
      if (done) return;
      done = true;
      const i = plugin.disposers.indexOf(dispose);
      if (i >= 0) plugin.disposers.splice(i, 1);
      undo();
    };
    plugin.disposers.push(dispose);
    return { dispose };
  };
  const ownTable = new Map<string, string>();

  const api = {
    /** The plugin API version (`"1"`). */
    version: API_VERSION,
    /** This plugin: id, name, version and folder. */
    plugin: { id: plugin.id, name: plugin.manifest.name, version: plugin.manifest.version ?? '', path: plugin.path },
    /** DEVR+ and the computer: version, os, home folder, Rust++ toolchain. */
    get app() {
      return workspace.info;
    },
    /** The editor's language: `en` or `ar`. */
    language,
    arabic,

    /** Text in the editor's language: English in, Arabic out when there is a translation. */
    t(english: string, vars?: Record<string, string | number>): string {
      if (arabic && ownTable.has(english)) {
        let text = ownTable.get(english)!;
        if (vars) text = text.replace(/\{(\w+)\}/g, (m, key) => (key in vars ? String(vars[key]) : m));
        return text;
      }
      return t(english, vars);
    },
    /** Arabic for this plugin's texts: `{ 'Hello': 'مرحبًا' }`. */
    addTranslations(table: Record<string, string>): Disposable {
      for (const [en, ar] of Object.entries(table)) ownTable.set(en, ar);
      // Also global, so texts shown by DEVR+ itself (menus, palette) are translated.
      return track(addTranslations(table));
    },

    /** Things to undo when the plugin stops: Disposables (Monaco's too) or functions. */
    subscriptions: {
      push(...items: (Disposable | (() => void))[]) {
        for (const item of items) track(typeof item === 'function' ? item : () => item.dispose());
      },
    },

    commands: {
      /** Adds a command to the palette (Ctrl+Shift+P); `key` also binds a shortcut. */
      register(command: Command): Disposable {
        const remove = registerCommand(command);
        const unbind = command.key ? bindKey(command.key, command.id) : () => {};
        plugin.added.commands.add(command.id);
        return track(() => {
          unbind();
          remove();
          plugin.added.commands.delete(command.id);
        });
      },
      run: runCommand,
      list: () => [...commands.values()],
    },

    keys: {
      /** `bind('Ctrl+Alt+R', 'cargo.run')`: also replaces DEVR+'s own shortcuts. */
      bind: (combo: string, commandId: string): Disposable => track(bindKey(combo, commandId)),
      list: () => [...keybindings.entries()].map(([key, command]) => ({ key, command })),
    },

    editor: {
      /** The whole Monaco API: languages, providers, decorations, themes… */
      monaco,
      /** The code editor (one for all tabs). */
      get current() {
        return workspace.editor;
      },
      /** The text model of the open file, or `null`. */
      get model() {
        return workspace.editor.getModel();
      },
      get text() {
        return workspace.editor.getModel()?.getValue() ?? '';
      },
      get selection() {
        return workspace.editor.getSelection();
      },
      /** Replaces the selection (or inserts at the cursor), as one undo step. */
      insert(text: string) {
        const ed = workspace.editor;
        const selection = ed.getSelection();
        if (!selection) return;
        ed.executeEdits(plugin.id, [{ range: selection, text, forceMoveMarkers: true }]);
        ed.pushUndoStop();
      },
      /** Editor options over DEVR+'s own (`{ fontSize: 16, cursorStyle: 'block' }`). */
      setOptions(options: monaco.editor.IEditorOptions): Disposable {
        const key = Symbol(plugin.id);
        editorOptionLayers.set(key, options);
        workspace.editor.updateOptions(options);
        return track(() => {
          editorOptionLayers.delete(key);
          onSettings.fire(settings); // Re-applies DEVR+'s options.
        });
      },
      onChange: (cb: (e: monaco.editor.IModelContentChangedEvent) => void): Disposable => {
        const d = workspace.editor.onDidChangeModelContent(cb);
        return track(() => d.dispose());
      },
      onCursor: (cb: (e: monaco.editor.ICursorSelectionChangedEvent) => void): Disposable => {
        const d = workspace.editor.onDidChangeCursorSelection(cb);
        return track(() => d.dispose());
      },
      /** Runs one of Monaco's actions (`editor.action.formatDocument`…). */
      action(id: string) {
        workspace.editor.focus();
        return workspace.editor.getAction(id)?.run();
      },
    },

    workspace: {
      /** The open folder, or `null`. */
      get root() {
        return workspace.root;
      },
      /** The path of the active tab, or `null`. */
      get active() {
        return workspace.active;
      },
      /** The open tabs. */
      get tabs() {
        return [...workspace.tabs];
      },
      open: (path: string, reveal?: Reveal) => workspace.open(path, reveal),
      openFolder: (path: string) => workspace.openFolder(path),
      save: (path?: string) => workspace.save(path ?? workspace.active),
      saveAll: () => workspace.saveAll(),
      close: (path: string) => workspace.close(path),
      isDirty: (path: string) => workspace.isDirty(path),
      /** All files of the open folder (without build output and `.git`). */
      files: async () => (workspace.root ? backend.call<string[]>('fs.listFiles', { path: workspace.root }) : []),
      readFile: async (path: string) => {
        const open = workspace.get(path);
        if (open) return open.model.getValue();
        return (await backend.call<{ text: string }>('fs.readFile', { path })).text;
      },
      writeFile: (path: string, text: string) => backend.call('fs.writeFile', { path, text }),
      readDir: (path: string) => backend.call<{ name: string; path: string; isDir: boolean }[]>('fs.readDir', { path }),
      onRoot: (cb: (root: string | null) => void): Disposable => track(workspace.onRoot.on(cb)),
      onActive: (cb: (path: string | null) => void): Disposable => track(workspace.onActive.on(cb)),
      onOpen: (cb: (path: string, model: monaco.editor.ITextModel) => void): Disposable =>
        track(workspace.onModelOpened.on((doc) => cb(doc.path, doc.model))),
      onSave: (cb: (path: string, model: monaco.editor.ITextModel) => void): Disposable =>
        track(workspace.onSaved.on((doc) => cb(doc.path, doc.model))),
      onClose: (cb: (path: string) => void): Disposable => track(workspace.onModelClosed.on((doc) => cb(doc.path))),
    },

    ui: {
      /** Makes elements: `h('div.card', { onclick }, 'text', child)`. */
      h,
      /** A VS Code icon (codicon) by name: `icon('rocket')`. */
      icon,
      toast: (message: string, level: Level = 'info', actions: Action[] = [], timeout?: number) =>
        toast(message, level, actions, timeout),
      confirm: (title: string, detail?: string, okLabel?: string, danger?: boolean) => confirm(title, detail, okLabel, danger),
      prompt: (title: string, value?: string, options?: Parameters<typeof prompt>[2]) => prompt(title, value, options),
      /** A quick pick at the top, like the command palette. */
      pick: (items: PickItem[], placeholder = '') => pick({ placeholder, items }),
      /** A dialog with any content; returns its close function. */
      dialog: (title: string, body: HTMLElement, wide = false) => {
        const close = showDialog(title, body, wide);
        track(close);
        return close;
      },
      contextMenu: (event: MouseEvent, items: (MenuItem | 'sep')[]) => contextMenu(event, items),
      /** A view in the side bar, with a button in the activity bar. */
      addSideView(view: SideView): Disposable {
        layout.addSideView(view);
        plugin.added.views.add(view.title);
        return track(() => {
          layout.removeSideView(view.id);
          plugin.added.views.delete(view.title);
        });
      },
      /** A tab in the bottom panel (next to Problems, Output, Terminal). */
      addPanel(view: PanelView): Disposable {
        layout.addPanelView(view);
        plugin.added.views.add(view.title);
        return track(() => {
          layout.removePanelView(view.id);
          plugin.added.views.delete(view.title);
        });
      },
      /** A button at the bottom of the activity bar. */
      addActivityButton(title: string, iconName: string, run: () => void): Disposable {
        const button = layout.addActivityButton(title, iconName, run);
        return track(() => button.remove());
      },
      /** An item in the status bar. */
      addStatusItem(options: { text?: string; icon?: string; tooltip?: string; align?: 'left' | 'right'; onClick?: () => void } = {}) {
        const el = h('button.status-item', { title: options.tooltip ?? '', onclick: () => options.onClick?.() });
        const set = (text: string, iconName = options.icon) => {
          el.replaceChildren(...(iconName ? [icon(iconName)] : []), text);
          el.style.display = text || iconName ? '' : 'none';
        };
        set(options.text ?? '');
        (options.align === 'right' ? layout.statusRight : layout.statusLeft).appendChild(el);
        const { dispose } = track(() => el.remove());
        return {
          element: el,
          set,
          setTooltip: (tooltip: string) => {
            el.title = tooltip;
          },
          dispose,
        };
      },
      /** A tab in the editor area with your own content (like the Doc Book). */
      openPage(page: { id: string; title: string; icon?: string; element: HTMLElement }): Disposable {
        wirePages();
        const key = `${PAGE_PREFIX}${plugin.id}/${page.id}`;
        let el = pageElements.get(key);
        if (!el) {
          el = h('div.plugin-page.hidden', {}, page.element);
          pageElements.set(key, el);
          layout.editorHost.appendChild(el);
        }
        workspace.openPage(key, page.title, page.icon ?? 'extensions');
        return track(() => {
          if (workspace.tabs.some((tab) => pathKey(tab) === pathKey(key))) workspace.close(key);
          pageElements.get(key)?.remove();
          pageElements.delete(key);
        });
      },
      showSide: (id: string) => layout.showSide(id),
      showPanel: (id: string) => layout.showPanel(id),
      toggleSidebar: () => layout.toggleSidebar(),
      togglePanel: (show?: boolean) => layout.togglePanel(show),
    },

    /** The workbench's elements (activity bar, side bar, tabs, panel, status bar…): change anything. */
    layout,

    theme: {
      /** Adds a color theme to the settings. */
      register(theme: Theme): Disposable {
        const remove = registerTheme(theme);
        plugin.added.themes.add(theme.id);
        return track(() => {
          remove();
          plugin.added.themes.delete(theme.id);
        });
      },
      /** Switches to a theme (`dark`, `light`, or a registered one). */
      use: (id: string) => updateSettings({ theme: id }),
      get current() {
        return currentTheme();
      },
      list: () => [...themes.values()].map((th) => ({ id: th.id, name: themeName(th), base: th.base })),
      /** CSS for the whole editor, removed when the plugin stops. */
      addStyle(css: string): Disposable {
        const style = document.createElement('style');
        style.dataset.plugin = plugin.id;
        style.textContent = css;
        document.head.appendChild(style);
        return track(() => style.remove());
      },
      /** Changes workbench colors: `setColors({ '--accent': '#2f9e44' })`. */
      setColors(colors: Record<string, string>): Disposable {
        const vars = Object.entries(colors).map(([name, value]) => `  ${name.startsWith('--') ? name : `--${name}`}: ${value};`);
        return api.theme.addStyle(`:root[data-theme] {\n${vars.join('\n')}\n}`);
      },
      onChange: (cb: (theme: Theme) => void): Disposable => track(onThemeApplied.on(cb)),
    },

    settings: {
      get: (): Settings => ({ ...settings }),
      update: (change: Partial<Settings>) => updateSettings(change),
      onChange: (cb: (s: Settings) => void): Disposable => track(onSettings.on(cb)),
    },

    /** Data this plugin keeps between runs (any JSON). */
    storage: {
      get: <T = any>() => backend.call<T>('plugins.dataGet', { id: plugin.id }),
      set: (value: unknown) => backend.call('plugins.dataSet', { id: plugin.id, value }),
    },

    terminal: {
      /** Runs a program in a terminal tab: `run('Tests', ['cargo', 'test'])`. */
      run: (title: string, command: string[], cwd?: string) => runTask(title, command, cwd ?? workspace.root ?? workspace.info.home ?? '.'),
      /** Runs `cargo …` in the open project. */
      cargo: (args: string[], title?: string) => runCargo(args, title),
      open: () => newTerminal(),
    },

    /** The language server (rust++-analyzer). */
    lsp: {
      get running() {
        return lsp.running;
      },
      request: <T = any>(method: string, params: unknown) => lsp.request<T>(method, params),
      notify: (method: string, params: unknown) => lsp.notify(method, params),
      onNotification: (method: string, cb: (params: any) => void): Disposable => track(lsp.onNotification(method, cb)),
      /** The errors and warnings now, by file. */
      diagnostics: () => [...diagnostics.values()],
      onDiagnostics: (cb: () => void): Disposable => track(onDiagnostics.on(cb)),
    },

    /** The DEVR+ backend: files, processes, search, Cargo… (`call('fs.readDir', { path })`). */
    backend: {
      call: <T = any>(method: string, params?: unknown) => backend.call<T>(method, params),
      on: (event: string, cb: (payload: any) => void): Disposable => track(backend.on(event, cb)),
    },

    book: {
      open: (chapter?: string) => openBook(chapter),
    },
  };
  return api;
}

export type Devr = ReturnType<typeof createApi>;
