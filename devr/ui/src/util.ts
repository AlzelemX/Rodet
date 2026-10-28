// Small helpers shared by the whole UI.

export type Child = Node | string | null | undefined | false;

/** Creates an element: `h('div.a.b', { onclick }, child, ...)`. */
export function h<K extends keyof HTMLElementTagNameMap>(
  spec: K | `${K}.${string}` | `${K}#${string}`,
  attrs: Record<string, any> = {},
  ...children: (Child | Child[])[]
): HTMLElementTagNameMap[K] {
  const [tagAndId, ...classes] = spec.split('.');
  const [tag, id] = tagAndId.split('#');
  const el = document.createElement(tag as K);
  if (id) el.id = id;
  if (classes.length) el.className = classes.join(' ');
  for (const [key, value] of Object.entries(attrs)) {
    if (value === undefined || value === null || value === false) continue;
    if (key.startsWith('on') && typeof value === 'function') {
      el.addEventListener(key.slice(2).toLowerCase(), value);
    } else if (key === 'class') {
      el.className += (el.className ? ' ' : '') + value;
    } else if (key === 'style' && typeof value === 'object') {
      Object.assign(el.style, value);
    } else if (key === 'dataset') {
      Object.assign(el.dataset, value);
    } else if (key in el && typeof value !== 'string') {
      (el as any)[key] = value;
    } else {
      el.setAttribute(key, value === true ? '' : String(value));
    }
  }
  append(el, children);
  return el;
}

export function append(parent: Node, children: (Child | Child[])[]) {
  for (const child of children.flat()) {
    if (child === null || child === undefined || child === false) continue;
    parent.appendChild(typeof child === 'string' ? document.createTextNode(child) : child);
  }
}

