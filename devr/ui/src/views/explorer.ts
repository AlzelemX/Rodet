// The file tree of the project.
import { t } from '../i18n';
import { backend, errorText } from '../backend';
import { diagnostics, onDiagnostics } from '../lsp/features';
import { invalidateFileCache } from '../ui/palette';
import { confirm, toast } from '../ui/notify';
import { contextMenu, type SideView } from '../ui/layout';
import { basename, debounce, dirname, fileIcon, h, icon, joinPath, relativePath } from '../util';
import { pathKey, workspace } from '../workspace';

interface Entry {
  name: string;
  path: string;
  isDir: boolean;
}

const tree = h('div.tree', { tabindex: 0 });
const element = h('div', {}, tree);
const expanded = new Set<string>();
const children = new Map<string, Entry[]>();
let selected: string | null = null;
let editing: { parent: string; kind: 'file' | 'folder' | 'rename'; path?: string } | null = null;

async function loadDir(path: string) {
  try {
    children.set(pathKey(path), await backend.call<Entry[]>('fs.readDir', { path }));
  } catch (err) {
    children.set(pathKey(path), []);
    toast(errorText(err), 'error');
  }
}

function problemState(path: string): 'error' | 'warning' | null {
  const key = pathKey(path);
  let state: 'error' | 'warning' | null = null;
  for (const [file, entry] of diagnostics) {
    if (file !== key && !file.startsWith(key + '/')) continue;
    for (const d of entry.diagnostics) {
      if ((d.severity ?? 1) === 1) return 'error';
      if (d.severity === 2) state = 'warning';
    }
  }
  return state;
}

function render() {
  const root = workspace.root;
  tree.replaceChildren();
  if (!root) {
    tree.appendChild(h('div.empty-state', {}, t('No folder is open.'),
      h('br'), h('button.btn.primary', { onclick: () => import('../main').then((m) => m.openFolderDialog()) }, icon('folder-opened'), t('Open Folder'))));
    return;
  }
  const rootRow = h('div.section-title', { onclick: () => toggle(root) },
    icon(expanded.has(pathKey(root)) ? 'chevron-down' : 'chevron-right'), basename(root),
    h('span.actions', {},
      action('new-file', 'New File', () => startCreate(root, 'file')),
      action('new-folder', 'New Folder', () => startCreate(root, 'folder')),
      action('refresh', 'Refresh', () => refresh()),
      action('collapse-all', 'Collapse Folders', () => { expanded.clear(); expanded.add(pathKey(root)); render(); })));
  tree.appendChild(rootRow);
  if (expanded.has(pathKey(root))) renderDir(root, 0);
}

function action(name: string, title: string, run: () => void) {
  return h('button.icon-button', { title: t(title), onclick: (e: MouseEvent) => { e.stopPropagation(); run(); } }, icon(name));
}

function renderDir(dir: string, depth: number) {
  const entries = children.get(pathKey(dir)) ?? [];
  if (editing && editing.kind !== 'rename' && pathKey(editing.parent) === pathKey(dir)) {
    tree.appendChild(inputRow(depth, editing.kind === 'folder' ? 'folder' : 'file', '', (name) => create(dir, name, editing!.kind === 'folder')));
  }
  for (const entry of entries) {
    if (editing?.kind === 'rename' && editing.path && pathKey(editing.path) === pathKey(entry.path)) {
      tree.appendChild(inputRow(depth, entry.isDir ? 'folder' : fileIcon(entry.path).name, entry.name, (name) => rename(entry.path, name)));
      continue;
    }
    const key = pathKey(entry.path);
    const open = entry.isDir && expanded.has(key);
    const fi = fileIcon(entry.path);
    const problems = problemState(entry.path);
    const dirty = !entry.isDir && workspace.isDirty(entry.path);
    const row = h(`div.row${selected === key ? '.selected' : ''}${problems ? '.' + problems : ''}${dirty ? '.dirty' : ''}` as 'div', {
      style: { paddingLeft: `${8 + depth * 14}px` },
      title: relativePath(workspace.root, entry.path),
      onclick: () => {
        selected = key;
        if (entry.isDir) toggle(entry.path);
        else {
          render();
          workspace.open(entry.path);
        }
      },
      oncontextmenu: (e: MouseEvent) => {
        selected = key;
        render();
        showMenu(e, entry);
      },
    },
    h('span.twistie', {}, entry.isDir ? icon(open ? 'chevron-down' : 'chevron-right') : null),
    entry.isDir ? icon(open ? 'folder-opened' : 'folder') : (() => {
      const i = icon(fi.name);
      i.style.color = fi.color;
      return i;
    })(),
    h('span.label', {}, entry.name),
    problems && !entry.isDir ? h('span.end', {}, icon(problems === 'error' ? 'error' : 'warning')) : null);
    tree.appendChild(row);
    if (open) renderDir(entry.path, depth + 1);
  }
}

function inputRow(depth: number, iconName: string, value: string, done: (name: string) => void) {
  const input = h('input.input', { value, spellcheck: false });
  let finished = false;
  const finish = (ok: boolean) => {
    if (finished) return;
    finished = true;
    editing = null;
    if (ok && input.value.trim() && input.value.trim() !== value) done(input.value.trim());
    else render();
  };
  input.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') finish(true);
    if (e.key === 'Escape') finish(false);
    e.stopPropagation();
  });
  input.addEventListener('blur', () => finish(true));
  setTimeout(() => {
    input.focus();
    const dot = value.lastIndexOf('.');
    input.setSelectionRange(0, dot > 0 ? dot : value.length);
  });
  return h('div.row.rename', { style: { paddingLeft: `${8 + depth * 14}px` } }, h('span.twistie'), icon(iconName), input);
}

