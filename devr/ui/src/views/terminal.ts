// The terminal panel: shells and Cargo tasks (Run, Build, Test…), with
// clickable `src/main.rs:12:5` links that open the file at that place.
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import '@xterm/xterm/css/xterm.css';
import { backend, errorText } from '../backend';
import { setTerminalRunner } from '../lsp/features';
import { settings, onSettings } from '../settings';
import { layout, type PanelView } from '../ui/layout';
import { toast } from '../ui/notify';
import { h, icon, isWindowsPath, joinPath } from '../util';
import { workspace } from '../workspace';

interface Term {
  id: number | null;
  title: string;
  xterm: Terminal;
  fit: FitAddon;
  host: HTMLElement;
  row: HTMLElement;
  exited: boolean;
  cwd: string;
  command: string[];
  task: boolean;
}

const terms: Term[] = [];
let active: Term | null = null;
const host = h('div.terminal-host');
const list = h('div.terminal-list');
const element = h('div', {}, h('div.terminal-area', {}, host, list));

function xtermTheme() {
  const dark = settings.theme === 'dark';
  return dark
    ? { background: '#181818', foreground: '#cccccc', cursor: '#e8590c', selectionBackground: '#264f78',
        black: '#000000', red: '#f14c4c', green: '#23d18b', yellow: '#f5f543', blue: '#3b8eea', magenta: '#d670d6',
        cyan: '#29b8db', white: '#e5e5e5', brightBlack: '#666666', brightRed: '#f14c4c', brightGreen: '#23d18b',
        brightYellow: '#f5f543', brightBlue: '#3b8eea', brightMagenta: '#d670d6', brightCyan: '#29b8db', brightWhite: '#e5e5e5' }
    : { background: '#f8f8f8', foreground: '#3b3b3b', cursor: '#c2410c', selectionBackground: '#add6ff',
        black: '#000000', red: '#cd3131', green: '#00bc00', yellow: '#949800', blue: '#0451a5', magenta: '#bc05bc',
        cyan: '#0598bc', white: '#555555', brightBlack: '#666666', brightRed: '#cd3131', brightGreen: '#14ce14',
        brightYellow: '#b5ba00', brightBlue: '#0451a5', brightMagenta: '#bc05bc', brightCyan: '#0598bc', brightWhite: '#a5a5a5' };
}

