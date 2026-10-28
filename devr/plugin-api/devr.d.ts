// DEVR+ plugin API, version 1.
// واجهة إضافات DEVR+، الإصدار 1.
//
// A plugin is a folder with plugin.json and a JavaScript module:
//   export function activate(devr) { … }     // runs when the plugin starts
//   export function deactivate() { … }       // optional, runs when it stops
// Everything added through `devr` is removed again when the plugin stops or
// reloads (saving one of its files reloads it).
//
// الإضافة مجلد فيه plugin.json وملف JavaScript فيه `activate(devr)`.
// كل ما تضيفه عبر `devr` يُزال تلقائيًا عند إيقاف الإضافة أو إعادة تحميلها.
//
// In main.js, for completion and checks:
//   // @ts-check
//   /** @param {import('./devr').Devr} devr */
//   export function activate(devr) { … }

/** Something that can be undone. Calling `dispose` early is fine. */
export interface Disposable {
  dispose(): void;
}

/** English text, or both languages. / نص إنجليزي، أو باللغتين. */
export type Text = string | { en: string; ar?: string };

export interface Command {
  /** Unique, e.g. `my-plugin.sayHello`. */
  id: string;
  /** Shown in the command palette (Ctrl+Shift+P). */
  title: string;
  /** A shortcut, e.g. `Ctrl+Alt+H` (`Ctrl` is `⌘` on macOS). */
  key?: string;
  /** A codicon name: https://microsoft.github.io/vscode-codicons/dist/codicon.html */
  icon?: string;
  run(): void;
}

export interface PickItem {
  label: string;
  description?: string;
  icon?: string;
  iconColor?: string;
  key?: string;
  run(): void;
}

export interface MenuItem {
  label: string;
  icon?: string;
  key?: string;
  run(): void;
}

export interface Action {
  label: string;
  run(): void;
  primary?: boolean;
}

export type Level = 'info' | 'warning' | 'error' | 'success';

export interface SideView {
  /** Unique, e.g. `my-plugin.todo`. */
  id: string;
  title: string;
  icon: string;
  /** Your content. */
  element: HTMLElement;
  /** Buttons in the view's title bar. */
  actions?: HTMLElement[];
  onShow?(): void;
}

export interface PanelView {
  id: string;
  title: string;
  element: HTMLElement;
  actions?: HTMLElement[];
  onShow?(): void;
}

export interface Theme {
  /** Unique, e.g. `my-plugin.ocean`. */
  id: string;
  name: Text;
  /** The built-in theme it starts from. */
  base: 'dark' | 'light';
  /**
   * Workbench colors (CSS variables): --bg, --bg-sidebar, --bg-activity,
   * --bg-panel, --bg-tabs, --bg-tab, --bg-tab-active, --bg-input, --bg-hover,
   * --bg-active, --bg-selected, --bg-widget, --bg-badge, --bg-status, --fg,
   * --fg-strong, --fg-muted, --fg-faint, --border, --border-strong, --shadow,
   * --accent, --accent-soft, --accent-fg, --error, --warning, --info,
   * --success, --mark, --scroll, --radius, --font-ui, --font-code.
   */
  colors?: Record<string, string>;
  /** Code editor colors (Monaco theme data: `rules`, `colors`). */
  editor?: { rules?: { token: string; foreground?: string; background?: string; fontStyle?: string }[]; colors?: Record<string, string> };
  /** Terminal colors (xterm.js): background, foreground, cursor, red, green… */
  terminal?: Record<string, string>;
  /** Any other CSS while the theme is on. */
  css?: string;
}

export interface Settings {
  language?: 'en' | 'ar';
  theme: string;
  fontSize: number;
  fontFamily: string;
  tabSize: number;
  wordWrap: boolean;
  minimap: boolean;
  autoSave: boolean;
  returnMarks: boolean;
  typeHints: boolean;
  parameterHints: boolean;
  speedScore: boolean;
  checkOnSave: boolean;
  disabledPlugins: string[];
}

export interface Reveal {
  line: number;
  column?: number;
  endLine?: number;
  endColumn?: number;
}

export interface StatusItem extends Disposable {
  element: HTMLElement;
  /** Changes the text (and the icon). An empty item is hidden. */
  set(text: string, icon?: string): void;
  setTooltip(tooltip: string): void;
}

export interface Devr {
  /** API version: `"1"`. */
  readonly version: string;
  /** This plugin. */
  readonly plugin: { id: string; name: Text | undefined; version: string; path: string };
  /** DEVR+ version, os, home folder and the Rodet toolchain. */
  readonly app: any;
  /** The editor's language. / لغة المحرر. */
  readonly language: 'en' | 'ar';
  readonly arabic: boolean;

  /** Text in the editor's language (English in; Arabic from `addTranslations`). */
  t(english: string, vars?: Record<string, string | number>): string;
  /** Arabic for your texts: `{ 'Hello {name}': 'مرحبًا {name}' }`. */
  addTranslations(table: Record<string, string>): Disposable;
  /** Undo these when the plugin stops (Monaco's disposables too). */
  readonly subscriptions: { push(...items: (Disposable | (() => void))[]): void };

  readonly commands: {
    register(command: Command): Disposable;
    run(id: string): void;
    list(): Command[];
  };

  readonly keys: {
    /** Binds a key to any command (also DEVR+'s own: `rodet.run`, `palette`…). */
    bind(key: string, commandId: string): Disposable;
    list(): { key: string; command: string }[];
  };

