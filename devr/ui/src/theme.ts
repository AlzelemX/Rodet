// Color themes: the two built in (dark, light) and the ones plugins add.
// A theme sets the workbench's CSS variables (`--bg`, `--accent`…) and,
// if it wants, the code editor's colors.
import * as monaco from './editor/monaco';
import { arabic, t } from './i18n';
import { settings } from './settings';
import { Emitter } from './util';

export interface Theme {
  id: string;
  /** Shown in the settings; `{ en, ar }` for both languages. */
  name: string | { en: string; ar?: string };
  /** The built-in theme it starts from. */
  base: 'dark' | 'light';
  /** CSS variables of the workbench: `{ '--bg': '#0b1021', '--accent': '#38bdf8' }`. */
  colors?: Record<string, string>;
  /** Colors of the code editor (Monaco's theme data; `base` and `inherit` may be left out). */
  editor?: Partial<monaco.editor.IStandaloneThemeData>;
  /** Colors of the terminal (xterm.js theme: `background`, `foreground`, `red`…). */
  terminal?: Record<string, string>;
  /** Any other CSS, active while the theme is on. */
  css?: string;
}

export const themes = new Map<string, Theme>([
  ['dark', { id: 'dark', name: { en: 'Dark', ar: 'داكن' }, base: 'dark' }],
  ['light', { id: 'light', name: { en: 'Light', ar: 'فاتح' }, base: 'light' }],
]);
export const onThemes = new Emitter<void>();
/** Fired after a theme is applied (terminals and plugins repaint). */
export const onThemeApplied = new Emitter<Theme>();

export function themeName(theme: Theme) {
  if (typeof theme.name === 'string') return t(theme.name);
  return (arabic && theme.name.ar) || theme.name.en;
}

/** Adds a theme; returns a function that removes it. */
export function registerTheme(theme: Theme): () => void {
  themes.set(theme.id, theme);
  onThemes.fire();
  if (settings.theme === theme.id) applyTheme();
  return () => {
    if (themes.get(theme.id) !== theme) return;
    themes.delete(theme.id);
    onThemes.fire();
    if (settings.theme === theme.id) applyTheme();
  };
}

const style = document.createElement('style');
style.id = 'devr-theme';

/** The theme in use (a missing plugin theme falls back to dark). */
export function currentTheme(): Theme {
  return themes.get(settings.theme) ?? themes.get(settings.theme === 'light' ? 'light' : 'dark')!;
}

export function applyTheme() {
  const theme = currentTheme();
  document.documentElement.dataset.theme = theme.base;
  // A <style> rather than inline styles, so plugin styles added later can still win.
  const vars = Object.entries(theme.colors ?? {})
    .filter(([name]) => name.startsWith('--'))
    .map(([name, value]) => `  ${name}: ${value};`)
    .join('\n');
  style.textContent = `${vars ? `:root[data-theme] {\n${vars}\n}\n` : ''}${theme.css ?? ''}`;
  if (!style.isConnected) document.head.appendChild(style);
  // Plugin styles stay after the theme.
  for (const el of document.head.querySelectorAll('style[data-plugin]')) document.head.appendChild(el);

  if (theme.editor) {
    const name = `devr-theme-${theme.id}`;
    monaco.editor.defineTheme(name, {
      base: theme.base === 'dark' ? 'vs-dark' : 'vs',
      inherit: true,
      rules: [],
      colors: {},
      ...theme.editor,
    } as monaco.editor.IStandaloneThemeData);
    monaco.editor.setTheme(name);
  } else {
    monaco.editor.setTheme(theme.base === 'dark' ? 'devr-dark' : 'devr-light');
  }
  onThemeApplied.fire(theme);
}
