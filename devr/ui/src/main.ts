// DEVR+: the editor for Rodet.
// First: the interface language, and Monaco's Arabic texts (before Monaco loads).
import './i18n/monaco-nls';
import { t } from './i18n';
import '@vscode/codicons/dist/codicon.css';
import './styles.css';
import * as monaco from './editor/monaco';
import EditorWorker from 'monaco-esm/editor/editor.worker.js?worker';
import { backend, errorText } from './backend';
import { registerLanguages, registerThemes } from './editor/language';
import {
  configurationChanged,
  lsp,
  refreshEditorFeatures,
  restartLanguageServer,
  setupLanguageFeatures,
  startLanguageServer,
} from './lsp/features';
import { loadSettings, onSettings, settings, updateSettings } from './settings';
import { layout } from './ui/layout';
import { confirm, prompt, toast } from './ui/notify';
import { commandPalette, keybindings, quickOpen, registerCommand, runCommand, symbolPicker } from './ui/palette';
import { basename, debounce, h, isMac } from './util';
import { silk, newProject, setupSilk } from './views/silk';
import { renderWelcome, setupEditorArea } from './views/editor-area';
import { explorer, setupExplorer } from './views/explorer';
import { outputPanel, problemsPanel, setupProblems } from './views/problems';
import { search, searchFor, setupSearch } from './views/search';
import { openSettings } from './views/settings-dialog';
import { renderToolchain, setupStatusbar } from './views/statusbar';
import { setupSymbols, symbols } from './views/symbols';
import { newTerminal, runRodet, runTask, setupTerminal, terminalPanel } from './views/terminal';
import { workspace } from './workspace';
import { openBook, reloadBook } from './views/book';
import { applyTheme, themes } from './theme';
import { pluginsView, setupPlugins, startPlugins } from './views/plugins';
import { pluginEditorOptions } from './plugins/api';

(self as any).MonacoEnvironment = {
  getWorker: () => new EditorWorker(),
};

function editorOptions(): monaco.editor.IEditorOptions & monaco.editor.IGlobalEditorOptions {
  return {
    fontFamily: settings.fontFamily,
    fontSize: settings.fontSize,
    fontLigatures: true,
    tabSize: settings.tabSize,
    insertSpaces: true,
    wordWrap: settings.wordWrap ? 'on' : 'off',
    minimap: { enabled: settings.minimap, renderCharacters: false, scale: 1 },
    'semanticHighlighting.enabled': true,
    bracketPairColorization: { enabled: true },
    guides: { bracketPairs: 'active', indentation: true },
    smoothScrolling: true,
    cursorBlinking: 'smooth',
    cursorSmoothCaretAnimation: 'on',
    renderLineHighlight: 'all',
    stickyScroll: { enabled: true },
    inlayHints: { enabled: 'on', padding: true },
    fixedOverflowWidgets: true,
    automaticLayout: true,
    padding: { top: 8 },
    scrollBeyondLastLine: true,
    suggest: { showStatusBar: true, preview: true },
    quickSuggestions: { other: true, comments: false, strings: false },
    lightbulb: { enabled: monaco.editor.ShowLightbulbIconMode.OnCode },
    renderWhitespace: 'selection',
    mouseWheelZoom: true,
    codeLens: true,
    links: true,
  };
}

