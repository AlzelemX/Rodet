// Tabs, breadcrumbs, the speed score next to Run, and the welcome page.
import * as monaco from '../editor/monaco';
import { backend } from '../backend';
import { t } from '../i18n';
import { translateTip } from '../i18n/messages';
import { currentSpeed, diagnostics, lsp, onDiagnostics, onSpeed, type SpeedScore, toLspPosition, toRange } from '../lsp/features';
import { settings } from '../settings';
import { contextMenu, layout } from '../ui/layout';
import { runCommand } from '../ui/palette';
import { basename, debounce, fileIcon, h, icon, keyLabel, relativePath, richText } from '../util';
import { pathKey, workspace } from '../workspace';
import { openBook, setupBook } from './book';

// ---------------------------------------------------------------- speed score

/** The speed score: a small item next to Run; hovering it shows the tips. */
const speedValue = h('b');
const speed = h('button.speed-chip.hidden', { onclick: () => togglePinned() }, icon('zap'), speedValue);
let popup: HTMLElement | null = null;
let pinned = false;
let hideTimer: number | undefined;

function speedColor(score: number) {
  if (score >= 90) return '#3fb950';
  if (score >= 75) return '#d29922';
  if (score >= 50) return '#e8590c';
  return '#f14c4c';
}

function speedWord(score: number) {
  if (score >= 95) return t('Excellent');
  if (score >= 85) return t('Very good');
  if (score >= 70) return t('Good');
  if (score >= 50) return t('Could be faster');
  return t('Slow spots');
}

function renderSpeed(result: SpeedScore | null) {
  const visible = settings.speedScore && !!result && !!workspace.active && pathKey(result.path) === pathKey(workspace.active);
  speed.classList.toggle('hidden', !visible);
  if (!visible || !result) {
    closePopup();
    return;
  }
  speed.style.setProperty('--speed', speedColor(result.score));
  speedValue.textContent = String(result.score);
  if (popup) showSpeedDetails(result);
}

function showSpeedDetails(result: SpeedScore) {
  popup?.remove();
  popup = h('div.speed-popup', {
    onmouseenter: () => clearTimeout(hideTimer),
    onmouseleave: () => scheduleHide(),
  },
  h('h3', {}, `⚡ ${t('Speed score')} ${result.score}/100 — ${speedWord(result.score)}`),
  result.findings.length
    ? result.findings.map((f) => h('div.finding', {
        onclick: () => {
          const r = toRange(f.range);
          workspace.editor.setSelection(r);
          workspace.editor.revealRangeInCenter(r, monaco.editor.ScrollType.Smooth);
          workspace.editor.focus();
        },
      }, h('span.penalty', {}, `−${f.penalty}`),
      h('div.text', {}, h('span', {}, ...richText(translateTip(f.message))), h('span.fix', {}, '💡 ', ...richText(translateTip(f.fix))),
        h('span.where', {}, t('Line {n}', { n: f.range.start.line + 1 })))))
    : h('div.empty', {}, t('No slow patterns found in this file. 🚀')),
  h('div.empty', { style: { fontSize: '11px' } },
    t('The score reads the code for patterns known to be slow (copies and allocations in loops, list searches in loops…). For real timings use Cargo → Run fast.')));
  document.body.appendChild(popup);
  const r = speed.getBoundingClientRect();
  const width = popup.getBoundingClientRect().width;
  popup.style.top = `${r.bottom + 6}px`;
  popup.style.left = `${Math.max(8, Math.min(window.innerWidth - width - 8, r.right - width))}px`;
}

function closePopup() {
  popup?.remove();
  popup = null;
  pinned = false;
}

function scheduleHide() {
  if (pinned) return;
  clearTimeout(hideTimer);
  hideTimer = window.setTimeout(() => closePopup(), 250);
}

function togglePinned() {
  const result = currentSpeed();
  if (!result) return;
  if (pinned) {
    closePopup();
    return;
  }
  pinned = true;
  showSpeedDetails(result);
}

speed.addEventListener('mouseenter', () => {
  clearTimeout(hideTimer);
  const result = currentSpeed();
  if (result && !popup) showSpeedDetails(result);
});
speed.addEventListener('mouseleave', () => scheduleHide());
document.addEventListener('mousedown', (e) => {
  if (popup && !popup.contains(e.target as Node) && !speed.contains(e.target as Node)) closePopup();
});

// ---------------------------------------------------------------- tabs