/** Links like `src/main.rs:12:5` (from rustc and cargo) open the file. */
function addFileLinks(term: Term) {
  const pattern = /((?:[A-Za-z]:)?[\w./\\-]+\.(?:rs|toml)):(\d+)(?::(\d+))?/g;
  term.xterm.registerLinkProvider({
    provideLinks(lineNumber, callback) {
      const line = term.xterm.buffer.active.getLine(lineNumber - 1)?.translateToString(true) ?? '';
      const links = [];
      for (const m of line.matchAll(pattern)) {
        const start = (m.index ?? 0) + 1;
        links.push({
          range: { start: { x: start, y: lineNumber }, end: { x: start + m[0].length - 1, y: lineNumber } },
          text: m[0],
          decorations: { underline: true, pointerCursor: true },
          activate: () => {
            const file = m[1];
            const absolute = file.startsWith('/') || isWindowsPath(file) && /^[A-Za-z]:/.test(file);
            const path = absolute ? file : joinPath(term.cwd, file.replace(/\//g, isWindowsPath(term.cwd) ? '\\' : '/'));
            workspace.open(path, { line: Number(m[2]), column: Number(m[3] ?? 1) });
          },
        });
      }
      callback(links);
    },
  });
}

async function create(title: string, cwd: string, command: string[] = [], task = false): Promise<Term> {
  const xterm = new Terminal({
    fontFamily: settings.fontFamily,
    fontSize: settings.fontSize - 1,
    cursorBlink: true,
    allowProposedApi: true,
    scrollback: 10000,
    theme: xtermTheme(),
  });
  const fit = new FitAddon();
  xterm.loadAddon(fit);
  xterm.loadAddon(new WebLinksAddon((_e, uri) => openUrl(uri)));
  const termHost = h('div');
  host.appendChild(termHost);
  const row = h('div.row');
  list.appendChild(row);
  const term: Term = { id: null, title, xterm, fit, host: termHost, row, exited: false, cwd, command, task };
  terms.push(term);
  renderList();
  xterm.open(termHost);
  addFileLinks(term);
  show(term);
  try {
    fit.fit();
  } catch {
    // Not visible yet.
  }
  try {
    const { id } = await backend.call<{ id: number }>('term.spawn', { cwd, cols: xterm.cols, rows: xterm.rows, command });
    term.id = id;
  } catch (err) {
    xterm.writeln(`\x1b[31m${errorText(err)}\x1b[0m`);
    term.exited = true;
  }
  xterm.onData((data) => {
    if (term.id !== null && !term.exited) backend.call('term.write', { id: term.id, data }).catch(() => {});
  });
  xterm.onResize(({ cols, rows }) => {
    if (term.id !== null && !term.exited) backend.call('term.resize', { id: term.id, cols, rows }).catch(() => {});
  });
  return term;
}

function renderList() {
  for (const term of terms) {
    term.row.className = `row${term === active ? ' selected' : ''}${term.exited ? ' exited' : ''}`;
    term.row.replaceChildren(
      icon(term.task ? 'tools' : 'terminal'),
      h('span.label', {}, term.title),
      h('span.end', {}, h('button.icon-button', {
        title: 'Kill',
        onclick: (e: MouseEvent) => {
          e.stopPropagation();
          kill(term);
        },
      }, icon('trash'))));
    term.row.onclick = () => show(term);
  }
}

function show(term: Term) {
  active = term;
  for (const t of terms) t.host.classList.toggle('active', t === term);
  renderList();
  requestAnimationFrame(() => {
    try {
      term.fit.fit();
    } catch {
      // Hidden.
    }
    term.xterm.focus();
  });
}

function kill(term: Term) {
  if (term.id !== null && !term.exited) backend.call('term.kill', { id: term.id }).catch(() => {});
  term.xterm.dispose();
  term.host.remove();
  term.row.remove();
  terms.splice(terms.indexOf(term), 1);
  if (active === term) {
    active = terms[terms.length - 1] ?? null;
    if (active) show(active);
  }
  renderList();
}

let opening = false;

export async function newTerminal() {
  opening = true;
  layout.showPanel('terminal');
  opening = false;
  const cwd = workspace.root ?? workspace.info.home ?? '.';
  await create(`Terminal ${terms.filter((t) => !t.task).length + 1}`, cwd);
}

/** Runs a command (e.g. `cargo run`) in a task terminal. */
export async function runTask(title: string, command: string[], cwd: string) {
  await workspace.saveAll();
  opening = true;
  layout.showPanel('terminal');
  opening = false;
  // Reuse the finished terminal of the same task, so tasks don't pile up.
  const old = terms.find((t) => t.task && t.title === title);
  if (old) {
    if (!old.exited) {
      backend.call('term.kill', { id: old.id }).catch(() => {});
    }
    kill(old);
  }
  const term = await create(title, cwd, command, true);
  term.xterm.writeln(`\x1b[2m> ${command.join(' ')}\x1b[0m`);
}

export function runCargo(args: string[], title?: string) {
  const root = workspace.root;
  if (!root) {
    toast('Open a Cargo project first.', 'warning');
    return;
  }
  runTask(title ?? `cargo ${args[0]}`, ['cargo', ...args], root);
}

function openUrl(url: string) {
  if (backendKindIsDesktop()) {
    import('@tauri-apps/plugin-opener').then((m) => m.openUrl(url)).catch(() => window.open(url, '_blank'));
  } else {
    window.open(url, '_blank');
  }
}

function backendKindIsDesktop() {
  return backend.kind === 'desktop';
}

export const terminalPanel: PanelView = {
  id: 'terminal',
  title: 'Terminal',
  element,
  onShow: () => {
    if (opening) return;
    if (!terms.length) newTerminal();
    else if (active) show(active);
  },
  actions: [
    h('button.icon-button', { title: 'New Terminal', onclick: () => newTerminal() }, icon('add')),
    h('button.icon-button', { title: 'Kill Terminal', onclick: () => active && kill(active) }, icon('trash')),
  ],
};

export function setupTerminal() {
  setTerminalRunner((title, command, cwd) => runTask(title, command, cwd));
  backend.on('term.data', ({ id, data }: { id: number; data: string }) => {
    terms.find((t) => t.id === id)?.xterm.write(data);
  });
  backend.on('term.exit', ({ id, code }: { id: number; code: number }) => {
    const term = terms.find((t) => t.id === id);
    if (!term) return;
    term.exited = true;
    if (term.task) {
      const ok = code === 0;
      term.xterm.writeln('');
      term.xterm.writeln(ok
        ? '\x1b[32m✓ Done.\x1b[0m'
        : `\x1b[31m✗ Exited with code ${code}.\x1b[0m \x1b[2mClick a file:line above to see the problem.\x1b[0m`);
    } else {
      term.xterm.writeln('\r\n\x1b[2m[process exited]\x1b[0m');
    }
    renderList();
  });
  layout.onResize.on(() => {
    if (active && !layout.panel.classList.contains('hidden')) {
      try {
        active.fit.fit();
      } catch {
        // Hidden.
      }
    }
  });
  onSettings.on(() => {
    for (const t of terms) {
      t.xterm.options.theme = xtermTheme();
      t.xterm.options.fontSize = settings.fontSize - 1;
    }
  });
}
