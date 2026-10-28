// Connects rodet-analyzer to the editor: keeps it in sync with the open
// files, and gives Monaco completion, hover, go to definition, errors and
// fixes, renaming, inlay hints (with the Rodet ↩ return marks), semantic
// colors, Run lenses and the speed score.
import { arabic, isolateLatin, t } from '../i18n';
import { translateAction, translateDiagnostic, translateLens, translateTip } from '../i18n/messages';
import { STD_AR } from '../i18n/std-ar';
import * as monaco from '../editor/monaco';
import { backend, errorText } from '../backend';
import { RODET } from '../editor/language';
import { settings } from '../settings';
import { Emitter, debounce } from '../util';
import { toast } from '../ui/notify';
import { type Doc, pathKey, pathOfUri, uriOf, workspace } from '../workspace';
import {
  type LspDiagnostic,
  type LspLocation,
  type LspRange,
  type LspTextEdit,
  type LspWorkspaceEdit,
  LspClient,
  isCancelled,
} from './client';

export const lsp = new LspClient();

// ---------------------------------------------------------------- conversions

export function toRange(r: LspRange) {
  return new monaco.Range(r.start.line + 1, r.start.character + 1, r.end.line + 1, r.end.character + 1);
}

export function toLspPosition(p: monaco.IPosition) {
  return { line: p.lineNumber - 1, character: p.column - 1 };
}

export function toLspRange(r: monaco.IRange): LspRange {
  return {
    start: { line: r.startLineNumber - 1, character: r.startColumn - 1 },
    end: { line: r.endLineNumber - 1, character: r.endColumn - 1 },
  };
}

function lspUri(model: monaco.editor.ITextModel) {
  return model.uri.toString();
}

