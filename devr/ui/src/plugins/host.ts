// Runs DEVR+ plugins.
//
// A plugin is a folder with a `plugin.json` in the DEVR+ plugins folder. It
// can bring styles (CSS for the whole editor), color themes, and a JavaScript
// module whose `activate(devr)` gets the whole editor API (see api.ts and
// devr/plugin-api/devr.d.ts). Everything a plugin adds is undone when it
// stops, so turning it off or reloading it never needs a restart.
import { backend, errorText } from '../backend';
import { arabic, t } from '../i18n';
import { settings, updateSettings } from '../settings';
import { registerTheme, type Theme } from '../theme';
import { toast } from '../ui/notify';
import { Emitter, basename } from '../util';
import { createApi } from './api';

export type Text = string | { en: string; ar?: string };

export interface Manifest {
  id: string;
  name?: Text;
  version?: string;
  description?: Text;
  author?: string;
  /** The JavaScript module with `activate(devr)`. */
  main?: string;
  /** CSS files, active while the plugin is on. */
  styles?: string[];
  /** Color themes: JSON files like `{ "name": …, "base": "dark", "colors": {…}, "editor": {…} }`. */
  themes?: string[];
  /** The plugin API version it was written for. */
  devr?: string;
}

export interface Installed {
  id: string;
  path: string;
  manifest: Manifest;
  error: string | null;
}

export type State = 'off' | 'starting' | 'on' | 'failed';

export interface Plugin {
  id: string;
  path: string;
  manifest: Manifest;
  state: State;
  /** Why it failed (or can't load). */
  error: string | null;
  /** Undo functions of everything it added, run in reverse order on stop. */
  disposers: (() => void)[];
  /** What it added, shown in the Extensions view. */
  added: { commands: Set<string>; themes: Set<string>; views: Set<string> };
  module: any;
}

export const API_VERSION = '1';
export const plugins = new Map<string, Plugin>();
export const onPlugins = new Emitter<void>();

export function text(value: Text | undefined, fallback = ''): string {
  if (!value) return fallback;
  if (typeof value === 'string') return value;
  return (arabic && value.ar) || value.en || fallback;
}

export function pluginName(p: Plugin) {
  return text(p.manifest.name, p.id);
}

export function isEnabled(id: string) {
  return !settings.disabledPlugins.includes(id);
}

/** Reads the installed plugins and starts the ones that are on. */
export async function loadPlugins() {
  for (const p of [...plugins.values()]) stopPlugin(p);
  plugins.clear();
  let list: Installed[] = [];
  try {
    list = await backend.call<Installed[]>('plugins.list');
  } catch (err) {
    console.error('DEVR+: can’t list plugins', err);
  }
  for (const item of list) {
    plugins.set(item.id, {
      id: item.id,
      path: item.path,
      manifest: { ...item.manifest, id: item.id },
      state: 'off',
      error: item.error,
      disposers: [],
      added: { commands: new Set(), themes: new Set(), views: new Set() },
      module: null,
    });
  }
  onPlugins.fire();
  await Promise.all([...plugins.values()].filter((p) => !p.error && isEnabled(p.id)).map((p) => startPlugin(p)));
}

function read(p: Plugin, file: string) {
  return backend.call<string>('plugins.read', { id: p.id, file });
}

