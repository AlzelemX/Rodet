// Tabs, breadcrumbs, the speed score in the corner, and the welcome page.
import * as monaco from '../editor/monaco';
import { backend } from '../backend';
import { currentSpeed, diagnostics, lsp, onDiagnostics, onSpeed, type SpeedScore, toLspPosition, toRange } from '../lsp/features';
import { contextMenu, layout } from '../ui/layout';
import { runCommand } from '../ui/palette';
import { basename, debounce, fileIcon, h, icon, keyLabel, relativePath } from '../util';
import { pathKey, workspace } from '../workspace';

// ---------------------------------------------------------------- tabs

const tabs = h('div.tabs');
const tabActions = h('div.tabs-actions', {},
  h('button.icon-button.run', { title: 'Run (F5)', onclick: () => runCommand('cargo.run') }, icon('play')),
  h('button.icon-button', { title: 'Find Symbols (Ctrl+T)', onclick: () => runCommand('symbols.find') }, icon('symbol-class')),
  h('button.icon-button', { title: 'More', onclick: (e: MouseEvent) => contextMenu(e, [
    { label: 'Close All', icon: 'close-all', run: () => workspace.closeAll() },
    { label: 'Save All', icon: 'save-all', key: keyLabel('Ctrl+Alt+S'), run: () => workspace.saveAll() },
  ]) }, icon('ellipsis')));

function fileHasErrors(path: string) {
  return diagnostics.get(pathKey(path))?.diagnostics.some((d) => (d.severity ?? 1) === 1) ?? false;
}

function renderTabs() {
  tabs.replaceChildren();
  const names = new Map<string, number>();
  for (const path of workspace.tabs) names.set(basename(path), (names.get(basename(path)) ?? 0) + 1);
  for (const path of workspace.tabs) {
    const active = workspace.active !== null && pathKey(workspace.active) === pathKey(path);
    const dirty = workspace.isDirty(path);
    const errors = fileHasErrors(path);
    const fi = fileIcon(path);
    const fileIconEl = icon(fi.name);
    fileIconEl.style.color = fi.color;
    const rel = relativePath(workspace.root, path);
    const tab = h(`div.tab${active ? '.active' : ''}${dirty ? '.dirty' : ''}` as 'div', {
      title: rel,
      onmousedown: (e: MouseEvent) => {
        if (e.button === 1) {
          e.preventDefault();
          workspace.close(path);
        }
      },
      onclick: () => workspace.show(path),
      oncontextmenu: (e: MouseEvent) => contextMenu(e, [
        { label: 'Close', icon: 'close', key: keyLabel('Ctrl+W'), run: () => workspace.close(path) },
        { label: 'Close Others', run: async () => { for (const t of [...workspace.tabs]) if (pathKey(t) !== pathKey(path)) if (!(await workspace.close(t))) break; } },
        { label: 'Close All', icon: 'close-all', run: () => workspace.closeAll() },
        'sep',
        { label: 'Copy Path', icon: 'copy', run: () => navigator.clipboard.writeText(path) },
      ]),
    },
    fileIconEl,
    h(errors ? 'span.name.has-errors' : 'span.name', {}, basename(path)),
    (names.get(basename(path)) ?? 0) > 1 ? h('span.hint', {}, rel.split('/').slice(-2, -1)[0] ?? '') : null,
    h('button.close', {
      title: 'Close',
      onclick: (e: MouseEvent) => {
        e.stopPropagation();
        workspace.close(path);
      },
    }, icon('close'), h('span.dot')));
    tabs.appendChild(tab);
    if (active) requestAnimationFrame(() => tab.scrollIntoView({ block: 'nearest', inline: 'nearest' }));
  }
}

// ---------------------------------------------------------------- breadcrumbs

let symbolTrail: string[] = [];

function renderBreadcrumbs() {
  const path = workspace.active;
  layout.breadcrumbs.replaceChildren();
  layout.breadcrumbs.style.display = path ? '' : 'none';
  if (!path) return;
  const parts = relativePath(workspace.root, path).split('/');
  parts.forEach((part, i) => {
    if (i) layout.breadcrumbs.appendChild(icon('chevron-right', 'sep'));
    layout.breadcrumbs.appendChild(h('span.crumb', {}, part));
  });
  for (const symbol of symbolTrail) {
    layout.breadcrumbs.append(icon('chevron-right', 'sep'), h('span.crumb', {}, symbol));
  }
}