/** Rodet spelling in text from the language server: `'static` is `_static`. */
export function rodetText(text: string) {
  return text.replace(/'static\b/g, '_static');
}

function markdown(value: any): monaco.IMarkdownString | undefined {
  if (!value) return undefined;
  if (typeof value === 'string') return { value: rodetText(value) };
  return { value: rodetText(value.value ?? ''), isTrusted: false, supportThemeIcons: true };
}

const completionKinds: Record<number, monaco.languages.CompletionItemKind> = (() => {
  const K = monaco.languages.CompletionItemKind;
  return {
    1: K.Text, 2: K.Method, 3: K.Function, 4: K.Constructor, 5: K.Field, 6: K.Variable, 7: K.Class,
    8: K.Interface, 9: K.Module, 10: K.Property, 11: K.Unit, 12: K.Value, 13: K.Enum, 14: K.Keyword,
    15: K.Snippet, 16: K.Color, 17: K.File, 18: K.Reference, 19: K.Folder, 20: K.EnumMember,
    21: K.Constant, 22: K.Struct, 23: K.Event, 24: K.Operator, 25: K.TypeParameter,
  };
})();

// ---------------------------------------------------------------- state

export interface FileDiagnostics {
  path: string;
  uri: string;
  diagnostics: LspDiagnostic[];
}

/** Diagnostics of every file, by path key. */
export const diagnostics = new Map<string, FileDiagnostics>();
export const onDiagnostics = new Emitter<void>();

export interface Progress {
  title: string;
  message?: string;
  percentage?: number;
}
const progress = new Map<string | number, Progress>();
export const onProgress = new Emitter<Progress | null>();

export interface SpeedFinding {
  range: LspRange;
  penalty: number;
  message: string;
  fix: string;
}
export interface SpeedScore {
  path: string;
  score: number;
  findings: SpeedFinding[];
}
export const onSpeed = new Emitter<SpeedScore | null>();
let lastSpeed: SpeedScore | null = null;
export function currentSpeed() {
  return lastSpeed;
}

/** Runs a `rodet` command line in a terminal; set by the terminal panel. */
export let runInTerminal: (title: string, command: string[], cwd: string) => void = () => {};
export function setTerminalRunner(runner: typeof runInTerminal) {
  runInTerminal = runner;
}

// ---------------------------------------------------------------- configuration

function analyzerConfig() {
  return {
    checkOnSave: settings.checkOnSave,
    check: { command: 'check', allTargets: true },
    cargo: { buildScripts: { enable: true }, allTargets: true },
    procMacro: { enable: true },
    files: { watcher: 'server' },
    diagnostics: { enable: true, experimental: { enable: false } },
    inlayHints: {
      typeHints: { enable: settings.typeHints, hideClosureInitialization: true },
      chainingHints: { enable: settings.typeHints },
      parameterHints: { enable: settings.parameterHints },
      closureReturnTypeHints: { enable: 'never' },
      closingBraceHints: { enable: false },
      returnValueHints: { enable: settings.returnMarks },
      maxLength: 30,
    },
    lens: {
      enable: true,
      run: { enable: true },
      debug: { enable: false },
      implementations: { enable: true },
      references: { adt: { enable: false }, method: { enable: false }, trait: { enable: false } },
    },
    hover: { actions: { enable: false }, documentation: { enable: true } },
    semanticHighlighting: { strings: { enable: true }, punctuation: { enable: false } },
    completion: { autoimport: { enable: true }, callable: { snippets: 'fill_arguments' } },
    workspace: { symbol: { search: { kind: 'all_symbols', limit: 256 } } },
    // The standard library's docs carry Arabic next to the English.
    rodet: { language: arabic ? 'ar' : 'en' },
  };
}

export async function startLanguageServer(root: string) {
  await lsp.stop();
  clearAllDiagnostics();
  const ok = await lsp.start(root, analyzerConfig(), uriOf(root).toString());
  if (!ok) return;
  for (const doc of workspace.docs.values()) openInServer(doc);
}

export async function restartLanguageServer() {
  if (workspace.root) await startLanguageServer(workspace.root);
}

export function configurationChanged() {
  if (lsp.running) lsp.notify('workspace/didChangeConfiguration', { settings: { 'rust-analyzer': analyzerConfig() } });
}

function clearAllDiagnostics() {
  diagnostics.clear();
  for (const model of monaco.editor.getModels()) monaco.editor.setModelMarkers(model, 'rodet', []);
  onDiagnostics.fire();
}

// ---------------------------------------------------------------- documents

const synced = new Map<string, { version: number; pending: boolean }>();

/** Only Rodet files go to the server: it reads any open file as Rodet code, so
 * an open `Silk.toml` got "Syntax Error" on every line. The server sees saved
 * `Silk.toml` changes through its file watcher. */
function wantsServer(doc: Doc) {
  return doc.model.getLanguageId() === RODET;
}

function openInServer(doc: Doc) {
  if (!lsp.running || !wantsServer(doc)) return;
  const key = pathKey(doc.path);
  if (synced.has(key)) return;
  synced.set(key, { version: 1, pending: false });
  lsp.notify('textDocument/didOpen', {
    textDocument: { uri: lspUri(doc.model), languageId: doc.model.getLanguageId(), version: 1, text: doc.model.getValue() },
  });
  applyMarkers(doc.path);
}

/** Sends the latest text of a changed document. */
function flush(doc: Doc) {
  const state = synced.get(pathKey(doc.path));
  if (!state || !state.pending || !lsp.running) return;
  state.pending = false;
  state.version += 1;
  lsp.notify('textDocument/didChange', {
    textDocument: { uri: lspUri(doc.model), version: state.version },
    contentChanges: [{ text: doc.model.getValue() }],
  });
}

function flushModel(model: monaco.editor.ITextModel) {
  const doc = workspace.get(pathOfUri(model.uri));
  if (doc) flush(doc);
}

export function setupDocumentSync() {
  lsp.onReady.on(() => {
    synced.clear();
    for (const doc of workspace.docs.values()) openInServer(doc);
  });
  lsp.onState.on((state) => {
    if (state === 'stopped' || state === 'error' || state === 'missing') synced.clear();
  });
  workspace.onModelOpened.on((doc) => {
    applyMarkers(doc.path);
    openInServer(doc);
    const flushSoon = debounce(() => flush(doc), 120);
    doc.model.onDidChangeContent((e) => {
      followEdits(doc, e.changes);
      const state = synced.get(pathKey(doc.path));
      if (state) {
        state.pending = true;
        flushSoon();
      }
      scheduleSpeed(doc);
    });
  });
  workspace.onModelClosed.on((doc) => {
    const key = pathKey(doc.path);
    if (synced.delete(key) && lsp.running) {
      lsp.notify('textDocument/didClose', { textDocument: { uri: lspUri(doc.model) } });
    }
  });
  workspace.onSaved.on((doc) => {
    flush(doc);
    if (synced.has(pathKey(doc.path)) && lsp.running) {
      lsp.notify('textDocument/didSave', { textDocument: { uri: lspUri(doc.model) } });
    }
    scheduleSpeed(doc, 0);
  });
  workspace.onActive.on((path) => {
    const doc = path ? workspace.get(path) : undefined;
    if (doc) scheduleSpeed(doc, 0);
    else {
      lastSpeed = null;
      onSpeed.fire(null);
    }
  });
}

// ---------------------------------------------------------------- diagnostics

function applyMarkers(path: string) {
  const doc = workspace.get(path);
  if (!doc) return;
  const entry = diagnostics.get(pathKey(path));
  const markers = (entry?.diagnostics ?? []).map((d) => toMarker(d));
  monaco.editor.setModelMarkers(doc.model, 'rodet', markers);
}

function toMarker(d: LspDiagnostic): monaco.editor.IMarkerData {
  const S = monaco.MarkerSeverity;
  const severity = { 1: S.Error, 2: S.Warning, 3: S.Info, 4: S.Hint }[d.severity ?? 1] ?? S.Error;
  const range = toRange(d.range);
  const code = d.code === undefined ? undefined : d.codeDescription
    ? { value: String(d.code), target: monaco.Uri.parse(d.codeDescription.href) }
    : String(d.code);
  return {
    ...range,
    severity,
    message: translateDiagnostic(rodetText(d.message)),
    source: d.source,
    code,
    tags: d.tags?.map((t) => (t === 1 ? monaco.MarkerTag.Unnecessary : monaco.MarkerTag.Deprecated)),
    relatedInformation: d.relatedInformation?.map((r) => ({
      resource: monaco.Uri.parse(r.location.uri),
      message: translateDiagnostic(r.message),
      ...toRange(r.location.range),
    })),
  };
}

function setupDiagnostics() {
  lsp.onNotification('textDocument/publishDiagnostics', (params: { uri: string; diagnostics: LspDiagnostic[] }) => {
    const path = pathOfUri(params.uri);
    const key = pathKey(path);
    if (params.diagnostics.length) diagnostics.set(key, { path, uri: params.uri, diagnostics: params.diagnostics });
    else diagnostics.delete(key);
    applyMarkers(path);
    onDiagnostics.fire();
  });
}

/**
 * Keeps the errors of an edited file in step with the text until the next
 * check: errors below an edit move with their lines, and errors from
 * `cargo check` on the lines just edited disappear at once (they describe
 * code that is no longer there). Without this, a fixed error stayed red
 * until the file was saved and checked again.
 */
function followEdits(doc: Doc, changes: readonly monaco.editor.IModelContentChange[]) {
  const key = pathKey(doc.path);
  const entry = diagnostics.get(key);
  if (!entry || !changes.length) return;
  let list = entry.diagnostics;
  // Bottom to top, so each change's range is still in the right coordinates.
  const sorted = [...changes].sort((a, b) => b.range.startLineNumber - a.range.startLineNumber || b.range.startColumn - a.range.startColumn);
  for (const change of sorted) {
    const first = change.range.startLineNumber - 1;
    const last = change.range.endLineNumber - 1;
    const delta = change.text.split('\n').length - 1 - (last - first);
    list = list.flatMap((d) => {
      const { start, end } = d.range;
      if (end.line < first) return [d];
      if (start.line > last) {
        if (!delta) return [d];
        return [{ ...d, range: { start: { ...start, line: start.line + delta }, end: { ...end, line: end.line + delta } } }];
      }
      // On an edited line: the server's own errors come back by themselves
      // in a moment; the compiler's only after the next check.
      if (d.source && d.source !== 'rust-analyzer') return [];
      return [d];
    });
  }
  if (list.length === entry.diagnostics.length && list.every((d, i) => d === entry.diagnostics[i])) return;
  if (list.length) diagnostics.set(key, { ...entry, diagnostics: list });
  else diagnostics.delete(key);
  applyMarkers(doc.path);
  onDiagnostics.fire();
}

export function diagnosticCounts() {
  let errors = 0;
  let warnings = 0;
  for (const file of diagnostics.values()) {
    for (const d of file.diagnostics) {
      if ((d.severity ?? 1) === 1) errors++;
      else if (d.severity === 2) warnings++;
    }
  }
  return { errors, warnings };
}

// ---------------------------------------------------------------- edits

async function applyTextEdits(path: string, edits: LspTextEdit[]) {
  let doc = workspace.get(path);
  if (!doc) doc = (await workspace.load(path)) ?? undefined;
  if (!doc) return;
  const operations = edits.map((e) => ({ range: toRange(e.range), text: e.newText, forceMoveMarkers: true }));
  doc.model.pushStackElement();
  doc.model.pushEditOperations([], operations, () => null);
  doc.model.pushStackElement();
  if (!doc.inTab) await workspace.save(doc.path);
}

export async function applyWorkspaceEdit(edit: LspWorkspaceEdit): Promise<boolean> {
  try {
    if (edit.documentChanges) {
      for (const change of edit.documentChanges) {
        if (change.kind === 'create') {
          await backend.call('fs.writeFile', { path: pathOfUri(change.uri), text: '' });
        } else if (change.kind === 'rename') {
          const from = pathOfUri(change.oldUri);
          const to = pathOfUri(change.newUri);
          await backend.call('fs.rename', { from, to });
          workspace.pathsMoved(from, to);
        } else if (change.kind === 'delete') {
          const path = pathOfUri(change.uri);
          await backend.call('fs.delete', { path });
          workspace.pathsMoved(path, null);
        } else if (change.textDocument) {
          await applyTextEdits(pathOfUri(change.textDocument.uri), change.edits);
        }
      }
    } else if (edit.changes) {
      for (const [uri, edits] of Object.entries(edit.changes)) await applyTextEdits(pathOfUri(uri), edits);
    }
    return true;
  } catch (err) {
    toast(`${t('Couldn’t apply the change:')} ${errorText(err)}`, 'error');
    return false;
  }
}

async function runCodeAction(action: any) {
  let resolved = action;
  if (!action.edit && action.data !== undefined && lsp.capabilities.codeActionProvider?.resolveProvider) {
    resolved = await lsp.request('codeAction/resolve', action);
  }
  if (resolved.edit) await applyWorkspaceEdit(resolved.edit);
  const command = resolved.command;
  if (command && typeof command === 'object') runServerCommand(command.command, command.arguments ?? []);
}

/** Commands the language server attaches to lenses and actions. */
function runServerCommand(id: string, args: any[]) {
  switch (id) {
    case 'rust-analyzer.runSingle':
    case 'rust-analyzer.debugSingle': {
      const runnable = args[0];
      const a = runnable?.args ?? {};
      const command = [a.overrideCargo ?? 'rodet', ...(a.cargoArgs ?? [])];
      if (a.executableArgs?.length) command.push('--', ...a.executableArgs);
      runInTerminal(runnable?.label ?? 'rodet', command, a.cwd ?? a.workspaceRoot ?? workspace.root ?? '.');
      break;
    }
    case 'rust-analyzer.showReferences':
    case 'rust-analyzer.gotoLocation': {
      const locations: LspLocation[] = id === 'rust-analyzer.gotoLocation' ? [args[0]] : args[2] ?? [];
      showLocations(locations, id === 'rust-analyzer.showReferences' ? t('References') : t('Location'));
      break;
    }
    default:
      if (lsp.capabilities.executeCommandProvider?.commands?.includes(id)) {
        lsp.request('workspace/executeCommand', { command: id, arguments: args }).catch(() => {});
      }
  }
}

/** Opens one location, or lets the user pick among several. */
export async function showLocations(locations: LspLocation[], title: string) {
  if (locations.length === 0) {
    toast(t('Nothing found: {what}', { what: title }), 'info');
    return;
  }
  if (locations.length === 1) {
    const l = locations[0];
    await workspace.open(pathOfUri(l.uri), { line: l.range.start.line + 1, column: l.range.start.character + 1 });
    return;
  }
  const { pickLocation } = await import('../ui/palette');
  pickLocation(title, locations);
}

/** Loads the files of `locations` so Monaco can show them in "peek". */
async function ensureModels(locations: { uri: string }[]) {
  const paths = [...new Set(locations.map((l) => pathOfUri(l.uri)))].slice(0, 40);
  await Promise.all(paths.map((p) => workspace.load(p)));
}

function toLocations(result: any): monaco.languages.Location[] {
  if (!result) return [];
  const list = Array.isArray(result) ? result : [result];
  return list.map((l: any) =>
    l.targetUri
      ? { uri: monaco.Uri.parse(l.targetUri), range: toRange(l.targetSelectionRange ?? l.targetRange) }
      : { uri: monaco.Uri.parse(l.uri), range: toRange(l.range) },
  );
}

// ---------------------------------------------------------------- Arabic docs

/**
 * The Arabic description (std-ar.ts) of the item a hover shows. The hover
 * starts with the item's path in a code block (`alloc::vec::Vec`), then its
 * signature (`impl<T, A> Vec<T, A>` / `pub fn push(&mut self, value: T)`).
 */
function arabicDocFor(hover: string): string | null {
  const blocks = [...hover.matchAll(/```rust\n([\s\S]*?)```/g)].map((m) => m[1].trim());
  if (blocks.length < 2) return null;
  const path = blocks[0].split('\n')[0].trim();
  const signature = blocks[1];
  const lastSegment = path.split('::').pop() ?? '';
  const keys: string[] = [];
  const macro = signature.match(/\bmacro_rules!\s+(\w+)/);
  const fn = signature.match(/\bfn\s+(\w+)/);
  const type = signature.match(/\b(?:struct|enum|trait|union|type)\s+(\w+)/);
  if (macro) keys.push(`${macro[1]}!`);
  else if (fn) {
    const impl = signature.match(/^impl(?:<.*?>)?\s+(?:.+?\s+for\s+)?(.+?)\s*$/m);
    let container = impl ? impl[1].replace(/<.*$/, '').replace(/^&(mut\s+)?/, '').trim() : '';
    if (container.startsWith('[')) container = 'slice';
    if (container) keys.push(`${container}::${fn[1]}`);
    keys.push(`${lastSegment}::${fn[1]}`);
    if (lastSegment === 'slice' || container === 'Vec') keys.push(`slice::${fn[1]}`, `Vec::${fn[1]}`);
  } else if (type) keys.push(type[1]);
  else {
    // An enum variant like `Some(T)`: the path ends with its enum.
    const variant = signature.match(/^(?:pub\s+)?(\w+)/);
    if (variant) keys.push(`${lastSegment}::${variant[1]}`);
  }
  for (const key of keys) {
    const text = STD_AR[key];
    if (text) return `**${t('In Arabic')}:** ${isolateLatin(text)}\n\n---`;
  }
  return null;
}

// ---------------------------------------------------------------- providers

function tokenOf(token: monaco.CancellationToken) {
  return { onCancellationRequested: (cb: () => void) => token.onCancellationRequested(cb) };
}

async function ask<T>(model: monaco.editor.ITextModel, method: string, params: object, token?: monaco.CancellationToken): Promise<T | null> {
  if (!lsp.running) return null;
  flushModel(model);
  try {
    return await lsp.request<T>(method, { textDocument: { uri: lspUri(model) }, ...params }, token ? tokenOf(token) : undefined);
  } catch (err) {
    if (!isCancelled(err)) console.warn(method, err);
    return null;
  }
}

let returnMarks: monaco.editor.IEditorDecorationsCollection | null = null;
const semanticTokensChanged = new monaco.Emitter<void>();
const inlayHintsChanged = new monaco.Emitter<void>();
const inlayCache = new Map<string, { version: number; generation: number; hints: any[] }>();
/** Bumped when the server's answers may have changed without an edit. */
let inlayGeneration = 0;
const codeLensesChanged = new monaco.Emitter<monaco.languages.CodeLensProvider>();
const speedMarks = new Map<string, monaco.editor.IEditorDecorationsCollection>();

function setupProviders() {
  const L = monaco.languages;
  const selector = RODET;

  L.registerCompletionItemProvider(selector, {
    triggerCharacters: ['.', ':', "'", '(', '<', '_'],
    async provideCompletionItems(model, position, context, token) {
      const result: any = await ask(model, 'textDocument/completion', {
        position: toLspPosition(position),
        context: { triggerKind: context.triggerKind + 1, triggerCharacter: context.triggerCharacter },
      }, token);
      if (!result) return { suggestions: [] };
      const items: any[] = Array.isArray(result) ? result : result.items ?? [];      const word = model.getWordUntilPosition(position);
      const defaultRange = new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, position.column);
      return {
        incomplete: !Array.isArray(result) && !!result.isIncomplete,
        suggestions: items.map((item) => {
          const edit = item.textEdit;
          const range = edit
            ? edit.range
              ? toRange(edit.range)
              : { insert: toRange(edit.insert), replace: toRange(edit.replace) }
            : defaultRange;
          return {
            label: item.labelDetails
              ? { label: item.label, detail: item.labelDetails.detail, description: item.labelDetails.description }
              : item.label,
            kind: completionKinds[item.kind] ?? L.CompletionItemKind.Text,
            detail: item.detail && rodetText(item.detail),
            documentation: markdown(item.documentation),
            insertText: edit?.newText ?? item.insertText ?? item.label,
            insertTextRules: item.insertTextFormat === 2 ? L.CompletionItemInsertTextRule.InsertAsSnippet : undefined,
            range,
            sortText: item.sortText,
            filterText: item.filterText,
            preselect: item.preselect,
            tags: item.deprecated || item.tags?.includes(1) ? [L.CompletionItemTag.Deprecated] : undefined,
            additionalTextEdits: item.additionalTextEdits?.map((e: LspTextEdit) => ({ range: toRange(e.range), text: e.newText })),
            command: item.command?.command === 'rust-analyzer.triggerParameterHints'
              ? { id: 'editor.action.triggerParameterHints', title: t('') }
              : undefined,
            lspItem: item,
          } as monaco.languages.CompletionItem;
        }),
      };
    },
    async resolveCompletionItem(item: any, token) {
      if (!lsp.capabilities.completionProvider?.resolveProvider || !item.lspItem) return item;
      try {
        const r = await lsp.request<any>('completionItem/resolve', item.lspItem, tokenOf(token));
        item.documentation = markdown(r.documentation) ?? item.documentation;
        item.detail = r.detail ?? item.detail;
        if (r.additionalTextEdits) {
          item.additionalTextEdits = r.additionalTextEdits.map((e: LspTextEdit) => ({ range: toRange(e.range), text: e.newText }));
        }
      } catch {
        // Keep the unresolved item.
      }
      return item;
    },
  });

  L.registerHoverProvider(selector, {
    async provideHover(model, position, token) {
      const r: any = await ask(model, 'textDocument/hover', { position: toLspPosition(position) }, token);
      if (!r?.contents) return null;
      const contents = (Array.isArray(r.contents) ? r.contents : [r.contents])
        .map((c: any) => (typeof c === 'string' ? markdown(c) : c.language ? { value: '```' + c.language + '\n' + rodetText(c.value) + '\n```' } : markdown(c)))
        .filter(Boolean);
      const text = contents.map((c: monaco.IMarkdownString) => c.value).join('\n');
      // The standard library's own Arabic (rodet-analyzer shows it when
      // `rodet.language` is `ar`) comes first; the editor's list is the fallback.
      const arabicDoc = arabic && !/[\u0600-\u06FF]/.test(text) ? arabicDocFor(text) : null;
      if (arabicDoc) contents.unshift({ value: arabicDoc });
      return { contents, range: r.range ? toRange(r.range) : undefined };
    },
  });

  L.registerSignatureHelpProvider(selector, {
    signatureHelpTriggerCharacters: ['(', ','],
    signatureHelpRetriggerCharacters: [')'],
    async provideSignatureHelp(model, position, token) {
      const r: any = await ask(model, 'textDocument/signatureHelp', { position: toLspPosition(position) }, token);
      if (!r?.signatures?.length) return null;
      return {
        value: {
          activeSignature: r.activeSignature ?? 0,
          activeParameter: r.activeParameter ?? r.signatures[r.activeSignature ?? 0]?.activeParameter ?? 0,
          signatures: r.signatures.map((s: any) => ({
            label: s.label,
            documentation: markdown(s.documentation),
            parameters: (s.parameters ?? []).map((p: any) => ({ label: p.label, documentation: markdown(p.documentation) })),
            activeParameter: s.activeParameter,
          })),
        },
        dispose() {},
      };
    },
  });

  const locationProvider = (method: string) => async (model: monaco.editor.ITextModel, position: monaco.Position, token: monaco.CancellationToken) => {
    const r = await ask(model, method, { position: toLspPosition(position) }, token);
    const locations = toLocations(r);
    await ensureModels(locations.map((l) => ({ uri: l.uri.toString() })));
    return locations;
  };
  L.registerDefinitionProvider(selector, { provideDefinition: locationProvider('textDocument/definition') });
  L.registerTypeDefinitionProvider(selector, { provideTypeDefinition: locationProvider('textDocument/typeDefinition') });
  L.registerImplementationProvider(selector, { provideImplementation: locationProvider('textDocument/implementation') });
  L.registerReferenceProvider(selector, {
    async provideReferences(model, position, context, token) {
      const r = await ask(model, 'textDocument/references', { position: toLspPosition(position), context }, token);
      const locations = toLocations(r);
      await ensureModels(locations.map((l) => ({ uri: l.uri.toString() })));
      return locations;
    },
  });

  L.registerDocumentHighlightProvider(selector, {
    async provideDocumentHighlights(model, position, token) {
      const r: any[] | null = await ask(model, 'textDocument/documentHighlight', { position: toLspPosition(position) }, token);
      return (r ?? []).map((h) => ({ range: toRange(h.range), kind: h.kind ? h.kind - 1 : 0 }));
    },
  });

  L.registerDocumentSymbolProvider(selector, {
    displayName: 'Rodet',
    async provideDocumentSymbols(model, token) {
      const r: any[] | null = await ask(model, 'textDocument/documentSymbol', {}, token);
      const convert = (s: any): monaco.languages.DocumentSymbol => ({
        name: s.name,
        detail: s.detail ?? '',
        kind: (s.kind ?? 13) - 1,
        tags: [],
        range: toRange(s.range ?? s.location.range),
        selectionRange: toRange(s.selectionRange ?? s.location.range),
        children: (s.children ?? []).map(convert),
      });
      return (r ?? []).map(convert);
    },
  });

  L.registerFoldingRangeProvider(selector, {
    async provideFoldingRanges(model, _context, token) {
      const r: any[] | null = await ask(model, 'textDocument/foldingRange', {}, token);
      const kinds: Record<string, monaco.languages.FoldingRangeKind> = {
        comment: L.FoldingRangeKind.Comment,
        imports: L.FoldingRangeKind.Imports,
        region: L.FoldingRangeKind.Region,
      };
      return (r ?? []).map((f) => ({ start: f.startLine + 1, end: f.endLine + 1, kind: f.kind ? kinds[f.kind] : undefined }));
    },
  });

  L.registerRenameProvider(selector, {
    async resolveRenameLocation(model, position, token) {
      const r: any = await ask(model, 'textDocument/prepareRename', { position: toLspPosition(position) }, token);
      if (!r) return { range: new monaco.Range(1, 1, 1, 1), text: '', rejectReason: t('This can’t be renamed.') };
      const range = toRange(r.range ?? r);
      return { range, text: r.placeholder ?? model.getValueInRange(range) };
    },
    async provideRenameEdits(model, position, newName) {
      const r: any = await ask(model, 'textDocument/rename', { position: toLspPosition(position), newName });
      if (r) await applyWorkspaceEdit(r);
      return { edits: [] };
    },
  });

  monaco.editor.registerCommand('devr.applyCodeAction', (_accessor, action) => runCodeAction(action));
  monaco.editor.registerCommand('devr.serverCommand', (_accessor, id, args) => runServerCommand(id, args ?? []));

  L.registerCodeActionProvider(selector, {
    async provideCodeActions(model, range, context, token) {
      const entry = diagnostics.get(pathKey(pathOfUri(model.uri)));
      const lspDiagnostics = (entry?.diagnostics ?? []).filter((d) => monaco.Range.areIntersectingOrTouching(toRange(d.range), range));
      const r: any[] | null = await ask(model, 'textDocument/codeAction', {
        range: toLspRange(range),
        context: { diagnostics: lspDiagnostics, only: context.only ? [context.only] : undefined, triggerKind: context.trigger === 1 ? 1 : 2 },
      }, token);
      const actions = (r ?? []).map((a: any) => {
        if (typeof a.command === 'string') {
          return { title: translateAction(a.title), command: { id: 'devr.serverCommand', title: a.title, arguments: [a.command, a.arguments] } };
        }
        return {
          title: translateAction(a.title),
          kind: a.kind,
          isPreferred: a.isPreferred,
          diagnostics: (a.diagnostics ?? []).map(toMarker),
          disabled: a.disabled?.reason,
          command: { id: 'devr.applyCodeAction', title: a.title, arguments: [a] },
        } as monaco.languages.CodeAction;
      });
      return { actions, dispose() {} };
    },
  }, { providedCodeActionKinds: ['quickfix', 'refactor', 'refactor.extract', 'refactor.inline', 'refactor.rewrite', 'source'] });

  const lensProvider: monaco.languages.CodeLensProvider = {
    onDidChange: codeLensesChanged.event,
    async provideCodeLenses(model, token) {
      const r: any[] | null = await ask(model, 'textDocument/codeLens', {}, token);
      return {
        lenses: (r ?? []).map((lens, i) => ({ range: toRange(lens.range), id: String(i), command: lensCommand(lens), lspLens: lens } as any)),
        dispose() {},
      };
    },
    async resolveCodeLens(_model, lens: any, token) {
      if (lens.command || !lens.lspLens) return lens;
      try {
        const r = await lsp.request<any>('codeLens/resolve', lens.lspLens, tokenOf(token));
        lens.command = lensCommand(r);
      } catch {
        lens.command = { id: '', title: t('') };
      }
      return lens;
    },
  };
  L.registerCodeLensProvider(selector, lensProvider);
  lensProviderRef = lensProvider;

  L.registerInlayHintsProvider(selector, {
    onDidChangeInlayHints: inlayHintsChanged.event,
    async provideInlayHints(model, range, token) {
      // Ask for the whole file (the ↩ marks are drawn for all of it), once
      // per version of the text: scrolling reuses the answer.
      const key = model.uri.toString();
      const version = model.getVersionId();
      let r: any[] | null;
      const cached = inlayCache.get(key);
      if (cached && cached.version === version && cached.generation === inlayGeneration) {
        r = cached.hints;
      } else {
        const full = model.getFullModelRange();
        r = await ask(model, 'textDocument/inlayHint', { range: toLspRange(full) }, token);
        if (r) inlayCache.set(key, { version, generation: inlayGeneration, hints: r });
      }
      const hints: monaco.languages.InlayHint[] = [];
      const marks: monaco.editor.IModelDeltaDecoration[] = [];
      for (const hint of r ?? []) {
        const label = typeof hint.label === 'string' ? hint.label : hint.label.map((p: any) => p.value).join('');
        const position = { lineNumber: hint.position.line + 1, column: hint.position.character + 1 };
        if (label.trim() === '↩') {
          marks.push({
            range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column),
            options: {
              showIfCollapsed: true,
              after: {
                content: ' ↩',
                inlineClassName: 'rpp-return-mark',
                cursorStops: monaco.editor.InjectedTextCursorStops.None,
              },
              hoverMessage: { value: t('**Returned value** — the last expression of the function, without `;`.') },
            },
          });
          continue;
        }
        if (!range.containsPosition(position)) continue;
        hints.push({
          label: typeof hint.label === 'string'
            ? rodetText(hint.label)
            : hint.label.map((p: any) => ({
                label: rodetText(p.value),
                tooltip: markdown(p.tooltip),
                location: p.location ? { uri: monaco.Uri.parse(p.location.uri), range: toRange(p.location.range) } : undefined,
              })),
          position,
          kind: hint.kind,
          paddingLeft: hint.paddingLeft,
          paddingRight: hint.paddingRight,
          tooltip: markdown(hint.tooltip),
        });
      }
      const editor = workspace.editor;
      if (r && editor.getModel() === model) {
        returnMarks ??= editor.createDecorationsCollection();
        returnMarks.set(settings.returnMarks ? marks : []);
      }
      return { hints, dispose() {} };
    },
  });

  const legend = () => lsp.capabilities.semanticTokensProvider?.legend ?? { tokenTypes: [], tokenModifiers: [] };
  L.registerDocumentSemanticTokensProvider(selector, {
    onDidChange: semanticTokensChanged.event,
    getLegend: legend,
    async provideDocumentSemanticTokens(model, _lastResultId, token) {
      if (!lsp.capabilities.semanticTokensProvider) return null;
      const r: any = await ask(model, 'textDocument/semanticTokens/full', {}, token);
      if (!r?.data) return null;
      return { data: new Uint32Array(r.data), resultId: r.resultId };
    },
    releaseDocumentSemanticTokens() {},
  });

  // Return marks belong to the model shown: clear them when switching files.
  workspace.onActive.on(() => returnMarks?.clear());
}

