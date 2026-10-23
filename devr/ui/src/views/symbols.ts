// Find a struct, enum, trait, function, impl… by name, with filters.
import { t } from '../i18n';
import type { LspLocation } from '../lsp/client';
import { lsp } from '../lsp/features';
import { openLocation, searchSymbols, symbolIcon } from '../ui/palette';
import type { SideView } from '../ui/layout';
import { basename, debounce, h, icon, relativePath } from '../util';
import { pathOfUri, workspace } from '../workspace';

/** Filters, with the LSP symbol kinds they keep. */
const kinds: { id: string; label: string; icon: string; kinds: number[] }[] = [
  { id: 'all', label: t('All'), icon: 'symbol-misc', kinds: [] },
  { id: 'struct', label: t('struct'), icon: 'symbol-structure', kinds: [23] },
  { id: 'enum', label: t('enum'), icon: 'symbol-enum', kinds: [10, 22] },
  { id: 'trait', label: t('trait'), icon: 'symbol-interface', kinds: [11] },
  { id: 'impl', label: t('impl'), icon: 'symbol-object', kinds: [] },
  { id: 'fn', label: t('fn'), icon: 'symbol-function', kinds: [12, 6] },
  { id: 'const', label: t('const'), icon: 'symbol-constant', kinds: [14] },
  { id: 'type', label: t('type'), icon: 'symbol-type-parameter', kinds: [26] },
  { id: 'mod', label: t('mod'), icon: 'symbol-namespace', kinds: [2] },
];

let kind = 'all';
const input = h('input.input', { placeholder: t('Name (e.g. Parser, new, Display)'), spellcheck: false });
const chips = h('div.chips');
const status = h('div.muted', { style: { fontSize: '12px' } });
const list = h('div.tree');

function renderChips() {
  chips.replaceChildren(...kinds.map((k) =>
    h(k.id === kind ? 'button.chip.on' : 'button.chip', { onclick: () => { kind = k.id; renderChips(); run(); } }, icon(k.icon), k.label)));
}

const element = h('div', {},
  h('div.pad', {}, input, chips, status),
  h('div', { style: { overflow: 'auto', flex: '1' } }, list));

let generation = 0;
const run = debounce(async () => {
  const query = input.value.trim();
  const gen = ++generation;
  list.replaceChildren();
  if (!lsp.running) {
    status.textContent = t('The language server is not running yet.');
    return;
  }
  if (!query) {
    status.textContent = kind === 'impl'
      ? t('Type a type or trait name to see every `impl` of it.')
      : t('Type a name. Use the filters to keep only structs, traits, functions…');
    return;
  }
  status.textContent = t('Searching…');
  if (kind === 'impl') return findImpls(query, gen);
  const filter = kinds.find((k) => k.id === kind)!;
  const symbols = (await searchSymbols(query)).filter((s) => !filter.kinds.length || filter.kinds.includes(s.kind));
  if (gen !== generation) return;
  status.textContent = symbols.length ? t('{n} found', { n: symbols.length }) : t('Nothing found.');
  for (const s of symbols) list.appendChild(row(s.name, s.kind, s.container, s.location));
}, 200);

function row(name: string, kindNumber: number, container: string | undefined, location: LspLocation, label?: string) {
  const path = pathOfUri(location.uri);
  return h('div.row', {
    style: { paddingLeft: '14px', height: 'auto', minHeight: '22px', alignItems: 'flex-start', padding: '3px 8px 3px 14px' },
    title: relativePath(workspace.root, path),
    onclick: () => openLocation(location),
  },
  icon(symbolIcon(kindNumber)),
  h('div', { style: { display: 'flex', flexDirection: 'column', minWidth: '0' } },
    h('span.label', {}, label ?? name),
    h('span.desc', {}, [container, `${basename(path)}:${location.range.start.line + 1}`].filter(Boolean).join(' · '))));
}

/** Every `impl` block of the type or trait named `query`. */
async function findImpls(query: string, gen: number) {
  const targets = (await searchSymbols(query)).filter((s) => [23, 10, 11, 26].includes(s.kind) && s.name.toLowerCase() === query.toLowerCase());
  const candidates = targets.length ? targets : (await searchSymbols(query)).filter((s) => [23, 10, 11, 26].includes(s.kind)).slice(0, 3);
  let found = 0;
  for (const target of candidates) {
    let result: any = null;
    try {
      result = await lsp.request('textDocument/implementation', {
        textDocument: { uri: target.location.uri },
        position: target.location.range.start,
      });
    } catch {
      continue;
    }
    if (gen !== generation) return;
    const locations: LspLocation[] = (Array.isArray(result) ? result : result ? [result] : [])
      .map((l: any) => (l.targetUri ? { uri: l.targetUri, range: l.targetSelectionRange ?? l.targetRange } : l));
    if (!locations.length) continue;
    list.appendChild(h('div.section-title', { style: { paddingLeft: '12px' } }, icon(symbolIcon(target.kind)), `${target.name}`));
    for (const location of locations) {
      const line = await lineOf(location);
      list.appendChild(row(target.name, 19, undefined, location, line || `impl ${target.name}`));
      found++;
    }
  }
  if (gen !== generation) return;
  status.textContent = found ? t('{n} impl blocks', { n: found }) : t('No impl blocks found.');
}

/** The source line of a location, e.g. `impl Display for Parser {`. */
async function lineOf(location: LspLocation) {
  const doc = await workspace.load(pathOfUri(location.uri));
  if (!doc) return '';
  const text = doc.model.getLineContent(location.range.start.line + 1).trim();
  return text.replace(/\{\s*$/, '').trim();
}

input.addEventListener('input', run);
input.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    (list.querySelector('.row') as HTMLElement | null)?.click();
  }
});

export const symbols: SideView = {
  id: 'symbols',
  title: t('Find Symbols'),
  icon: 'symbol-class',
  element,
  onShow: () => setTimeout(() => input.focus()),
};

export function setupSymbols() {
  renderChips();
  lsp.onState.on(() => run());
}