/** Text with `code` parts shown as code (kept left to right in Arabic). */
export function richText(text: string): (string | HTMLElement)[] {
  return text.split(/(`[^`]+`)/).filter(Boolean).map((part) =>
    part.length > 2 && part.startsWith('`') && part.endsWith('`') ? h('code.inline', { dir: 'ltr' }, part.slice(1, -1)) : part);
}

/** A VS Code icon (codicon). */
export function icon(name: string, extra = ''): HTMLElement {
  return h('i', { class: `codicon codicon-${name} ${extra}`.trim(), 'aria-hidden': 'true' });
}

export class Emitter<T> {
  private listeners = new Set<(value: T) => void>();
  on(listener: (value: T) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  fire(value: T) {
    for (const listener of [...this.listeners]) listener(value);
  }
}

export function debounce<A extends unknown[]>(fn: (...args: A) => void, ms: number) {
  let timer: number | undefined;
  const run = (...args: A) => {
    clearTimeout(timer);
    timer = window.setTimeout(() => fn(...args), ms);
  };
  run.cancel = () => clearTimeout(timer);
  return run;
}

// ---- Paths (the backend uses the computer's own separator) ----

export function isWindowsPath(path: string) {
  return /^[a-zA-Z]:[\\/]/.test(path) || path.includes('\\');
}

export function sepOf(path: string) {
  return isWindowsPath(path) ? '\\' : '/';
}

export function basename(path: string) {
  const parts = path.split(/[\\/]/).filter(Boolean);
  return parts[parts.length - 1] ?? path;
}

export function dirname(path: string) {
  const i = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
  if (i <= 0) return path.slice(0, i + 1) || path;
  if (/^[a-zA-Z]:$/.test(path.slice(0, i))) return path.slice(0, i + 1);
  return path.slice(0, i);
}

export function joinPath(dir: string, name: string) {
  const sep = sepOf(dir);
  return dir.endsWith('/') || dir.endsWith('\\') ? dir + name : dir + sep + name;
}

/** `path` relative to `root`, with `/`, or `path` itself when outside. */
export function relativePath(root: string | null, path: string) {
  if (!root) return path;
  const norm = (p: string) => p.replace(/\\/g, '/').replace(/\/+$/, '');
  const r = norm(root);
  const p = norm(path);
  const same = isWindowsPath(root) ? p.toLowerCase().startsWith(r.toLowerCase() + '/') : p.startsWith(r + '/');
  return same ? p.slice(r.length + 1) : p;
}

export function samePath(a: string, b: string) {
  const norm = (p: string) => p.replace(/\\/g, '/').replace(/\/+$/, '');
  return isWindowsPath(a) ? norm(a).toLowerCase() === norm(b).toLowerCase() : norm(a) === norm(b);
}

export function extension(path: string) {
  const name = basename(path);
  const dot = name.lastIndexOf('.');
  return dot > 0 ? name.slice(dot + 1).toLowerCase() : '';
}

/** The codicon for a file in the explorer and tabs. */
export function fileIcon(path: string): { name: string; color: string } {
  const name = basename(path).toLowerCase();
  if (['silk.toml', 'silk.lock', 'cargo.toml', 'cargo.lock'].includes(name)) return { name: 'package', color: '#e8590c' };
  switch (extension(path)) {
    case 'rd':
    case 'rs':
      return { name: 'symbol-namespace', color: '#ff8a3d' };
    case 'toml':
      return { name: 'settings', color: '#9c8b7a' };
    case 'md':
      return { name: 'markdown', color: '#519aba' };
    case 'json':
      return { name: 'json', color: '#cbcb41' };
    case 'png':
    case 'jpg':
    case 'jpeg':
    case 'gif':
    case 'svg':
    case 'ico':
      return { name: 'file-media', color: '#a074c4' };
    case 'lock':
      return { name: 'lock', color: '#8a8a8a' };
    case 'sh':
    case 'ps1':
    case 'bat':
      return { name: 'terminal', color: '#89e051' };
    default:
      return { name: 'file', color: 'var(--fg-muted)' };
  }
}

/** Fuzzy match: returns a score (higher is better) or -1, and the matched positions. */
export function fuzzy(query: string, text: string): { score: number; positions: number[] } {
  if (!query) return { score: 0, positions: [] };
  const q = query.toLowerCase();
  const t = text.toLowerCase();
  const positions: number[] = [];
  let score = 0;
  let ti = 0;
  let prev = -2;
  for (const ch of q) {
    if (ch === ' ') continue;
    const found = t.indexOf(ch, ti);
    if (found < 0) return { score: -1, positions: [] };
    positions.push(found);
    score += found === prev + 1 ? 5 : 1;
    if (found === 0 || '/\\_-. '.includes(text[found - 1] ?? '')) score += 4;
    if (text[found] !== t[found] && text[found] === text[found].toUpperCase()) score += 1;
    prev = found;
    ti = found + 1;
  }
  // Prefer matches in the file name and shorter paths.
  const nameStart = Math.max(text.lastIndexOf('/'), text.lastIndexOf('\\')) + 1;
  if (positions[0] >= nameStart) score += 10;
  score -= text.length / 100;
  return { score, positions };
}

/** `text` with the characters at `positions` highlighted. */
export function highlighted(text: string, positions: number[]): HTMLElement {
  const span = h('span');
  const set = new Set(positions);
  let run = '';
  let inMatch = false;
  const flush = () => {
    if (!run) return;
    span.appendChild(inMatch ? h('mark', {}, run) : document.createTextNode(run));
    run = '';
  };
  for (let i = 0; i < text.length; i++) {
    const m = set.has(i);
    if (m !== inMatch) {
      flush();
      inMatch = m;
    }
    run += text[i];
  }
  flush();
  return span;
}

export function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

export const isMac = navigator.platform.toLowerCase().includes('mac');

/** `Ctrl+S` on Windows/Linux, `⌘S` on macOS. */
export function keyLabel(keys: string) {
  if (!isMac) return keys;
  return keys
    .replace(/Ctrl\+/g, '⌘')
    .replace(/Shift\+/g, '⇧')
    .replace(/Alt\+/g, '⌥');
}