const tabs = h('div.tabs');
const tabActions = h('div.tabs-actions', {},
  speed,
  h('button.icon-button.run', { title: t('Run (F5)'), onclick: () => runCommand('cargo.run') }, icon('play')),
  h('button.icon-button', { title: t('Find Symbols (Ctrl+T)'), onclick: () => runCommand('symbols.find') }, icon('symbol-class')),
  h('button.icon-button', { title: t('More'), onclick: (e: MouseEvent) => contextMenu(e, [
    { label: t('Doc Book'), icon: 'book', run: () => openBook() },
    'sep',
    { label: t('Close All'), icon: 'close-all', run: () => workspace.closeAll() },
    { label: t('Save All'), icon: 'save-all', key: keyLabel('Ctrl+Alt+S'), run: () => workspace.saveAll() },
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
    const page = workspace.pages.get(path);
    const dirty = !page && workspace.isDirty(path);
    const errors = !page && fileHasErrors(path);
    const fi = page ? { name: page.icon, color: 'var(--accent)' } : fileIcon(path);
    const fileIconEl = icon(fi.name);
    fileIconEl.style.color = fi.color;
    const rel = page ? page.title : relativePath(workspace.root, path);
    const name = page ? page.title : basename(path);
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
        { label: t('Close'), icon: 'close', key: keyLabel('Ctrl+W'), run: () => workspace.close(path) },
        { label: t('Close Others'), run: async () => { for (const x of [...workspace.tabs]) if (pathKey(x) !== pathKey(path)) if (!(await workspace.close(x))) break; } },
        { label: t('Close All'), icon: 'close-all', run: () => workspace.closeAll() },
        ...(page ? [] : ['sep' as const, { label: t('Copy Path'), icon: 'copy', run: () => navigator.clipboard.writeText(path) }]),
      ]),
    },
    fileIconEl,
    h(errors ? 'span.name.has-errors' : 'span.name', {}, name),
    !page && (names.get(basename(path)) ?? 0) > 1 ? h('span.hint', {}, rel.split('/').slice(-2, -1)[0] ?? '') : null,
    h('button.close', {
      title: t('Close'),
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
  layout.breadcrumbs.style.display = path && !workspace.isPage(path) ? '' : 'none';
  if (!path || workspace.isPage(path)) return;
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
      icon(ok ? 'pass-filled' : 'warning', ok ? 'ok' : 'bad'), h('span', {}, text), detail ? h('code', { dir: 'ltr' }, detail) : null);

  welcome.replaceChildren(h('div.welcome-inner', {},
    h('div.brand', {}, h('img', { src: './logo.svg', alt: '' }),
      h('div', {}, h('h1', { dir: 'ltr' }, 'DEVR+'), h('p', {}, t('The editor for Rust++ — fast, clear, made for the language.')))),
    h('div.columns', {},
      h('div', {},
        h('section', {}, h('h2', {}, t('Start')),
          link('add', t('New Rust++ Project…'), () => runCommand('project.new')),
          link('folder-opened', t('Open Folder…'), () => runCommand('folder.open'), keyLabel('Ctrl+O')),
          link('symbol-class', t('Find a Symbol'), () => runCommand('symbols.find'), keyLabel('Ctrl+T')),
          link('terminal', t('New Terminal'), () => runCommand('terminal.new'), keyLabel('Ctrl+`'))),
        h('section', {}, h('h2', {}, t('Recent')),
          recent.length
            ? recent.slice(0, 8).map((path) => link('folder', basename(path), () => workspace.openFolder(path), path))
            : h('div.muted', {}, t('Projects you open appear here.')))),
      h('div', {},
        h('section', {}, h('h2', {}, t('Rust++ on this computer')),
          h('div.toolchain-card', {},
            toolRow(tc.isRustpp, tc.isRustpp ? t('Rust++ compiler') : tc.rustc ? t('A compiler was found, but it is not Rust++') : t('Rust++ is not installed'), tc.rustcVersion ?? undefined),
            toolRow(!!tc.cargo, tc.cargo ? 'Cargo' : t('Cargo was not found')),
            toolRow(!!tc.analyzer, tc.analyzer ? t('rust++-analyzer (errors, completion, search)') : t('rust++-analyzer was not found')),
            !tc.isRustpp || !tc.analyzer
              ? h('div', { style: { marginTop: '6px' } }, h('button.btn.primary.small', { onclick: () => runCommand('rustpp.install') }, icon('cloud-download'), t('Install / Update Rust++')))
              : null)),
        h('section', {}, h('h2', {}, t('Learn')),
          link('book', t('Doc Book'), () => openBook(), t('The guide to Rust++, for your version')),
          link('settings-gear', t('Settings'), () => runCommand('settings'), keyLabel('Ctrl+,')))))));
}

function updateWelcome() {
  welcome.classList.toggle('hidden', !!workspace.active);
}

// ----------------------------------------------------------------

export function setupEditorArea() {
  layout.tabsBar.append(tabs, tabActions);
  layout.editorHost.append(welcome);
  setupBook(layout.editorHost);
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