/** Starts a plugin: its styles, its themes, then `activate(devr)`. */
export async function startPlugin(p: Plugin) {
  if (p.state === 'on' || p.state === 'starting') return;
  p.state = 'starting';
  p.error = null;
  onPlugins.fire();
  try {
    for (const file of p.manifest.styles ?? []) {
      const css = await read(p, file);
      const style = document.createElement('style');
      style.dataset.plugin = p.id;
      style.textContent = `${css}\n/*# sourceURL=devr-plugin://${p.id}/${file} */`;
      document.head.appendChild(style);
      p.disposers.push(() => style.remove());
    }
    for (const file of p.manifest.themes ?? []) {
      const theme = JSON.parse(await read(p, file)) as Partial<Theme>;
      const id = theme.id ?? `${p.id}.${basename(file).replace(/\.json$/i, '')}`;
      const remove = registerTheme({ base: 'dark', name: id, ...theme, id } as Theme);
      p.added.themes.add(id);
      p.disposers.push(() => {
        remove();
        p.added.themes.delete(id);
      });
    }
    if (p.manifest.main) {
      const code = await read(p, p.manifest.main);
      const url = URL.createObjectURL(new Blob(
        [`${code}\n//# sourceURL=devr-plugin://${p.id}/${p.manifest.main}\n`],
        { type: 'text/javascript' },
      ));
      try {
        p.module = await import(/* @vite-ignore */ url);
      } finally {
        URL.revokeObjectURL(url);
      }
      const activate = p.module.activate ?? p.module.default;
      if (typeof activate === 'function') {
        const api = createApi(p);
        await withTimeout(Promise.resolve(activate(api)), 15000, t('`activate` took longer than 15 seconds'));
      }
    }
    p.state = 'on';
  } catch (err) {
    console.error(`DEVR+: plugin ${p.id} failed`, err);
    undo(p);
    p.state = 'failed';
    p.error = errorText(err);
    toast(t('The plugin {name} failed to start: {error}', { name: pluginName(p), error: p.error }), 'error', [
      { label: t('Turn Off'), run: () => setEnabled(p.id, false) },
    ]);
  }
  onPlugins.fire();
}

function withTimeout<T>(promise: Promise<T>, ms: number, message: string): Promise<T> {
  return Promise.race([promise, new Promise<T>((_, reject) => setTimeout(() => reject(new Error(message)), ms))]);
}

function undo(p: Plugin) {
  try {
    p.module?.deactivate?.();
  } catch (err) {
    console.error(`DEVR+: plugin ${p.id}: deactivate failed`, err);
  }
  p.module = null;
  for (const dispose of p.disposers.splice(0).reverse()) {
    try {
      dispose();
    } catch (err) {
      console.error(`DEVR+: plugin ${p.id}: cleanup failed`, err);
    }
  }
}

/** Stops a plugin and removes everything it added. */
export function stopPlugin(p: Plugin) {
  if (p.state === 'off') return;
  undo(p);
  p.state = 'off';
  onPlugins.fire();
}

export async function reloadPlugin(id: string) {
  const p = plugins.get(id);
  if (!p) return;
  stopPlugin(p);
  // Read plugin.json again: the author may have changed it.
  const fresh = (await backend.call<Installed[]>('plugins.list')).find((x) => x.id === id);
  if (fresh) {
    p.manifest = { ...fresh.manifest, id };
    p.error = fresh.error;
  }
  if (!p.error && isEnabled(id)) await startPlugin(p);
  else onPlugins.fire();
}

export async function setEnabled(id: string, on: boolean) {
  const disabled = new Set(settings.disabledPlugins);
  if (on) disabled.delete(id);
  else disabled.add(id);
  updateSettings({ disabledPlugins: [...disabled] });
  const p = plugins.get(id);
  if (!p) return;
  if (on && !p.error) await startPlugin(p);
  if (!on) stopPlugin(p);
}

export async function removePlugin(id: string) {
  const p = plugins.get(id);
  if (p) stopPlugin(p);
  await backend.call('plugins.remove', { id });
  plugins.delete(id);
  if (settings.disabledPlugins.includes(id)) {
    updateSettings({ disabledPlugins: settings.disabledPlugins.filter((x) => x !== id) });
  }
  onPlugins.fire();
}

/** The plugin a file belongs to (for reloading it when it is saved). */
export function pluginOfPath(path: string): Plugin | null {
  const norm = (s: string) => s.replace(/\\/g, '/').toLowerCase();
  const file = norm(path);
  for (const p of plugins.values()) {
    if (file.startsWith(norm(p.path).replace(/\/?$/, '/'))) return p;
  }
  return null;
}
