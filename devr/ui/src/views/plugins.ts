// The Extensions view (Ctrl+Shift+X): installed plugins, turn them on and
// off, reload, remove, create a new one, install one from a folder.
import { backend, errorText } from '../backend';
import { t } from '../i18n';
import {
  isEnabled, loadPlugins, onPlugins, pluginName, pluginOfPath, plugins, reloadPlugin, removePlugin,
  setEnabled, text, type Plugin,
} from '../plugins/host';
import type { SideView } from '../ui/layout';
import { confirm, prompt, toast } from '../ui/notify';
import { registerCommand } from '../ui/palette';
import { h, icon, joinPath } from '../util';
import { workspace } from '../workspace';
import { openBook } from './book';

const body = h('div');
const element = h('div', { style: { overflow: 'auto' } }, body);

function stateLabel(p: Plugin): [string, string] {
  if (p.error && p.state !== 'failed') return ['error', t('Can’t load')];
  switch (p.state) {
    case 'on': return ['on', t('On')];
    case 'starting': return ['starting', t('Starting…')];
    case 'failed': return ['error', t('Failed')];
    default: return ['off', t('Off')];
  }
}

function addedSummary(p: Plugin) {
  const parts: string[] = [];
  if (p.added.commands.size) parts.push(t('{n} commands', { n: p.added.commands.size }));
  if (p.added.themes.size) parts.push(t('{n} themes', { n: p.added.themes.size }));
  if (p.added.views.size) parts.push(t('{n} views', { n: p.added.views.size }));
  if (p.manifest.styles?.length && p.state === 'on') parts.push(t('styles'));
  return parts.join(' · ');
}

function card(p: Plugin) {
  const [cls, label] = stateLabel(p);
  const toggle = h('input', { type: 'checkbox', checked: isEnabled(p.id), disabled: !!p.error && p.state !== 'failed' });
  toggle.addEventListener('change', () => setEnabled(p.id, toggle.checked));
  const description = text(p.manifest.description);
  const summary = addedSummary(p);
  return h('div.plugin-card', {},
    h('div.plugin-head', {},
      icon('extensions', 'plugin-icon'),
      h('div.plugin-title', {},
        h('b', {}, pluginName(p)),
        p.manifest.version ? h('span.plugin-version', {}, p.manifest.version) : null),
      h('label.switch', { title: t('Turn on or off') }, toggle, h('span'))),
    description ? h('div.plugin-desc', {}, description) : null,
    h('div.plugin-meta', {},
      h(`span.plugin-state.${cls}` as 'span', {}, label),
      p.manifest.author ? h('span', {}, p.manifest.author) : null,
      summary ? h('span', {}, summary) : null),
    p.error ? h('div.plugin-error', {}, p.error) : null,
    h('div.plugin-actions', {},
      h('button.icon-button', { title: t('Reload'), onclick: () => reloadPlugin(p.id) }, icon('refresh')),
      h('button.icon-button', { title: t('Edit the plugin'), onclick: () => editPlugin(p) }, icon('edit')),
      h('button.icon-button', {
        title: t('Remove'),
        onclick: async () => {
          if (await confirm(t('Remove the plugin {name}?', { name: pluginName(p) }), t('Its folder is deleted.'), t('Remove'), true)) {
            removePlugin(p.id).catch((err) => toast(errorText(err), 'error'));
          }
        },
      }, icon('trash'))));
}

function render() {
  body.replaceChildren();
  const list = [...plugins.values()];
  if (!list.length) {
    body.appendChild(h('div.empty-state', {},
      t('No plugins yet. Plugins add commands, views, themes and styles: they can change anything in DEVR+.'),
      h('br'),
      h('button.btn.primary', { onclick: () => createPlugin() }, icon('add'), t('Create a Plugin')),
      h('br'),
      h('button.btn', { onclick: () => installPlugin() }, icon('folder-opened'), t('Install from a Folder…')),
      h('br'),
      h('button.btn', { onclick: () => openBook('start/plugins.md') }, icon('book'), t('How to write a plugin'))));
    return;
  }
  body.append(...list.map(card));
}

export const pluginsView: SideView = {
  id: 'plugins',
  title: t('Extensions'),
  icon: 'extensions',
  element,
  onShow: () => render(),
  actions: [
    h('button.icon-button', { title: t('Create a Plugin'), onclick: () => createPlugin() }, icon('add')),
    h('button.icon-button', { title: t('Install from a Folder…'), onclick: () => installPlugin() }, icon('folder-opened')),
    h('button.icon-button', { title: t('Reload All Plugins'), onclick: () => loadPlugins() }, icon('refresh')),
    h('button.icon-button', { title: t('Plugins Folder'), onclick: () => showFolder() }, icon('folder')),
  ],
};