  readonly editor: {
    /** The whole Monaco API (`monaco.languages`, `monaco.editor`…). */
    readonly monaco: any;
    /** The code editor (IStandaloneCodeEditor). */
    readonly current: any;
    /** The open file's text model (ITextModel), or null. */
    readonly model: any;
    readonly text: string;
    readonly selection: any;
    /** Replaces the selection / inserts at the cursor. */
    insert(text: string): void;
    /** Editor options over DEVR+'s (`{ cursorStyle: 'block' }`). */
    setOptions(options: Record<string, unknown>): Disposable;
    onChange(cb: (event: any) => void): Disposable;
    onCursor(cb: (event: any) => void): Disposable;
    /** Runs a Monaco action, e.g. `editor.action.formatDocument`. */
    action(id: string): Promise<void> | undefined;
  };

  readonly workspace: {
    /** The open folder, or null. */
    readonly root: string | null;
    /** The active tab's path, or null. */
    readonly active: string | null;
    readonly tabs: string[];
    open(path: string, reveal?: Reveal): Promise<unknown>;
    openFolder(path: string): Promise<void>;
    save(path?: string): Promise<unknown>;
    saveAll(): Promise<unknown>;
    close(path: string): Promise<boolean>;
    isDirty(path: string): boolean;
    /** All files of the open folder. */
    files(): Promise<string[]>;
    readFile(path: string): Promise<string>;
    writeFile(path: string, text: string): Promise<unknown>;
    readDir(path: string): Promise<{ name: string; path: string; isDir: boolean }[]>;
    onRoot(cb: (root: string | null) => void): Disposable;
    onActive(cb: (path: string | null) => void): Disposable;
    onOpen(cb: (path: string, model: any) => void): Disposable;
    onSave(cb: (path: string, model: any) => void): Disposable;
    onClose(cb: (path: string) => void): Disposable;
  };

  readonly ui: {
    /** Makes an element: `h('div.card', { onclick() {} }, 'text', child)`. */
    h(spec: string, attrs?: Record<string, any>, ...children: any[]): HTMLElement;
    /** A codicon: `icon('rocket')`. */
    icon(name: string, extraClass?: string): HTMLElement;
    toast(message: string, level?: Level, actions?: Action[], timeoutMs?: number): () => void;
    confirm(title: string, detail?: string, okLabel?: string, danger?: boolean): Promise<boolean>;
    prompt(title: string, value?: string, options?: { placeholder?: string; validate?(value: string): string | null }): Promise<string | null>;
    pick(items: PickItem[], placeholder?: string): void;
    /** A dialog with your content; returns its close function. */
    dialog(title: string, body: HTMLElement, wide?: boolean): () => void;
    contextMenu(event: MouseEvent, items: (MenuItem | 'sep')[]): void;
    addSideView(view: SideView): Disposable;
    addPanel(view: PanelView): Disposable;
    addActivityButton(title: string, icon: string, run: () => void): Disposable;
    addStatusItem(options?: { text?: string; icon?: string; tooltip?: string; align?: 'left' | 'right'; onClick?(): void }): StatusItem;
    /** A tab in the editor area with your own page. */
    openPage(page: { id: string; title: string; icon?: string; element: HTMLElement }): Disposable;
    showSide(id: string): void;
    showPanel(id: string): void;
    toggleSidebar(): void;
    togglePanel(show?: boolean): void;
  };

  /**
   * The workbench's elements: root, activitybar, sidebar, sidebarBody,
   * center, editorGroup, tabsBar, breadcrumbs, editorHost, panel, panelBody,
   * statusbar, statusLeft, statusRight… Move, hide or restyle anything.
   */
  readonly layout: any;

  readonly theme: {
    register(theme: Theme): Disposable;
    /** Switches theme: `dark`, `light` or a registered id. */
    use(id: string): void;
    readonly current: Theme;
    list(): { id: string; name: string; base: 'dark' | 'light' }[];
    /** CSS for the whole editor. / تنسيق CSS للمحرر كله. */
    addStyle(css: string): Disposable;
    /** `setColors({ '--accent': '#2f9e44' })`. */
    setColors(colors: Record<string, string>): Disposable;
    onChange(cb: (theme: Theme) => void): Disposable;
  };

  readonly settings: {
    get(): Settings;
    update(change: Partial<Settings>): void;
    onChange(cb: (settings: Settings) => void): Disposable;
  };

  /** Your plugin's data, kept between runs (any JSON). */
  readonly storage: {
    get<T = any>(): Promise<T | null>;
    set(value: unknown): Promise<unknown>;
  };

  readonly terminal: {
    run(title: string, command: string[], cwd?: string): Promise<void>;
    /** Runs `rodet …` (Silk) in the open project. */
    rodet(args: string[], title?: string): void;
    open(): Promise<void>;
  };

  /** rodet-analyzer (Language Server Protocol). */
  readonly lsp: {
    readonly running: boolean;
    request<T = any>(method: string, params: unknown): Promise<T>;
    notify(method: string, params: unknown): void;
    onNotification(method: string, cb: (params: any) => void): Disposable;
    diagnostics(): { uri: string; diagnostics: any[] }[];
    onDiagnostics(cb: () => void): Disposable;
  };

  /** The DEVR+ backend: `call('fs.readDir', { path })`, `call('search.text', …)`… */
  readonly backend: {
    call<T = any>(method: string, params?: unknown): Promise<T>;
    on(event: string, cb: (payload: any) => void): Disposable;
  };

  readonly book: { open(chapter?: string): Promise<void> };
}
