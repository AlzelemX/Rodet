// The open project: its folder, the open files (Monaco models), the tabs,
// saving, and reloading files that changed on disk.
import { t } from './i18n';
import * as monaco from './editor/monaco';
import { backend, errorText } from './backend';
import { languageOf } from './editor/language';
import { Emitter, basename, isWindowsPath } from './util';
import { toast } from './ui/notify';

export interface Toolchain {
  rodetBin: string | null;
  rustc: string | null;
  cargo: string | null;
  analyzer: string | null;
  rustcVersion: string | null;
  isRodet: boolean;
}

export interface AppInfo {
  version: string;
  os: string;
  arch: string;
  home: string | null;
  pathSeparator: string;
  toolchain: Toolchain;
}

export interface Doc {
  path: string;
  model: monaco.editor.ITextModel;
  savedVersion: number;
  viewState: monaco.editor.ICodeEditorViewState | null;
  /** Shown in a tab (not only loaded for "peek" or search). */
  inTab: boolean;
}

export interface Reveal {
  line: number;
  column?: number;
  endLine?: number;
  endColumn?: number;
}

/** Key for a path: separators unified, and case-insensitive on Windows. */
export function pathKey(path: string) {
  const unified = path.replace(/\\/g, '/');
  return isWindowsPath(path) ? unified.toLowerCase() : unified;
}

export function uriOf(path: string) {
  return monaco.Uri.file(path);
}

export function pathOfUri(uri: string | monaco.Uri) {
  const parsed = typeof uri === 'string' ? monaco.Uri.parse(uri) : uri;
  return parsed.fsPath;
}

class Workspace {
  info!: AppInfo;
  root: string | null = null;
  readonly docs = new Map<string, Doc>();
  tabs: string[] = [];
  active: string | null = null;
  editor!: monaco.editor.IStandaloneCodeEditor;

  readonly onRoot = new Emitter<string | null>();
  readonly onTabs = new Emitter<void>();
  readonly onActive = new Emitter<string | null>();
  readonly onDirty = new Emitter<string>();
  /** A model was created for a file (tab or not). */
  readonly onModelOpened = new Emitter<Doc>();
  readonly onModelClosed = new Emitter<Doc>();
  readonly onSaved = new Emitter<Doc>();

  private loading = new Map<string, Promise<Doc | null>>();
  /** Tabs that are not files (the Doc Book): path → title and icon. */
  readonly pages = new Map<string, { title: string; icon: string }>();

  isPage(path: string | null): boolean {
    return !!path && this.pages.has(path);
  }

  /** Opens (or shows) a tab that is not a file, like the Doc Book. */
  openPage(path: string, title: string, icon: string) {
    this.pages.set(path, { title, icon });
    if (!this.tabs.includes(path)) {
      const at = this.active ? this.tabs.findIndex((t) => pathKey(t) === pathKey(this.active!)) : -1;
      this.tabs.splice(at + 1, 0, path);
    }
    this.onTabs.fire();
    this.show(path);
  }

  async init() {
    this.info = await backend.call<AppInfo>('app.info');
    backend.on('fs.changed', ({ paths }: { paths: string[] }) => this.onDiskChanged(paths));
  }

  async refreshToolchain() {
    this.info = await backend.call<AppInfo>('app.info');
  }

  async openFolder(path: string) {
    await this.closeAll(true);
    this.root = path;
    backend.call('recent.add', { path }).catch(() => {});
    backend.call('fs.watch', { path }).catch((err) => console.warn('watch failed', err));
    this.onRoot.fire(path);
  }

  async closeFolder() {
    if (!(await this.closeAll())) return;
    this.root = null;
    this.onRoot.fire(null);
  }

  get(path: string) {
    return this.docs.get(pathKey(path));
  }

  isDirty(path: string) {
    const doc = this.get(path);
    return !!doc && doc.model.getAlternativeVersionId() !== doc.savedVersion;
  }

  get dirtyDocs() {
    return [...this.docs.values()].filter((d) => d.inTab && this.isDirty(d.path));
  }

  /** Loads a file into a model (no tab). */
  async load(path: string): Promise<Doc | null> {
    const key = pathKey(path);
    const existing = this.docs.get(key);
    if (existing) return existing;
    let pending = this.loading.get(key);
    if (!pending) {
      pending = (async () => {
        try {
          const file = await backend.call<{ text: string; binary: boolean }>('fs.readFile', { path });
          if (file.binary) {
            toast(t('{name} is not a text file.', { name: basename(path) }), 'warning');
            return null;
          }
          const uri = uriOf(path);
          const model = monaco.editor.getModel(uri) ?? monaco.editor.createModel(file.text, languageOf(path), uri);
          const doc: Doc = { path, model, savedVersion: model.getAlternativeVersionId(), viewState: null, inTab: false };
          model.onDidChangeContent(() => this.onDirty.fire(doc.path));
          this.docs.set(key, doc);
          this.onModelOpened.fire(doc);
          return doc;
        } catch (err) {
          toast(errorText(err), 'error');
          return null;
        } finally {
          this.loading.delete(key);
        }
      })();
      this.loading.set(key, pending);
    }
    return pending;
  }

