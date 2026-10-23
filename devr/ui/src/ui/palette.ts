// Quick pick at the top of the window: commands (Ctrl+Shift+P), files
// (Ctrl+P), symbols (Ctrl+T) and lists of locations.
import { backend } from '../backend';
import type { LspLocation } from '../lsp/client';
import { basename, fileIcon, fuzzy, h, highlighted, icon, keyLabel, relativePath } from '../util';
import { pathOfUri, workspace } from '../workspace';

export interface PickItem {
  label: string;
  description?: string;
  detail?: string;
  icon?: string;
  iconColor?: string;
  key?: string;
  run: () => void;
}

interface PickOptions {
  placeholder: string;
  /** Static items, filtered with fuzzy matching. */
  items?: PickItem[];
  /** Items computed from the typed text (e.g. symbol search). */
  query?: (text: string) => Promise<PickItem[]>;
  value?: string;
  /** Typing this prefix switches to another picker (like `>` for commands). */
  prefixes?: Record<string, () => void>;
}

let current: (() => void) | null = null;

export function pick(options: PickOptions) {
  current?.();
  const input = h('input.input', { placeholder: options.placeholder, spellcheck: false, value: options.value ?? '' });
  const list = h('div.list');
  const box = h('div.palette', {}, input, list);
  const backdrop = h('div.palette-backdrop', {}, box);
  let items: { item: PickItem; positions: number[] }[] = [];
  let selected = 0;
  let generation = 0;

  const close = () => {
    backdrop.remove();
    current = null;
  };
  current = close;

  const render = () => {
    list.replaceChildren();
    if (!items.length) {
      list.appendChild(h('div.empty', {}, input.value ? 'No results' : 'Type to search'));
      return;
    }
    items.slice(0, 200).forEach(({ item, positions }, i) => {
      const row = h(i === selected ? 'div.item.selected' : 'div.item', {
        onmousedown: (e: MouseEvent) => { e.preventDefault(); choose(i); },
        onmousemove: () => { if (selected !== i) { selected = i; render(); } },
      },
      item.icon ? icon(item.icon) : null,
      h('span.label', {}, highlighted(item.label, positions)),
      item.description ? h('span.desc', {}, item.description) : null,
      item.key ? h('span.key', {}, keyLabel(item.key)) : null);
      if (item.icon && item.iconColor) (row.firstChild as HTMLElement).style.color = item.iconColor;
      list.appendChild(row);
    });
    (list.children[selected] as HTMLElement | undefined)?.scrollIntoView({ block: 'nearest' });
  };

  const choose = (i: number) => {
    const entry = items[i];
    if (!entry) return;
    close();
    entry.item.run();
  };

  const update = async () => {
    const text = input.value;
    for (const [prefix, open] of Object.entries(options.prefixes ?? {})) {
      if (text.startsWith(prefix) && !(options.value ?? '').startsWith(prefix)) {
        close();
        open();
        return;
      }
    }
    const gen = ++generation;
    if (options.query) {
      const result = await options.query(text);
      if (gen !== generation) return;
      items = result.map((item) => ({ item, positions: fuzzy(text.replace(/^[#>@]/, ''), item.label).positions }));
    } else {
      const q = text.replace(/^>/, '').trim();
      items = (options.items ?? [])
        .map((item) => ({ item, ...fuzzy(q, item.description ? `${item.label} ${item.description}` : item.label) }))
        .filter((r) => r.score >= 0)
        .sort((a, b) => b.score - a.score)
        .map(({ item, positions }) => ({ item, positions: positions.filter((p) => p < item.label.length) }));
    }
    selected = 0;
    render();
  };

  input.addEventListener('input', update);
  input.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowDown') {
      selected = Math.min(items.length - 1, selected + 1);
      render();
    } else if (e.key === 'ArrowUp') {
      selected = Math.max(0, selected - 1);
      render();
    } else if (e.key === 'Enter') {
      choose(selected);
    } else if (e.key === 'Escape') {
      close();
    } else return;
    e.preventDefault();
  });
  backdrop.addEventListener('mousedown', (e) => {
    if (e.target === backdrop) close();
  });
  document.body.appendChild(backdrop);
  input.focus();
  input.setSelectionRange(input.value.length, input.value.length);
  update();
}

// ---- commands ----

export interface Command {
  id: string;
  title: string;
  key?: string;
  icon?: string;
  run: () => void;
}

export const commands = new Map<string, Command>();

export function registerCommand(command: Command) {
  commands.set(command.id, command);
}

export function runCommand(id: string) {
  commands.get(id)?.run();
}

export function commandPalette() {
  pick({
    placeholder: 'Type a command',
    value: '>',
    items: [...commands.values()].map((c) => ({ label: c.title, key: c.key, icon: c.icon, run: c.run })),
    prefixes: { '#': () => symbolPicker(), '@': () => symbolPicker() },
  });
}

// ---- files ----

let fileCache: { root: string; files: string[] } | null = null;
export function invalidateFileCache() {
  fileCache = null;
}

export async function quickOpen() {
  const root = workspace.root;
  if (!root) {
    commandPalette();
    return;
  }
  if (!fileCache || fileCache.root !== root) {
    fileCache = { root, files: await backend.call<string[]>('fs.listFiles', { path: root }) };
  }
  const files = fileCache.files;
  pick({
    placeholder: 'Search files by name (> for commands, # for symbols)',
    prefixes: { '>': () => commandPalette(), '#': () => symbolPicker() },
    query: async (text) => {
      const q = text.trim();
      const scored = files
        .map((path) => ({ path, rel: relativePath(root, path), ...fuzzy(q, relativePath(root, path)) }))
        .filter((r) => r.score >= 0)
        .sort((a, b) => b.score - a.score)
        .slice(0, 100);
      return scored.map(({ path, rel }) => {
        const fi = fileIcon(path);
        return {
          label: basename(path),
          description: rel.includes('/') ? rel.slice(0, rel.lastIndexOf('/')) : '',
          icon: fi.name,
          iconColor: fi.color,
          run: () => workspace.open(path),
        };
      });
    },
  });
}

// ---- symbols ----

const symbolIcons: Record<number, string> = {
  2: 'symbol-namespace', 5: 'symbol-class', 6: 'symbol-method', 8: 'symbol-field', 10: 'symbol-enum',
  11: 'symbol-interface', 12: 'symbol-function', 13: 'symbol-variable', 14: 'symbol-constant',
  19: 'symbol-object', 22: 'symbol-enum-member', 23: 'symbol-structure', 26: 'symbol-type-parameter',
};
export function symbolIcon(kind: number) {
  return symbolIcons[kind] ?? 'symbol-misc';
}

export async function searchSymbols(query: string): Promise<{ name: string; kind: number; container?: string; location: LspLocation }[]> {
  const { lsp } = await import('../lsp/features');
  if (!lsp.running || !query.trim()) return [];
  try {
    const result: any[] = (await lsp.request('workspace/symbol', { query })) ?? [];
    return result.map((s) => ({ name: s.name, kind: s.kind, container: s.containerName, location: s.location }));
  } catch {
    return [];
  }
}

export function symbolPicker() {
  pick({
    placeholder: 'Go to a struct, enum, trait, function… in the project',
    value: '#',
    prefixes: { '>': () => commandPalette() },
    query: async (text) => {
      const symbols = await searchSymbols(text.replace(/^#/, ''));
      return symbols.map((s) => ({
        label: s.name,
        description: [s.container, relativePath(workspace.root, pathOfUri(s.location.uri))].filter(Boolean).join(' · '),
        icon: symbolIcon(s.kind),
        run: () => openLocation(s.location),
      }));
    },
  });
}

export function openLocation(location: LspLocation) {
  return workspace.open(pathOfUri(location.uri), {
    line: location.range.start.line + 1,
    column: location.range.start.character + 1,
    endLine: location.range.end.line + 1,
    endColumn: location.range.end.character + 1,
  });
}

export function pickLocation(title: string, locations: LspLocation[]) {
  pick({
    placeholder: `${title} (${locations.length})`,
    items: locations.map((l) => ({
      label: `${basename(pathOfUri(l.uri))}:${l.range.start.line + 1}`,
      description: relativePath(workspace.root, pathOfUri(l.uri)),
      icon: 'location',
      run: () => openLocation(l),
    })),
  });
}