function lensCommand(lens: any): monaco.languages.Command | undefined {
  const c = lens?.command;
  if (!c) return undefined;
  return { id: 'devr.serverCommand', title: translateLens(c.title), arguments: [c.command, c.arguments] };
}

// ---------------------------------------------------------------- server status

function setupStatus() {
  lsp.onRequest('workspace/configuration', (params: { items: { section?: string }[] }) =>
    params.items.map((item) => (item.section === 'rust-analyzer' || !item.section ? analyzerConfig() : null)),
  );
  lsp.onRequest('window/workDoneProgress/create', () => null);
  lsp.onRequest('client/registerCapability', () => null);
  lsp.onRequest('client/unregisterCapability', () => null);
  lsp.onRequest('workspace/applyEdit', async (params: { edit: LspWorkspaceEdit }) => ({ applied: await applyWorkspaceEdit(params.edit) }));
  lsp.onRequest('window/showMessageRequest', (params: { message: string; type: number }) => {
    toast(params.message, params.type === 1 ? 'error' : params.type === 2 ? 'warning' : 'info');
    return null;
  });
  lsp.onRequest('workspace/semanticTokens/refresh', () => {
    semanticTokensChanged.fire();
    return null;
  });
  lsp.onRequest('workspace/inlayHint/refresh', () => {
    inlayGeneration++;
    inlayHintsChanged.fire();
    return null;
  });
  lsp.onRequest('workspace/codeLens/refresh', () => {
    if (lensProviderRef) codeLensesChanged.fire(lensProviderRef);
    return null;
  });
  lsp.onRequest('workspace/diagnostic/refresh', () => null);

  lsp.onNotification('window/showMessage', (params: { message: string; type: number }) => {
    if (params.type <= 2) toast(params.message, params.type === 1 ? 'error' : 'warning');
  });
  lsp.onNotification('window/logMessage', (params: { message: string }) => lsp.onLog.fire(params.message));
  lsp.onNotification('experimental/serverStatus', (params: { health: string; quiescent: boolean; message?: string }) => {
    const wasReady = lsp.state === 'ready';
    lsp.markReady(params.quiescent);
    if (params.health === 'error' && params.message) lsp.onLog.fire(params.message);
    if (params.quiescent) {
      onProgress.fire(null);
      if (!wasReady) refreshEditorFeatures();
    }
  });
  lsp.onNotification('$/progress', (params: { token: string | number; value: any }) => {
    const v = params.value;
    if (v.kind === 'begin') progress.set(params.token, { title: v.title, message: v.message, percentage: v.percentage });
    else if (v.kind === 'report') {
      const p = progress.get(params.token);
      if (p) Object.assign(p, { message: v.message ?? p.message, percentage: v.percentage ?? p.percentage });
    } else if (v.kind === 'end') progress.delete(params.token);
    const last = [...progress.values()].pop() ?? null;
    onProgress.fire(last);
  });
}

