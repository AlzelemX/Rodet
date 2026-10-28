// The Problems panel (errors and warnings of every file) and the Output
// panel (language server messages).
import { arabic, t } from '../i18n';
import { translateDiagnostic } from '../i18n/messages';
import { diagnostics, diagnosticCounts, lsp, onDiagnostics, rodetText } from '../lsp/features';
import type { PanelView } from '../ui/layout';
import { basename, debounce, fileIcon, h, icon, relativePath, richText } from '../util';
import { workspace } from '../workspace';

const list = h('div.problems');
const filterInput = h('input.input', { placeholder: t('Filter (e.g. text, **/*.rd)'), style: { width: '220px', height: '24px' } });
const count = h('span.count', {}, '0');
const collapsed = new Set<string>();
/** Problems whose full compiler explanation is shown. */
const expanded = new Set<string>();

const render = debounce(() => {
  const { errors, warnings } = diagnosticCounts();
  count.textContent = String(errors + warnings);
  count.style.display = errors + warnings ? '' : 'none';
  const filter = filterInput.value.trim().toLowerCase();
  list.replaceChildren();
  const files = [...diagnostics.values()].sort((a, b) => a.path.localeCompare(b.path));
  for (const file of files) {
    const items = file.diagnostics
      .filter((d) => !filter || d.message.toLowerCase().includes(filter) || translateDiagnostic(d.message).toLowerCase().includes(filter) || file.path.toLowerCase().includes(filter))
      .sort((a, b) => (a.severity ?? 1) - (b.severity ?? 1) || a.range.start.line - b.range.start.line);
    if (!items.length) continue;
    const open = !collapsed.has(file.path);
    const fi = fileIcon(file.path);
    const fileIconEl = icon(fi.name);
    fileIconEl.style.color = fi.color;
    list.appendChild(h('div.row', {
      onclick: () => {
        if (open) collapsed.add(file.path);
        else collapsed.delete(file.path);
        render();
      },
    }, h('span.twistie', {}, icon(open ? 'chevron-down' : 'chevron-right')), fileIconEl,
      h('span.label', {}, basename(file.path)),
      h('span.desc', {}, relativePath(workspace.root, file.path)),
      h('span.count-badge', {}, String(items.length))));
    if (!open) continue;
    for (const d of items) {
      const kind = (d.severity ?? 1) === 1 ? 'error' : d.severity === 2 ? 'warning' : 'info';
      const code = d.code !== undefined ? ` ${sourceName(d.source)}(${d.code})` : d.source ? ` ${sourceName(d.source)}` : '';
      const rendered: string | undefined = (d.data as any)?.rendered;
      const id = `${file.path}:${d.range.start.line}:${d.range.start.character}:${d.message}`;
      const open = expanded.has(id);
      list.appendChild(h('div.row', {
        style: { paddingLeft: '22px' },
        onclick: () => workspace.open(file.path, {
          line: d.range.start.line + 1,
          column: d.range.start.character + 1,
          endLine: d.range.end.line + 1,
          endColumn: d.range.end.character + 1,
        }),
      },
      h('span.twistie', {
        title: rendered ? t('Show the compiler’s full explanation') : '',
        onclick: (e: MouseEvent) => {
          if (!rendered) return;
          e.stopPropagation();
          if (open) expanded.delete(id);
          else expanded.add(id);
          render();
        },
      }, rendered ? icon(open ? 'chevron-down' : 'chevron-right') : null),
      icon(kind), h('span.msg', { title: arabic ? rodetText(d.message.split('\n')[0]) : '' }, ...richText(translateDiagnostic(rodetText(d.message)).split('\n')[0]),
        h('span.where', {}, `${code} [${t('Ln {line}, Col {col}', { line: d.range.start.line + 1, col: d.range.start.character + 1 })}]`))));
      if (open && rendered) list.appendChild(h('pre.explain', {}, rodetText(rendered.trimEnd())));
    }
  }
  if (!list.children.length) {
    list.appendChild(h('div.empty-state', {}, filter ? t('No problems match the filter.') : t('No problems have been found in the project. 🎉')));
  }
}, 120);

export const problemsPanel: PanelView = {
  id: 'problems',
  title: t('Problems'),
  element: h('div', {}, list),
  count,
  actions: [filterInput],
};

// ---- output ----

const output = h('div.output');
let lines = 0;

export function log(line: string) {
  if (lines > 3000) {
    output.firstChild?.remove();
  } else lines++;
  output.appendChild(document.createTextNode(line + '\n'));
  if (output.scrollHeight - output.scrollTop - output.clientHeight < 80) output.scrollTop = output.scrollHeight;
}

export const outputPanel: PanelView = {
  id: 'output',
  title: t('Output'),
  element: h('div', {}, output),
  actions: [h('button.icon-button', { title: t('Clear Output'), onclick: () => { output.replaceChildren(); lines = 0; } }, icon('clear-all'))],
};

export function setupProblems() {
  onDiagnostics.on(render);
  filterInput.addEventListener('input', render);
  lsp.onLog.on((line) => log(line));
  render();
}

/** Rodet's names for the tools that report problems. */
function sourceName(source: string | undefined) {
  if (!source || source === 'rustc') return 'rodetc';
  if (source === 'rust-analyzer') return 'rodet-analyzer';
  return source;
}
