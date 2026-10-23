// Search in all files of the project (Ctrl+Shift+F).
import { backend, errorText } from '../backend';
import type { SideView } from '../ui/layout';
import { basename, debounce, fileIcon, h, icon, relativePath } from '../util';
import { workspace } from '../workspace';

interface Match {
  line: number;
  start: number;
  end: number;
  preview: string;
  previewStart: number;
}
interface FileMatches {
  path: string;
  matches: Match[];
}

const options = { regex: false, caseSensitive: false, wholeWord: false };
const input = h('input.input', { placeholder: 'Search', spellcheck: false });
const include = h('input.input', { placeholder: 'Files to include (e.g. .rs)', spellcheck: false });
const summary = h('div.muted', { style: { fontSize: '12px' } });
const results = h('div.tree');
const collapsed = new Set<string>();

function toggleButton(name: string, title: string, key: keyof typeof options) {
  const button = h('button.icon-button', {
    title,
    onclick: () => {
      options[key] = !options[key];
      button.classList.toggle('on', options[key]);
      run();
    },
  }, icon(name));
  return button;
}

const element = h('div', {},
  h('div.pad', {},
    h('div.search-box', {}, input, h('div.toggles', {},
      toggleButton('case-sensitive', 'Match Case', 'caseSensitive'),
      toggleButton('whole-word', 'Match Whole Word', 'wholeWord'),
      toggleButton('regex', 'Use Regular Expression', 'regex'))),
    include,
    summary),
  h('div', { style: { overflow: 'auto', flex: '1' } }, results));

let generation = 0;
const run = debounce(async () => {
  const query = input.value;
  const root = workspace.root;
  const gen = ++generation;
  if (!query || !root) {
    results.replaceChildren();
    summary.textContent = root ? '' : 'Open a folder to search in it.';
    return;
  }
  summary.textContent = 'Searching…';
  try {
    const result = await backend.call<{ files: FileMatches[]; total: number; truncated: boolean }>('search.text', {
      root, query, include: include.value.trim(), ...options,
    });
    if (gen !== generation) return;
    summary.textContent = result.total
      ? `${result.total}${result.truncated ? '+' : ''} results in ${result.files.length} files`
      : 'No results found.';
    render(result.files);
  } catch (err) {
    summary.textContent = errorText(err);
    results.replaceChildren();
  }
}, 250);

function render(files: FileMatches[]) {
  results.replaceChildren();
  for (const file of files) {
    const open = !collapsed.has(file.path);
    const fi = fileIcon(file.path);
    const fileIconEl = icon(fi.name);
    fileIconEl.style.color = fi.color;
    results.appendChild(h('div.row', {
      style: { paddingLeft: '8px' },
      onclick: () => {
        if (open) collapsed.add(file.path);
        else collapsed.delete(file.path);
        render(files);
      },
    }, h('span.twistie', {}, icon(open ? 'chevron-down' : 'chevron-right')), fileIconEl,
      h('span.label', {}, basename(file.path)),
      h('span.desc', {}, relativePath(workspace.root, file.path)),
      h('span.end', {}, h('span.count-badge', {}, String(file.matches.length)))));
    if (!open) continue;
    for (const m of file.matches) {
      const before = m.preview.slice(0, m.previewStart);
      const hit = m.preview.slice(m.previewStart, m.previewStart + (m.end - m.start));
      const after = m.preview.slice(m.previewStart + (m.end - m.start));
      results.appendChild(h('div.row.result-line', {
        style: { paddingLeft: '42px' },
        title: `Line ${m.line}`,
        onclick: () => workspace.open(file.path, { line: m.line, column: m.start, endLine: m.line, endColumn: m.end }),
      }, h('span.preview', {}, before, h('mark', {}, hit), after)));
    }
  }
}

input.addEventListener('input', run);
include.addEventListener('input', run);
input.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') run();
});

export const search: SideView = {
  id: 'search',
  title: 'Search',
  icon: 'search',
  element,
  onShow: () => setTimeout(() => {
    input.focus();
    input.select();
  }),
  actions: [h('button.icon-button', { title: 'Clear', onclick: () => { input.value = ''; run(); } }, icon('clear-all'))],
};

/** Starts a search for `text` (e.g. the selected word). */
export function searchFor(text: string) {
  if (text) input.value = text;
  run();
}

export function setupSearch() {
  workspace.onRoot.on(() => {
    results.replaceChildren();
    summary.textContent = '';
  });
}