async function main() {
  const app = document.getElementById('app')!;
  registerLanguages();
  registerThemes();
  layout.mount(app);

  try {
    await workspace.init();
  } catch (err) {
    app.replaceChildren(h('div.empty-state', {}, `${t('DEVR+ couldn’t reach its backend:')} ${errorText(err)}`));
    return;
  }
  await loadSettings();
  applyTheme();

  // Code is always written left to right, also in Arabic.
  const editorElement = h('div.monaco', { dir: 'ltr' });
  layout.editorHost.appendChild(editorElement);
  workspace.editor = monaco.editor.create(editorElement, { model: null, ...editorOptions() });
  // For automated UI tests in a browser (devr-server).
  if (backend.kind === 'web') (window as any).__devr = { monaco, workspace };

  // Go to definition / peek into other files opens them in a tab.
  monaco.editor.registerEditorOpener({
    openCodeEditor(_source, resource, selectionOrPosition) {
      const sel = selectionOrPosition as any;
      const reveal = !sel ? undefined : 'startLineNumber' in sel
        ? { line: sel.startLineNumber, column: sel.startColumn, endLine: sel.endLineNumber, endColumn: sel.endColumn }
        : { line: sel.lineNumber, column: sel.column };
      workspace.open(resource.fsPath, reveal);
      return true;
    },
  });

  layout.addSideView(explorer);
  layout.addSideView(search);
  layout.addSideView(symbols);
  layout.addSideView(silk);
  layout.addSideView(pluginsView);
  layout.addActivityButton(t('Doc Book'), 'book', () => openBook());
  layout.addActivityButton(t('Settings (Ctrl+,)'), 'settings-gear', () => openSettings());
  layout.addPanelView(problemsPanel);
  layout.addPanelView(outputPanel);
  layout.addPanelView(terminalPanel);

  setupLanguageFeatures();
  setupEditorArea();
  setupExplorer();
  setupSearch();
  setupSymbols();
  setupSilk();
  setupProblems();
  setupTerminal();
  setupStatusbar();
  registerCommands();
  setupKeys();
  setupPlugins();

  layout.showSide('explorer');
  layout.togglePanel(false);
  layout.showPanel('problems');
  layout.togglePanel(false);

  onSettings.on(() => {
    applyTheme();
    workspace.editor.updateOptions({ ...editorOptions(), ...pluginEditorOptions() });
    configurationChanged();
    refreshEditorFeatures();
  });

  const autoSave = debounce(() => {
    if (settings.autoSave) workspace.saveAll();
  }, 1000);
  workspace.onDirty.on(() => autoSave());

  workspace.onRoot.on(async (root) => {
    document.title = root ? `${basename(root)} — DEVR+` : 'DEVR+';
    try {
      localStorage.setItem('devr.lastFolder', root ?? '');
    } catch {
      // No storage.
    }
    if (root) await startLanguageServer(root);
    else await lsp.stop();
  });

  lsp.onState.on((state) => {
    if (state === 'missing') {
      toast(t('rodet-analyzer was not found, so errors, completion and search are off. Install or update Rodet to turn them on.'), 'warning', [
        { label: t('Install Rodet'), primary: true, run: () => runCommand('rodet.install') },
      ]);
    } else if (state === 'error') {
      toast(t('The language server stopped.'), 'error', [
        { label: t('Restart'), primary: true, run: () => restartLanguageServer() },
        { label: t('Show Output'), run: () => layout.showPanel('output') },
      ]);
    }
  });

  setupWindowClose();
  checkUpdates();
  // Plugins start once the workbench is ready, before the last folder opens.
  await startPlugins();

  let last = '';
  try {
    last = localStorage.getItem('devr.lastFolder') ?? '';
  } catch {
    // No storage.
  }
  if (last) {
    const stat = await backend.call<{ exists: boolean; isDir: boolean }>('fs.stat', { path: last }).catch(() => null);
    if (stat?.isDir) await workspace.openFolder(last);
  }
}

export async function openFolderDialog() {
  let path: string | null = null;
  if (backend.kind === 'desktop') {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const picked = await open({ directory: true, multiple: false, title: t('Open Folder') });
    path = typeof picked === 'string' ? picked : null;
  } else {
    path = await prompt(t('Open Folder (full path on the computer running devr-server)'), workspace.root ?? workspace.info.home ?? '');
  }
  if (!path) return;
  const stat = await backend.call<{ exists: boolean; isDir: boolean }>('fs.stat', { path });
  if (!stat.isDir) {
    toast(t('{path} is not a folder.', { path }), 'error');
    return;
  }
  await workspace.openFolder(path);
}

function installRodet() {
  const windows = workspace.info.os === 'windows';
  const command = windows
    ? ['powershell.exe', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command',
        "$f = \"$env:TEMP\\rodet_init.exe\"; Invoke-WebRequest https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.exe -OutFile $f; & $f -y"]
    : ['sh', '-c', 'curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh'];
  runTask('Install Rodet', command, workspace.info.home ?? '.');
  const off = backend.on('term.exit', async () => {
    off();
    await workspace.refreshToolchain();
    renderToolchain();
    renderWelcome();
    reloadBook();
    if (workspace.root) restartLanguageServer();
  });
}