let lensProviderRef: monaco.languages.CodeLensProvider | null = null;

/** Asks Monaco to fetch semantic tokens, hints and lenses again. */
export function refreshEditorFeatures() {
  semanticTokensChanged.fire();
  inlayGeneration++;
  inlayHintsChanged.fire();
  if (lensProviderRef) codeLensesChanged.fire(lensProviderRef);
  const model = workspace.editor?.getModel();
  const doc = model ? workspace.get(pathOfUri(model.uri)) : undefined;
  if (doc) scheduleSpeed(doc, 0);
}

// ---------------------------------------------------------------- speed score

const speedTimers = new Map<string, number>();

function scheduleSpeed(doc: Doc, delay = 900) {
  if (doc.model.getLanguageId() !== RODET) {
    if (workspace.active && pathKey(workspace.active) === pathKey(doc.path)) {
      lastSpeed = null;
      onSpeed.fire(null);
    }
    return;
  }
  const key = pathKey(doc.path);
  clearTimeout(speedTimers.get(key));
  speedTimers.set(key, window.setTimeout(() => fetchSpeed(doc), delay));
}

async function fetchSpeed(doc: Doc) {
  if (!settings.speedScore || !lsp.running) return;
  if (!workspace.active || pathKey(workspace.active) !== pathKey(doc.path)) return;
  const r = await ask<{ score: number; findings: SpeedFinding[] }>(doc.model, 'rodet/speedScore', {});
  if (!r) return;
  lastSpeed = { path: doc.path, score: r.score, findings: r.findings };
  onSpeed.fire(lastSpeed);
  showSpeedMarks(doc, r.findings);
}

export function showSpeedMarks(doc: Doc, findings: SpeedFinding[]) {
  const key = doc.model.uri.toString();
  let collection = speedMarks.get(key);
  if (!collection) {
    collection = workspace.editor.createDecorationsCollection();
    speedMarks.set(key, collection);
  }
  collection.set(
    findings.map((f) => ({
      range: toRange(f.range),
      options: {
        inlineClassName: 'rpp-speed-mark',
        hoverMessage: { value: `**⚡ −${f.penalty} ${t('speed')}** · ${translateTip(f.message)}\n\n💡 ${translateTip(f.fix)}` },
        overviewRuler: { color: '#e8590c88', position: monaco.editor.OverviewRulerLane.Center },
      },
    })),
  );
}

export function clearSpeedMarks() {
  for (const c of speedMarks.values()) c.clear();
  speedMarks.clear();
}

// ----------------------------------------------------------------

export function setupLanguageFeatures() {
  setupDiagnostics();
  setupStatus();
  setupProviders();
  setupDocumentSync();
  workspace.onActive.on(() => clearSpeedMarks());
}
