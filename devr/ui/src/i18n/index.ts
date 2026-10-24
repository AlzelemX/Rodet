// The language of DEVR+'s interface: English or Arabic.
//
// Every text in the interface goes through `t()`, written in English:
// `t('Open Folder…')`, `t('{n} results', { n })`. In Arabic, `t()` returns
// the translation from `ar.ts` (the English text when one is missing).
//
// The language is read once at start-up (modules build parts of the
// interface when they load), so changing it reloads the window.
import { AR } from './ar';

export type Language = 'en' | 'ar';

const STORAGE_KEY = 'devr.language';

function initialLanguage(): Language {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === 'en' || saved === 'ar') return saved;
  } catch {
    // No storage: follow the system.
  }
  return navigator.language?.toLowerCase().startsWith('ar') ? 'ar' : 'en';
}

export const language: Language = initialLanguage();
export const arabic = language === 'ar';

document.documentElement.lang = language;
document.documentElement.dataset.lang = language;

/** Arabic for texts added at run time (by plugins). */
const extra = new Map<string, string>();

/** Adds Arabic translations: `{ 'Hello': 'مرحبًا' }`. Returns a function that removes them. */
export function addTranslations(table: Record<string, string>): () => void {
  const added = Object.entries(table).filter(([en]) => !extra.has(en));
  for (const [en, ar] of added) extra.set(en, ar);
  return () => {
    for (const [en, ar] of added) if (extra.get(en) === ar) extra.delete(en);
  };
}

/** The interface text for `english`, with `{name}` placeholders filled in. */
export function t(english: string, vars?: Record<string, string | number>): string {
  let text = arabic ? isolateLatin(AR[english] ?? extra.get(english) ?? english) : english;
  if (vars) text = text.replace(/\{(\w+)\}/g, (m, key) => (key in vars ? String(vars[key]) : m));
  return text;
}

/** In right-to-left text, `Rust++` would show as `++Rust`: names ending in
 * symbols are wrapped in left-to-right isolates. */
export function isolateLatin(text: string) {
  return text.replace(/(Rust\+\+|DEVR\+|C\+\+)/g, '\u2066$1\u2069');
}

/** `count` with the right plural: `plural(3, '{n} file', '{n} files')`. */
export function plural(n: number, one: string, many: string) {
  return t(n === 1 ? one : many, { n });
}

/** Saves the language and reloads the window with it. */
export function switchLanguage(next: Language) {
  if (next === language) return;
  try {
    localStorage.setItem(STORAGE_KEY, next);
  } catch {
    // Without storage the setting file still remembers it.
  }
  location.reload();
}

/** Called with the saved setting: reloads once if it differs from the
 * language the window started with (a new computer, cleared storage…). */
export function syncLanguage(saved: Language | undefined) {
  if (!saved) return;
  try {
    if (localStorage.getItem(STORAGE_KEY) === saved) return;
  } catch {
    return;
  }
  switchLanguage(saved);
}