function registerCommands() {
  const C = registerCommand;
  C({ id: 'folder.open', title: t('File: Open Folder…'), key: 'Ctrl+O', icon: 'folder-opened', run: () => openFolderDialog() });
  C({ id: 'folder.close', title: t('File: Close Folder'), icon: 'close', run: () => workspace.closeFolder() });
  C({ id: 'project.new', title: t('Rodet: New Project…'), icon: 'add', run: () => newProject() });
  C({ id: 'file.new', title: t('File: New File…'), key: 'Ctrl+N', icon: 'new-file', run: () => newFile() });
  C({ id: 'file.save', title: t('File: Save'), key: 'Ctrl+S', icon: 'save', run: () => workspace.save() });
  C({ id: 'file.saveAll', title: t('File: Save All'), key: 'Ctrl+Alt+S', icon: 'save-all', run: () => workspace.saveAll() });
  C({ id: 'file.close', title: t('View: Close Editor'), key: 'Ctrl+W', icon: 'close', run: () => workspace.active && workspace.close(workspace.active) });
  C({ id: 'file.quickOpen', title: t('Go to File…'), key: 'Ctrl+P', icon: 'go-to-file', run: () => quickOpen() });
  C({ id: 'symbols.find', title: t('Go to Symbol in Project…'), key: 'Ctrl+T', icon: 'symbol-class', run: () => symbolPicker() });
  C({ id: 'symbols.view', title: t('View: Find Symbols (struct, impl, fn…)'), key: 'Ctrl+Shift+T', icon: 'symbol-class', run: () => layout.showSide('symbols') });
  C({ id: 'symbols.file', title: t('Go to Symbol in File…'), key: 'Ctrl+Shift+O', icon: 'symbol-method', run: () => editorAction('editor.action.quickOutline') });
  C({ id: 'palette', title: t('Show All Commands'), key: 'Ctrl+Shift+P', icon: 'terminal', run: () => commandPalette() });
  C({ id: 'view.explorer', title: t('View: Explorer'), key: 'Ctrl+Shift+E', icon: 'files', run: () => layout.showSide('explorer') });
  C({ id: 'view.search', title: t('View: Search in Files'), key: 'Ctrl+Shift+F', icon: 'search', run: () => {
    layout.showSide('search');
    const selection = workspace.editor.getSelection();
    const model = workspace.editor.getModel();
    if (selection && model && !selection.isEmpty() && selection.startLineNumber === selection.endLineNumber) searchFor(model.getValueInRange(selection));
  } });
  C({ id: 'view.silk', title: t('View: Silk'), icon: 'package', run: () => layout.showSide('silk') });
  C({ id: 'view.sidebar', title: t('View: Toggle Side Bar'), key: 'Ctrl+B', icon: 'layout-sidebar-left', run: () => layout.toggleSidebar() });
  C({ id: 'view.panel', title: t('View: Toggle Panel'), key: 'Ctrl+J', icon: 'layout-panel', run: () => layout.togglePanel() });
  C({ id: 'view.problems', title: t('View: Problems'), key: 'Ctrl+Shift+M', icon: 'error', run: () => layout.showPanel('problems') });
  C({ id: 'view.output', title: t('View: Output'), icon: 'output', run: () => layout.showPanel('output') });
  C({ id: 'terminal.new', title: t('Terminal: New Terminal'), key: 'Ctrl+`', icon: 'terminal', run: () => newTerminal() });
  C({ id: 'rodet.run', title: t('Rodet: Run'), key: 'F5', icon: 'play', run: () => runRodet(['run'], 'rodet run') });
  C({ id: 'rodet.runRelease', title: t('Rodet: Run Fast (release)'), key: 'Ctrl+F5', icon: 'rocket', run: () => runRodet(['run', '--release'], 'rodet run --release') });
  C({ id: 'rodet.build', title: t('Rodet: Build'), key: 'Ctrl+Shift+B', icon: 'tools', run: () => runRodet(['build']) });
  C({ id: 'rodet.check', title: t('Rodet: Check'), icon: 'check', run: () => runRodet(['check']) });
  C({ id: 'rodet.test', title: t('Rodet: Test'), icon: 'beaker', run: () => runRodet(['test']) });
  C({ id: 'rodet.clean', title: t('Rodet: Clean'), icon: 'trash', run: () => runRodet(['clean']) });
  C({ id: 'book.open', title: t('Help: Doc Book'), icon: 'book', run: () => openBook() });
  C({ id: 'settings', title: t('Preferences: Settings'), key: 'Ctrl+,', icon: 'settings-gear', run: () => openSettings() });
  C({ id: 'theme.toggle', title: t('Preferences: Toggle Dark / Light Theme'), icon: 'color-mode', run: () => updateSettings({ theme: (themes.get(settings.theme)?.base ?? settings.theme) === 'dark' ? 'light' : 'dark' }) });
  C({ id: 'analyzer.restart', title: t('Rodet: Restart Language Server'), icon: 'refresh', run: () => restartLanguageServer() });
  C({ id: 'analyzer.menu', title: t('Rodet: Language Server Status'), icon: 'info', run: () => {
    if (lsp.state === 'error' || lsp.state === 'stopped') restartLanguageServer();
    else layout.showPanel('output');
  } });
  C({ id: 'rodet.install', title: t('Rodet: Install or Update Rodet'), icon: 'cloud-download', run: () => installRodet() });
  C({ id: 'rodet.about', title: t('Rodet: About This Toolchain'), icon: 'info', run: () => {
    const tc = workspace.info.toolchain;
    toast(`${tc.rodetcVersion ?? t('Rodet is not installed')}\nrodetc: ${tc.rodetc ?? '–'}\nrodet: ${tc.rodet ?? '–'}\nanalyzer: ${tc.analyzer ?? '–'}`, 'info', [
      { label: t('Install / Update'), run: () => installRodet() },
    ]);
  } });
  C({ id: 'editor.rename', title: t('Rename Symbol'), key: 'F2', icon: 'edit', run: () => editorAction('editor.action.rename') });
  C({ id: 'editor.definition', title: t('Go to Definition'), key: 'F12', icon: 'go-to-file', run: () => editorAction('editor.action.revealDefinition') });
  C({ id: 'editor.references', title: t('Find All References'), key: 'Shift+F12', icon: 'references', run: () => editorAction('editor.action.goToReferences') });
  C({ id: 'editor.implementations', title: t('Go to Implementations'), key: 'Ctrl+F12', icon: 'symbol-interface', run: () => editorAction('editor.action.goToImplementation') });
  C({ id: 'editor.quickFix', title: t('Quick Fix…'), key: 'Ctrl+.', icon: 'lightbulb', run: () => editorAction('editor.action.quickFix') });
  C({ id: 'editor.nextProblem', title: t('Go to Next Problem'), key: 'F8', icon: 'arrow-down', run: () => editorAction('editor.action.marker.nextInFiles') });
  C({ id: 'editor.format', title: t('Format Document'), key: 'Shift+Alt+F', icon: 'whole-word', run: () => editorAction('editor.action.formatDocument') });
}

