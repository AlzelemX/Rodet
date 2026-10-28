// User settings, saved by the backend in the DEVR+ config folder.
import { backend } from './backend';
import { type Language, syncLanguage } from './i18n';
import { Emitter } from './util';

export interface Settings {
  /** The interface language (also kept in the window's storage, see i18n). */
  language?: Language;
  /** `dark`, `light`, or the id of a theme from a plugin. */
  theme: string;
  fontSize: number;
  fontFamily: string;
  tabSize: number;
  wordWrap: boolean;
  minimap: boolean;
  autoSave: boolean;
  /** The ↩ mark after returned values. */
  returnMarks: boolean;
  typeHints: boolean;
  parameterHints: boolean;
  speedScore: boolean;
  /** Run `rodet check` on save and show the compiler's errors. */
  checkOnSave: boolean;
  /** Ids of installed plugins that are turned off. */
  disabledPlugins: string[];
}

export const defaults: Settings = {
  theme: 'dark',
  fontSize: 14,
  fontFamily: "'Cascadia Code', 'JetBrains Mono', 'Fira Code', Consolas, 'Ubuntu Mono', Menlo, monospace",
  tabSize: 4,
  wordWrap: false,
  minimap: true,
  autoSave: true,
  returnMarks: true,
  typeHints: true,
  parameterHints: true,
  speedScore: true,
  checkOnSave: true,
  disabledPlugins: [],
};

const SETTINGS_VERSION = 2;

export let settings: Settings = { ...defaults };
export const onSettings = new Emitter<Settings>();

export async function loadSettings() {
  try {
    const saved = await backend.call<Partial<Settings> & { version?: number }>('settings.get');
    // Version 2 turned auto save on by default (DEVR+ 0.1 saved `false`).
    if ((saved.version ?? 1) < 2) delete saved.autoSave;
    settings = { ...defaults, ...saved };
    syncLanguage(settings.language);
  } catch {
    settings = { ...defaults };
  }
  onSettings.fire(settings);
}

export function updateSettings(change: Partial<Settings>) {
  settings = { ...settings, ...change };
  onSettings.fire(settings);
  backend.call('settings.set', { ...settings, version: SETTINGS_VERSION }).catch(() => {});
}