const updateSymbolTrail = debounce(async () => {
  const model = workspace.editor.getModel();
  const position = workspace.editor.getPosition();
  if (!model || !position || !lsp.running || model.getLanguageId() !== 'rust') {
    if (symbolTrail.length) {
      symbolTrail = [];
      renderBreadcrumbs();
    }
    return;
  }
  try {
    const symbols: any[] = (await lsp.request('textDocument/documentSymbol', { textDocument: { uri: model.uri.toString() } })) ?? [];
    const trail: string[] = [];
    const lspPos = toLspPosition(position);
    const inside = (r: any) =>
      (r.start.line < lspPos.line || (r.start.line === lspPos.line && r.start.character <= lspPos.character)) &&
      (r.end.line > lspPos.line || (r.end.line === lspPos.line && r.end.character >= lspPos.character));
    let level = symbols;
    while (level?.length) {
      const hit = level.find((s) => s.range && inside(s.range));
      if (!hit) break;
      trail.push(hit.name);
      level = hit.children ?? [];
    }
    symbolTrail = trail;
    renderBreadcrumbs();
  } catch {
    // Keep the old trail.
  }
}, 250);

// ---------------------------------------------------------------- speed score

const speedRing = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
speedRing.setAttribute('viewBox', '0 0 40 40');
speedRing.innerHTML = `
  <circle class="ring-bg" cx="20" cy="20" r="16" fill="none" stroke-width="4"/>
  <circle class="ring" cx="20" cy="20" r="16" fill="none" stroke-width="4" stroke-linecap="round"
    transform="rotate(-90 20 20)" stroke-dasharray="100.5" stroke-dashoffset="100.5"/>
  <text class="score" x="20" y="24.5" text-anchor="middle">–</text>`;
const speedLabel = h('div.label', {}, h('b', {}, 'Speed'), h('span', {}, ''));
const speed = h('div.speed.hidden', { title: 'Speed score of this file — click for details' }, speedRing, speedLabel);
let popup: HTMLElement | null = null;

function speedColor(score: number) {
  if (score >= 90) return '#3fb950';
  if (score >= 75) return '#d29922';
  if (score >= 50) return '#e8590c';
  return '#f14c4c';
}

function speedWord(score: number) {
  if (score >= 95) return 'Excellent';
  if (score >= 85) return 'Very good';
  if (score >= 70) return 'Good';
  if (score >= 50) return 'Could be faster';
  return 'Slow spots';
}

function renderSpeed(result: SpeedScore | null) {
  const visible = !!result && !!workspace.active && pathKey(result.path) === pathKey(workspace.active);
  speed.classList.toggle('hidden', !visible);
  if (!visible || !result) {
    popup?.remove();
    popup = null;
    return;
  }
  const ring = speedRing.querySelector('.ring') as SVGCircleElement;
  ring.style.stroke = speedColor(result.score);
  ring.style.strokeDashoffset = String(100.5 * (1 - result.score / 100));
  (speedRing.querySelector('.score') as SVGTextElement).textContent = String(result.score);
  (speedLabel.querySelector('span') as HTMLElement).textContent =
    `${speedWord(result.score)}${result.findings.length ? ` · ${result.findings.length} tip${result.findings.length > 1 ? 's' : ''}` : ''}`;
  if (popup) showSpeedDetails(result);
}

function showSpeedDetails(result: SpeedScore) {
  popup?.remove();
  popup = h('div.speed-popup', {},
    h('h3', {}, `⚡ Speed score ${result.score}/100 — ${speedWord(result.score)}`),
    result.findings.length
      ? result.findings.map((f) => h('div.finding', {
          onclick: () => {
            const r = toRange(f.range);
            workspace.editor.setSelection(r);
            workspace.editor.revealRangeInCenter(r, monaco.editor.ScrollType.Smooth);
            workspace.editor.focus();
          },
        }, h('span.penalty', {}, `−${f.penalty}`),
        h('div.text', {}, h('span', {}, f.message), h('span.fix', {}, `💡 ${f.fix}`), h('span.where', {}, `Line ${f.range.start.line + 1}`))))
      : h('div.empty', {}, 'No slow patterns found in this file. 🚀'),
    h('div.empty', { style: { fontSize: '11px' } },
      'The score reads the code for patterns known to be slow (copies and allocations in loops, list searches in loops…). ',
      'For real timings use Cargo → Run fast.'));
  layout.editorHost.appendChild(popup);
}

speed.addEventListener('click', (e) => {
  e.stopPropagation();
  const result = currentSpeed();
  if (popup) {
    popup.remove();
    popup = null;
  } else if (result) showSpeedDetails(result);
});
document.addEventListener('mousedown', (e) => {
  if (popup && !popup.contains(e.target as Node) && !speed.contains(e.target as Node)) {
    popup.remove();
    popup = null;
  }
});

// ---------------------------------------------------------------- welcome

const welcome = h('div.welcome');