function editorAction(id: string) {
  workspace.editor.focus();
  workspace.editor.getAction(id)?.run();
}

async function newFile() {
  const root = workspace.root;
  if (!root) {
    toast(t('Open a folder first.'), 'warning');
    return;
  }
  const name = await prompt(t('New file (path in the project)'), 'src/');
  if (!name) return;
  const sep = workspace.info.pathSeparator;
  const path = root + sep + name.replace(/[\\/]/g, sep);
  try {
    await backend.call('fs.createFile', { path });
    workspace.open(path);
  } catch (err) {
    toast(errorText(err), 'error');
  }
}

/** Keyboard shortcuts of the workbench (the editor's own keys stay in Monaco). */
function setupKeys() {
  const defaults: Record<string, string> = {
    'Ctrl+O': 'folder.open',
    'Ctrl+N': 'file.new',
    'Ctrl+S': 'file.save',
    'Ctrl+Alt+S': 'file.saveAll',
    'Ctrl+W': 'file.close',
    'Ctrl+P': 'file.quickOpen',
    'Ctrl+T': 'symbols.find',
    'Ctrl+Shift+T': 'symbols.view',
    'Ctrl+Shift+O': 'symbols.file',
    'Ctrl+Shift+P': 'palette',
    'F1': 'palette',
    'Ctrl+Shift+E': 'view.explorer',
    'Ctrl+Shift+F': 'view.search',
    'Ctrl+Shift+X': 'view.plugins',
    'Ctrl+B': 'view.sidebar',
    'Ctrl+J': 'view.panel',
    'Ctrl+Shift+M': 'view.problems',
    'Ctrl+`': 'terminal.new',
    'F5': 'rodet.run',
    'Ctrl+F5': 'rodet.runRelease',
    'Ctrl+Shift+B': 'rodet.build',
    'Ctrl+,': 'settings',
  };
  for (const [combo, id] of Object.entries(defaults)) keybindings.set(combo, id);
  window.addEventListener('keydown', (e) => {
    const ctrl = isMac ? e.metaKey : e.ctrlKey;
    const key = e.key.length === 1 ? e.key.toUpperCase() : e.key;
    const combo = `${ctrl ? 'Ctrl+' : ''}${e.altKey ? 'Alt+' : ''}${e.shiftKey ? 'Shift+' : ''}${key === 'Dead' ? '`' : key}`;
    const id = keybindings.get(combo) ?? (e.code === 'Backquote' && ctrl ? keybindings.get('Ctrl+`') : undefined);
    if (!id) return;
    // Inside a terminal, keep Ctrl+letters for the shell (Ctrl+C, Ctrl+W…), except ours that matter.
    const inTerminal = (e.target as HTMLElement | null)?.closest?.('.xterm');
    if (inTerminal && !['Ctrl+`', 'Ctrl+J', 'Ctrl+P', 'Ctrl+Shift+P', 'F5', 'Ctrl+B'].includes(combo)) return;
    e.preventDefault();
    e.stopPropagation();
    runCommand(id);
  }, true);
}

