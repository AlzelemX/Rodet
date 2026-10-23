// User settings, saved by the backend in the DEVR+ config folder.
import { backend } from './backend';
import { Emitter } from './util';

export interface Settings {
  theme: 'dark' | 'light';
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
  /** Run `cargo check` on save and show the compiler's errors. */
  checkOnSave: boolean;
}

export const defaults: Settings = {
  theme: 'dark',
  fontSize: 14,
  fontFamily: "'Cascadia Code', 'JetBrains Mono', 'Fira Code', Consolas, 'Ubuntu Mono', Menlo, monospace",
  tabSize: 4,
  wordWrap: false,
  minimap: true,
  autoSave: false,
  returnMarks: true,
  typeHints: true,
  parameterHints: true,
  speedScore: true,
  checkOnSave: true,
};

export let settings: Settings = { ...defaults };
export const onSettings = new Emitter<Settings>();

export async function loadSettings() {
  try {
    const saved = await backend.call<Partial<Settings>>('settings.get');
    settings = { ...defaults, ...saved };
  } catch {
    settings = { ...defaults };
  }
  onSettings.fire(settings);
}

export function updateSettings(change: Partial<Settings>) {
  settings = { ...settings, ...change };
  onSettings.fire(settings);
  backend.call('settings.set', settings).catch(() => {});
}