export async function renderWelcome() {
  const info = workspace.info;
  const tc = info.toolchain;
  let recent: string[] = [];
  try {
    recent = await backend.call<string[]>('recent.get');
  } catch {
    // None.
  }
  const link = (iconName: string, label: string, run: () => void, muted?: string) =>
    h('button.link', { onclick: run }, icon(iconName), label, muted ? h('span.muted', {}, muted) : null);

  const toolRow = (ok: boolean, text: string, detail?: string) =>
    h('div.row', { style: { cursor: 'default', background: 'none', height: 'auto' } },
      icon(ok ? 'pass-filled' : 'warning', ok ? 'ok' : 'bad'), h('span', {}, text), detail ? h('code', {}, detail) : null);

  welcome.replaceChildren(h('div.welcome-inner', {},
    h('div.brand', {}, h('img', { src: './logo.svg', alt: '' }),
      h('div', {}, h('h1', {}, 'DEVR+'), h('p', {}, 'The editor for Rust++ — fast, clear, made for the language.'))),
    h('div.columns', {},
      h('div', {},
        h('section', {}, h('h2', {}, 'Start'),
          link('add', 'New Rust++ Project…', () => runCommand('project.new')),
          link('folder-opened', 'Open Folder…', () => runCommand('folder.open'), keyLabel('Ctrl+O')),
          link('symbol-class', 'Find a Symbol', () => runCommand('symbols.find'), keyLabel('Ctrl+T')),
          link('terminal', 'New Terminal', () => runCommand('terminal.new'), keyLabel('Ctrl+`'))),
        h('section', {}, h('h2', {}, 'Recent'),
          recent.length
            ? recent.slice(0, 8).map((path) => link('folder', basename(path), () => workspace.openFolder(path), path))
            : h('div.muted', {}, 'Projects you open appear here.'))),
      h('div', {},
        h('section', {}, h('h2', {}, 'Rust++ on this computer'),
          h('div.toolchain-card', {},
            toolRow(tc.isRustpp, tc.isRustpp ? 'Rust++ compiler' : tc.rustc ? 'A compiler was found, but it is not Rust++' : 'Rust++ is not installed', tc.rustcVersion ?? undefined),
            toolRow(!!tc.cargo, tc.cargo ? 'Cargo' : 'Cargo was not found'),
            toolRow(!!tc.analyzer, tc.analyzer ? 'rust++-analyzer (errors, completion, search)' : 'rust++-analyzer was not found'),
            !tc.isRustpp || !tc.analyzer
              ? h('div', { style: { marginTop: '6px' } }, h('button.btn.primary.small', { onclick: () => runCommand('rustpp.install') }, icon('cloud-download'), 'Install / Update Rust++'))
              : null)),
        h('section', {}, h('h2', {}, 'Rust++ in one minute'),
          h('div.muted', { style: { lineHeight: '1.7' } },
            h('div', {}, h('span.kbd', {}, 'out!("…")'), ' prints a line'),
            h('div', {}, h('span.kbd', {}, 'area(width = 3, height = 4)'), ' named arguments'),
            h('div', {}, h('span.kbd', {}, 'fn open(path: &str, write: bool = false)'), ' default values'),
            h('div', {}, h('span.kbd', {}, '-> &{x, y} str'), ' borrow sources'),
            h('div', {}, h('span.kbd', {}, 'struct P<_a>'), ' lifetimes without quotes'),
            link('book', 'Read the Doc Book', () => openUrl('https://alzelemx.github.io/Rust-/'))))))));
}

function openUrl(url: string) {
  if (backend.kind === 'desktop') {
    import('@tauri-apps/plugin-opener').then((m) => m.openUrl(url)).catch(() => window.open(url, '_blank'));
  } else window.open(url, '_blank');
}

function updateWelcome() {
  welcome.classList.toggle('hidden', !!workspace.active);
}

// ----------------------------------------------------------------

export function setupEditorArea() {
  layout.tabsBar.append(tabs, tabActions);
  layout.editorHost.append(welcome, speed);
  workspace.onTabs.on(() => {
    renderTabs();
    updateWelcome();
  });
  workspace.onActive.on(() => {
    renderTabs();
    symbolTrail = [];
    renderBreadcrumbs();
    updateSymbolTrail();
    updateWelcome();
    renderSpeed(currentSpeed());
  });
  workspace.onDirty.on(debounce(() => renderTabs(), 50));
  workspace.onRoot.on(() => renderWelcome());
  onDiagnostics.on(debounce(() => renderTabs(), 200));
  onSpeed.on((result) => renderSpeed(result));
  workspace.editor.onDidChangeCursorPosition(() => updateSymbolTrail());
  renderBreadcrumbs();
  renderWelcome();
  updateWelcome();
}