/** Opens the plugin's main file (or plugin.json) in a tab. */
function editPlugin(p: Plugin) {
  const sep = workspace.info.pathSeparator;
  const file = p.manifest.main ?? 'plugin.json';
  workspace.open(p.path + sep + file.replace(/[\\/]/g, sep));
}

function slug(name: string) {
  return name.toLowerCase().normalize('NFKD').replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '').slice(0, 40);
}

const ID_RULE = /^[a-z0-9][a-z0-9._-]{0,63}$/;

export async function createPlugin() {
  const name = await prompt(t('Name of the new plugin'), t('My Plugin'));
  if (!name?.trim()) return;
  let id: string | null = slug(name);
  if (!ID_RULE.test(id) || plugins.has(id)) {
    id = await prompt(t('Plugin id (a-z, 0-9, - . _)'), id || 'my-plugin', {
      validate: (v) => (!ID_RULE.test(v) ? t('Use a-z, 0-9, `-`, `.` and `_`.') : plugins.has(v) ? t('This id is taken.') : null),
    });
    if (!id) return;
  }
  try {
    const folder = await backend.call<string>('plugins.create', { id, name: name.trim() });
    await loadPlugins();
    await workspace.open(joinPath(folder, 'main.js'));
    toast(t('The plugin {name} is on. Change main.js and save: it reloads by itself.', { name: name.trim() }), 'success', [
      { label: t('How to write a plugin'), run: () => openBook('start/plugins.md') },
    ]);
  } catch (err) {
    toast(errorText(err), 'error');
  }
}

export async function installPlugin() {
  let path: string | null = null;
  if (backend.kind === 'desktop') {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const picked = await open({ directory: true, multiple: false, title: t('The folder of the plugin (with plugin.json)') });
    path = typeof picked === 'string' ? picked : null;
  } else {
    path = await prompt(t('The folder of the plugin (with plugin.json), on the computer running devr-server'), workspace.info.home ?? '');
  }
  if (!path) return;
  const ok = await confirm(
    t('Install this plugin?'),
    t('A plugin can do anything DEVR+ can: read and change your files and run programs. Install only plugins you trust.'),
    t('Install'),
  );
  if (!ok) return;
  let id: string;
  try {
    id = await backend.call<string>('plugins.install', { from: path });
  } catch (err) {
    const message = errorText(err);
    if (!message.includes('already installed')) {
      toast(message, 'error');
      return;
    }
    if (!(await confirm(t('Replace the installed plugin?'), message, t('Replace'), true))) return;
    try {
      id = await backend.call<string>('plugins.install', { from: path, replace: true });
    } catch (err2) {
      toast(errorText(err2), 'error');
      return;
    }
  }
  await loadPlugins();
  const p = plugins.get(id);
  if (p) toast(t('{name} is installed.', { name: pluginName(p) }), p.error ? 'warning' : 'success');
}

async function showFolder() {
  const dir = await backend.call<string>('plugins.dir');
  toast(t('Plugins are kept in: {dir}', { dir }), 'info', [
    { label: t('Copy Path'), run: () => navigator.clipboard?.writeText(dir).catch(() => {}) },
    { label: t('Open as Folder'), run: () => backend.call('fs.createDir', { path: dir }).catch(() => {}).then(() => workspace.openFolder(dir)) },
  ]);
}

export function setupPlugins() {
  onPlugins.on(() => render());
  // Saving a file of a plugin reloads it: change, save, see.
  workspace.onSaved.on((doc) => {
    const p = pluginOfPath(doc.path);
    if (p) reloadPlugin(p.id);
  });
  const C = registerCommand;
  C({ id: 'view.plugins', title: t('View: Extensions'), key: 'Ctrl+Shift+X', icon: 'extensions', run: () => import('../ui/layout').then((m) => m.layout.showSide('plugins')) });
  C({ id: 'plugins.create', title: t('Plugins: Create a Plugin…'), icon: 'add', run: () => createPlugin() });
  C({ id: 'plugins.install', title: t('Plugins: Install from a Folder…'), icon: 'folder-opened', run: () => installPlugin() });
  C({ id: 'plugins.reload', title: t('Plugins: Reload All'), icon: 'refresh', run: () => loadPlugins() });
  C({ id: 'plugins.folder', title: t('Plugins: Show Plugins Folder'), icon: 'folder', run: () => showFolder() });
}

/** Starts the plugins that are on (after the workbench is ready). */
export async function startPlugins() {
  await loadPlugins();
}