  /** Opens a file in a tab and shows it. */
  async open(path: string, reveal?: Reveal, focus = true) {
    const doc = await this.load(path);
    if (!doc) return;
    if (!doc.inTab) {
      doc.inTab = true;
      const at = this.active ? this.tabs.findIndex((t) => pathKey(t) === pathKey(this.active!)) : -1;
      this.tabs.splice(at + 1, 0, doc.path);
      this.onTabs.fire();
    }
    this.show(doc.path);
    if (reveal) {
      const range = new monaco.Range(
        reveal.line,
        reveal.column ?? 1,
        reveal.endLine ?? reveal.line,
        reveal.endColumn ?? reveal.column ?? 1,
      );
      this.editor.setSelection(range);
      this.editor.revealRangeInCenterIfOutsideViewport(range, monaco.editor.ScrollType.Smooth);
    }
    if (focus) this.editor.focus();
  }

  /** Switches the editor to a tab. */
  show(path: string | null) {
    const current = this.active ? this.get(this.active) : undefined;
    if (current) current.viewState = this.editor.saveViewState();
    this.active = path;
    const doc = path ? this.get(path) : undefined;
    this.editor.setModel(doc?.model ?? null);
    if (doc?.viewState) this.editor.restoreViewState(doc.viewState);
    this.onActive.fire(path);
  }

  async save(path = this.active) {
    if (!path) return;
    const doc = this.get(path);
    if (!doc) return;
    try {
      await backend.call('fs.writeFile', { path: doc.path, text: doc.model.getValue() });
      doc.savedVersion = doc.model.getAlternativeVersionId();
      this.onDirty.fire(doc.path);
      this.onSaved.fire(doc);
    } catch (err) {
      toast(errorText(err), 'error');
    }
  }

  async saveAll() {
    for (const doc of this.dirtyDocs) await this.save(doc.path);
  }

  /** Closes a tab; asks first when it has unsaved changes. Returns false when cancelled. */
  async close(path: string, force = false): Promise<boolean> {
    if (this.pages.has(path)) {
      const index = this.tabs.indexOf(path);
      if (index >= 0) this.tabs.splice(index, 1);
      this.pages.delete(path);
      if (this.active === path) this.show(this.tabs[Math.min(index, this.tabs.length - 1)] ?? null);
      this.onTabs.fire();
      return true;
    }
    const doc = this.get(path);
    if (!doc) return true;
    if (!force && this.isDirty(path)) {
      const { confirm } = await import('./ui/notify');
      const save = await confirm(t('Save changes to {name}?', { name: basename(path) }), t('Your changes will be lost if you don’t save them.'), t('Save'));
      if (save) await this.save(path);
      else if (!(await confirm(t('Close {name} without saving?', { name: basename(path) }), '', t('Don’t Save'), true))) return false;
    }
    const index = this.tabs.findIndex((t) => pathKey(t) === pathKey(path));
    if (index >= 0) this.tabs.splice(index, 1);
    doc.inTab = false;
    if (this.active && pathKey(this.active) === pathKey(path)) {
      this.show(this.tabs[Math.min(index, this.tabs.length - 1)] ?? null);
    }
    this.docs.delete(pathKey(path));
    this.onModelClosed.fire(doc);
    doc.model.dispose();
    this.onTabs.fire();
    return true;
  }

  async closeAll(force = false): Promise<boolean> {
    if (!force && this.dirtyDocs.length) {
      const { confirm } = await import('./ui/notify');
      const ok = await confirm(
        t('Save {n} changed file(s)?', { n: this.dirtyDocs.length }),
        this.dirtyDocs.map((d) => basename(d.path)).join(', '),
        t('Save All'),
      );
      if (ok) await this.saveAll();
      else if (!(await confirm(t('Close without saving?'), '', t('Don’t Save'), true))) return false;
    }
    for (const doc of [...this.docs.values()]) {
      this.onModelClosed.fire(doc);
      doc.model.dispose();
    }
    this.docs.clear();
    this.tabs = this.tabs.filter((t) => this.pages.has(t));
    this.show(this.tabs[0] ?? null);
    this.onTabs.fire();
    return true;
  }

  /** A file or folder was renamed or deleted from the explorer. */
  pathsMoved(from: string, to: string | null) {
    const prefix = pathKey(from);
    for (const doc of [...this.docs.values()]) {
      const key = pathKey(doc.path);
      if (key !== prefix && !key.startsWith(prefix + '/')) continue;
      if (to === null) {
        this.close(doc.path, true);
      } else {
        const newPath = to + doc.path.slice(from.length);
        const text = doc.model.getValue();
        const wasActive = this.active && pathKey(this.active) === key;
        const index = this.tabs.findIndex((t) => pathKey(t) === key);
        this.close(doc.path, true).then(async () => {
          const moved = await this.load(newPath);
          if (!moved) return;
          moved.model.setValue(text);
          if (index >= 0) {
            moved.inTab = true;
            this.tabs.splice(index, 0, newPath);
            this.onTabs.fire();
          }
          if (wasActive) this.show(newPath);
        });
      }
    }
  }

  private async onDiskChanged(paths: string[]) {
    for (const path of paths) {
      const doc = this.get(path);
      if (!doc || this.isDirty(doc.path)) continue;
      try {
        const file = await backend.call<{ text: string; binary: boolean }>('fs.readFile', { path: doc.path });
        if (file.binary || file.text === doc.model.getValue()) continue;
        doc.model.pushEditOperations([], [{ range: doc.model.getFullModelRange(), text: file.text }], () => null);
        doc.savedVersion = doc.model.getAlternativeVersionId();
        this.onDirty.fire(doc.path);
      } catch {
        // Deleted: keep the open text.
      }
    }
  }
}

export const workspace = new Workspace();