function setupWindowClose() {
  if (backend.kind === 'desktop') {
    import('@tauri-apps/api/window').then(({ getCurrentWindow }) => {
      const win = getCurrentWindow();
      win.onCloseRequested(async (event) => {
        if (!workspace.dirtyDocs.length) {
          await lsp.stop();
          return;
        }
        event.preventDefault();
        const ok = await confirm(t('Quit with {n} unsaved file(s)?', { n: workspace.dirtyDocs.length }), t('Unsaved changes will be lost.'), t('Quit'), true);
        if (ok) {
          await lsp.stop();
          await win.destroy();
        }
      });
    });
  } else {
    window.addEventListener('beforeunload', (e) => {
      if (workspace.dirtyDocs.length) e.preventDefault();
    });
    backend.on('backend.disconnected', () => toast(t('Disconnected from devr-server. Reload the page to reconnect.'), 'error', [], 0));
  }
}

async function checkUpdates() {
  try {
    const updates = await backend.call<{ devrLatest: { tag: string; url: string } | null; devrCurrent: string }>('updates.check');
    const latest = updates.devrLatest?.tag.replace(/^devr-v/, '');
    if (latest && newer(latest, updates.devrCurrent)) {
      toast(t('DEVR+ {latest} is available (you have {current}).', { latest, current: updates.devrCurrent }), 'info', [
        { label: t('Download'), primary: true, run: () => import('@tauri-apps/plugin-opener').then((m) => m.openUrl(updates.devrLatest!.url)).catch(() => window.open(updates.devrLatest!.url, '_blank')) },
        { label: t('Later'), run: () => {} },
      ]);
    }
  } catch {
    // Offline.
  }
}

function newer(a: string, b: string) {
  const pa = a.split('.').map(Number);
  const pb = b.split('.').map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    if ((pa[i] ?? 0) !== (pb[i] ?? 0)) return (pa[i] ?? 0) > (pb[i] ?? 0);
  }
  return false;
}

main();
