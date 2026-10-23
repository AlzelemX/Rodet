// The status bar: Rust++ version, language server state, errors and
// warnings, cursor position.
import { t } from '../i18n';
import { diagnosticCounts, lsp, onDiagnostics, onProgress } from '../lsp/features';
import { layout } from '../ui/layout';
import { runCommand } from '../ui/palette';
import { h, icon } from '../util';
import { workspace } from '../workspace';

const brand = h('button.status-item.brand', { title: t('DEVR+ — show the welcome page'), onclick: () => workspace.show(null) }, 'DEVR+');
const problems = h('button.status-item', { title: t('Problems (Ctrl+Shift+M)'), onclick: () => layout.showPanel('problems') });
const server = h('button.status-item', { title: t('rust++-analyzer'), onclick: () => runCommand('analyzer.menu') });
const toolchain = h('button.status-item', { title: t('Rust++ toolchain'), onclick: () => runCommand('rustpp.about') });
const position = h('button.status-item', { title: t('Go to Line (Ctrl+G)'), onclick: () => {
  workspace.editor.focus();
  workspace.editor.trigger('statusbar', 'editor.action.gotoLine', null);
} });
const language = h('span.status-item');

function renderProblems() {
  const { errors, warnings } = diagnosticCounts();
  problems.className = `status-item${errors ? ' error' : ''}${warnings ? ' warning' : ''}`;
  problems.replaceChildren(icon('error'), String(errors), icon('warning'), String(warnings));
}

let progressText = '';
function renderServer() {
  const state = lsp.state;
  const map: Record<string, [string, string, boolean]> = {
    stopped: ['circle-slash', t('Analyzer off'), false],
    starting: ['loading', t('Starting…'), true],
    indexing: ['loading', progressText || t('Indexing…'), true],
    ready: ['check', 'Rust++', false],
    error: ['error', t('Analyzer stopped'), false],
    missing: ['warning', t('No analyzer'), false],
  };
  const [iconName, text, spinning] = map[state] ?? map.stopped;
  server.replaceChildren(icon(iconName, spinning ? 'spin' : ''), text);
  server.title = state === 'missing'
    ? t('rust++-analyzer was not found: install or update Rust++')
    : state === 'error'
      ? `${t('The language server stopped.')}${lsp.lastError ? ` ${lsp.lastError}` : ''} ${t('Click to restart.')}`
      : `rust++-analyzer${lsp.serverPath ? `: ${lsp.serverPath}` : ''}`;
}

export function renderToolchain() {
  const tc = workspace.info.toolchain;
  const version = tc.rustcVersion?.replace(/^rustc\s+/, '') ?? 'not installed';
  // `1.101.0-dev (Rust++ #19)` → `1.101.0-dev #19`
  const short = version.replace(/\s*\(Rust\+\+\s*(#\d+)?\)/, (_m, build) => (build ? ` ${build}` : ''));
  toolchain.replaceChildren(icon(tc.isRustpp ? 'symbol-namespace' : 'warning'), tc.isRustpp ? short : `Rust++ ${tc.rustcVersion ? version : t('not installed')}`);
}

function renderPosition() {
  const pos = workspace.editor.getPosition();
  const model = workspace.editor.getModel();
  position.style.display = model ? '' : 'none';
  language.style.display = model ? '' : 'none';
  if (!pos || !model) return;
  const selection = workspace.editor.getSelection();
  const selected = selection && !selection.isEmpty() ? ` (${t('{n} selected', { n: model.getValueInRange(selection).length })})` : '';
  position.textContent = `${t('Ln {line}, Col {col}', { line: pos.lineNumber, col: pos.column })}${selected}`;
  const lang = model.getLanguageId();
  language.textContent = lang === 'rust' ? 'Rust++' : lang === 'toml' ? 'TOML' : lang === 'markdown' ? 'Markdown' : t('Plain Text');
}

export function setupStatusbar() {
  layout.statusLeft.append(brand, toolchain, server, problems);
  layout.statusRight.append(position, h('span.status-item', {}, 'UTF-8'), language);
  onDiagnostics.on(renderProblems);
  lsp.onState.on(renderServer);
  onProgress.on((p) => {
    progressText = p ? `${p.title}${p.message ? ` ${p.message}` : ''}${p.percentage !== undefined ? ` ${p.percentage}%` : ''}` : '';
    if (progressText.length > 48) progressText = progressText.slice(0, 47) + '…';
    renderServer();
  });
  workspace.editor.onDidChangeCursorSelection(renderPosition);
  workspace.onActive.on(renderPosition);
  renderProblems();
  renderServer();
  renderToolchain();
  renderPosition();
}