async function toggle(path: string) {
  const key = pathKey(path);
  if (expanded.has(key)) expanded.delete(key);
  else {
    expanded.add(key);
    if (!children.has(key)) await loadDir(path);
  }
  render();
}

async function startCreate(parent: string, kind: 'file' | 'folder') {
  expanded.add(pathKey(parent));
  if (!children.has(pathKey(parent))) await loadDir(parent);
  editing = { parent, kind };
  render();
}

async function create(dir: string, name: string, folder: boolean) {
  const path = joinPath(dir, name);
  try {
    await backend.call(folder ? 'fs.createDir' : 'fs.createFile', { path });
    await loadDir(dir);
    invalidateFileCache();
    selected = pathKey(path);
    render();
    if (!folder) workspace.open(path);
  } catch (err) {
    toast(errorText(err), 'error');
    render();
  }
}

async function rename(path: string, name: string) {
  const to = joinPath(dirname(path), name);
  try {
    await backend.call('fs.rename', { from: path, to });
    workspace.pathsMoved(path, to);
    await loadDir(dirname(path));
    invalidateFileCache();
    selected = pathKey(to);
  } catch (err) {
    toast(errorText(err), 'error');
  }
  render();
}

async function remove(entry: Entry) {
  const ok = await confirm(t(entry.isDir ? 'Delete the folder “{name}”?' : 'Delete “{name}”?', { name: entry.name }),
    entry.isDir ? t('The folder and everything in it will be deleted. This can’t be undone.') : t('This can’t be undone.'), t('Delete'), true);
  if (!ok) return;
  try {
    await backend.call('fs.delete', { path: entry.path });
    workspace.pathsMoved(entry.path, null);
    await loadDir(dirname(entry.path));
    invalidateFileCache();
  } catch (err) {
    toast(errorText(err), 'error');
  }
  render();
}

function showMenu(e: MouseEvent, entry: Entry) {
  const dir = entry.isDir ? entry.path : dirname(entry.path);
  contextMenu(e, [
    ...(entry.isDir ? [] : [{ label: t('Open'), icon: 'go-to-file', run: () => workspace.open(entry.path) }, 'sep' as const]),
    { label: t('New File…'), icon: 'new-file', run: () => startCreate(dir, 'file') },
    { label: t('New Folder…'), icon: 'new-folder', run: () => startCreate(dir, 'folder') },
    'sep',
    { label: t('Rename…'), icon: 'edit', key: 'F2', run: () => { editing = { parent: dirname(entry.path), kind: 'rename', path: entry.path }; render(); } },
    { label: t('Delete'), icon: 'trash', key: 'Delete', run: () => remove(entry) },
    'sep',
    { label: t('Copy Path'), icon: 'copy', run: () => navigator.clipboard.writeText(entry.path) },
    { label: t('Copy Relative Path'), icon: 'copy', run: () => navigator.clipboard.writeText(relativePath(workspace.root, entry.path)) },
  ]);
}

async function refresh() {
  const root = workspace.root!;
  const all = [...children.values()].flat();
  const dirs = [root, ...all.filter((e) => e.isDir && expanded.has(pathKey(e.path))).map((e) => e.path)];
  children.clear();
  for (const dir of dirs) await loadDir(dir);
  invalidateFileCache();
  render();
}

/** Reloads the folders whose content changed on disk. */
const onDiskChanged = debounce(async (paths: string[]) => {
  const dirs = new Set(paths.map((p) => dirname(p)));
  let changed = false;
  for (const dir of dirs) {
    if (children.has(pathKey(dir))) {
      await loadDir(dir);
      changed = true;
    }
  }
  if (changed) {
    invalidateFileCache();
    render();
  }
}, 200);

/** Shows the active file in the tree. */
export async function reveal(path: string) {
  const root = workspace.root;
  if (!root) return;
  const rel = relativePath(root, path);
  if (rel === path) return;
  let dir = root;
  expanded.add(pathKey(root));
  for (const part of rel.split('/').slice(0, -1)) {
    if (!children.has(pathKey(dir))) await loadDir(dir);
    dir = joinPath(dir, part);
    expanded.add(pathKey(dir));
  }
  if (!children.has(pathKey(dir))) await loadDir(dir);
  selected = pathKey(path);
  render();
  tree.querySelector('.row.selected')?.scrollIntoView({ block: 'nearest' });
}

tree.addEventListener('keydown', (e) => {
  if (!selected) return;
  const entry = [...children.values()].flat().find((x) => pathKey(x.path) === selected);
  if (!entry) return;
  if (e.key === 'F2') {
    editing = { parent: dirname(entry.path), kind: 'rename', path: entry.path };
    render();
  } else if (e.key === 'Delete') {
    remove(entry);
  }
});

export const explorer: SideView = {
  id: 'explorer',
  title: t('Explorer'),
  icon: 'files',
  element,
  actions: [
    action('new-file', 'New File', () => workspace.root && startCreate(workspace.root, 'file')),
    action('new-folder', 'New Folder', () => workspace.root && startCreate(workspace.root, 'folder')),
    action('refresh', 'Refresh', () => workspace.root && refresh()),
  ],
};

export function setupExplorer() {
  workspace.onRoot.on(async (root) => {
    children.clear();
    expanded.clear();
    selected = null;
    if (root) {
      expanded.add(pathKey(root));
      await loadDir(root);
    }
    render();
  });
  workspace.onActive.on((path) => {
    if (path) reveal(path);
  });
  workspace.onDirty.on(debounce(() => render(), 100));
  onDiagnostics.on(debounce(() => render(), 300));
  backend.on('fs.changed', ({ paths }: { paths: string[] }) => onDiskChanged(paths));
  render();
}
