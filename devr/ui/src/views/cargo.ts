// The Cargo view: run, build and test the project, its programs and
// libraries (crates), and which libraries have a newer version.
import { t } from '../i18n';
import { backend, errorText } from '../backend';
import type { SideView } from '../ui/layout';
import { prompt, toast } from '../ui/notify';
import { h, icon, joinPath } from '../util';
import { workspace } from '../workspace';
import { runCargo } from './terminal';

interface Dependency {
  name: string;
  version: string;
  kind: string;
  source: string | null;
}
interface Project {
  root: string | null;
  name: string | null;
  version: string | null;
  edition: string | null;
  bins: string[];
  examples: string[];
  hasLib: boolean;
  dependencies: Dependency[];
  members: string[];
}

const body = h('div');
const element = h('div', { style: { overflow: 'auto' } }, body);
let project: Project | null = null;
const latest = new Map<string, string | null>();

function actionButton(label: string, iconName: string, args: string[], title?: string) {
  return h('button.btn.small', { title: title ?? `cargo ${args.join(' ')}`, onclick: () => runCargo(args, args.length === 1 && args[0] === 'run' ? 'cargo run' : undefined) }, icon(iconName), t(label));
}

async function load() {
  const root = workspace.root;
  if (!root) {
    project = null;
    render();
    return;
  }
  try {
    project = await backend.call<Project>('cargo.project', { path: root });
  } catch (err) {
    toast(errorText(err), 'error');
    project = null;
  }
  render();
}

function render() {
  body.replaceChildren();
  if (!workspace.root) {
    body.appendChild(h('div.empty-state', {}, t('Open a Cargo project to run, build and test it.'),
      h('br'), h('button.btn.primary', { onclick: () => newProject() }, icon('add'), t('New Rust++ Project'))));
    return;
  }
  if (!project?.root) {
    body.appendChild(h('div.empty-state', {}, t('This folder has no Cargo.toml.'),
      h('br'), h('button.btn.primary', { onclick: () => newProject() }, icon('add'), t('New Rust++ Project'))));
    return;
  }
  const p = project;
  body.append(
    h('div.pad', {},
      h('div', { style: { display: 'flex', alignItems: 'center', gap: '8px' } },
        icon('package'),
        h('b', { style: { fontSize: '14px', color: 'var(--fg-strong)' } }, p.name ?? (p.members.length ? t('workspace') : t('project'))),
        h('span.faint', {}, [p.version, p.edition ? t('edition {edition}', { edition: p.edition }) : null].filter(Boolean).join(' · '))),
      h('div.cargo-actions', {},
        actionButton('Run', 'play', ['run']),
        actionButton('Build', 'tools', ['build']),
        actionButton('Check', 'check', ['check']),
        actionButton('Test', 'beaker', ['test']),
        actionButton('Run fast', 'rocket', ['run', '--release'], t('cargo run --release: optimized, measures real speed')),
        actionButton('Clean', 'trash', ['clean'])),
    ),
  );

  if (p.bins.length > 1 || p.examples.length) {
    body.appendChild(h('div.section-title', { style: { paddingLeft: '14px' } }, t('Programs')));
    for (const bin of p.bins) {
      body.appendChild(h('div.row', { style: { paddingLeft: '20px' }, onclick: () => runCargo(['run', '--bin', bin], `run ${bin}`) },
        icon('play'), h('span.label', {}, bin), h('span.desc', {}, 'bin')));
    }
    for (const example of p.examples) {
      body.appendChild(h('div.row', { style: { paddingLeft: '20px' }, onclick: () => runCargo(['run', '--example', example], `example ${example}`) },
        icon('play'), h('span.label', {}, example), h('span.desc', {}, t('example'))));
    }
  }

  body.appendChild(h('div.section-title', { style: { paddingLeft: '14px', marginTop: '8px' } }, t('Libraries (crates)'),
    h('span.actions', { style: { display: 'flex' } },
      h('button.icon-button', { title: t('Add a library'), onclick: (e: MouseEvent) => { e.stopPropagation(); addDependency(); } }, icon('add')),
      h('button.icon-button', { title: t('Check for newer versions'), onclick: (e: MouseEvent) => { e.stopPropagation(); checkVersions(true); } }, icon('refresh')),
      h('button.icon-button', { title: t('cargo update'), onclick: (e: MouseEvent) => { e.stopPropagation(); runCargo(['update']); } }, icon('cloud-download')))));
  if (!p.dependencies.length) {
    body.appendChild(h('div.pad.muted', {}, t('No libraries yet. Add one with +, for example “rand”.')));
  }
  for (const dep of p.dependencies) {
    const newest = latest.get(dep.name);
    const outdated = newest && dep.version && !versionMatches(dep.version, newest);
    body.appendChild(h('div.row', {
      style: { paddingLeft: '20px' },
      title: dep.source ?? `${dep.name} = "${dep.version}"`,
      onclick: () => openUrl(`https://crates.io/crates/${dep.name}`),
    },
    icon(dep.source ? 'folder-library' : 'library'),
    h('span.label', {}, dep.name),
    h(`span.desc.dep-version${outdated ? '.old' : ''}` as 'span', {}, dep.version || (dep.source ? t('local') : '')),
    dep.kind !== 'dependencies' ? h('span.desc', {}, dep.kind.replace('-dependencies', '')) : null,
    h('span.end', {}, outdated ? h('span.dep-version.new', { title: t('Newest: {version}', { version: newest }) }, `→ ${newest}`) : null)));
  }
  checkVersions(false);
}

function versionMatches(requirement: string, newest: string) {
  const req = requirement.replace(/^[\^~=\s]+/, '').split('.');
  const got = newest.split('.');
  // Cargo's default (caret) requirement: same first non-zero part.
  const major = req[0] === '0' ? 2 : 1;
  return req.slice(0, major).every((part, i) => part === got[i]);
}

let checking = false;
async function checkVersions(force: boolean) {
  if (!project || checking) return;
  const names = project.dependencies.filter((d) => !d.source && (force || !latest.has(d.name))).map((d) => d.name);
  if (!names.length) return;
  checking = true;
  try {
    await Promise.all(names.map(async (name) => {
      try {
        latest.set(name, await backend.call<string | null>('cargo.latestVersion', { name }));
      } catch {
        latest.set(name, null);
      }
    }));
  } finally {
    checking = false;
  }
  render();
}

async function addDependency() {
  const name = await prompt(t('Add a library (crate) from crates.io'), '', {
    placeholder: t('e.g. rand, serde --features derive'),
    validate: (v) => (/^[\w-]+(\s+.*)?$/.test(v.trim()) ? null : t('Use the crate name, optionally followed by options')),
  });
  if (!name) return;
  runCargo(['add', ...name.trim().split(/\s+/)], 'cargo add');
}

export async function newProject() {
  const parent = workspace.root ? workspace.root.replace(/[\\/][^\\/]+$/, '') : workspace.info.home ?? '.';
  const name = await prompt(t('New Rust++ project in {folder}', { folder: parent }), 'hello', {
    validate: (v) => (/^[A-Za-z][\w-]*$/.test(v) ? null : t('Start with a letter; use letters, numbers, - and _')),
  });
  if (!name) return;
  try {
    const path = await backend.call<string>('cargo.new', { parent, name });
    await workspace.openFolder(path);
    workspace.open(joinPath(joinPath(path, 'src'), 'main.rs'));
    toast(t('Created {name}. Press F5 to run it.', { name }), 'success');
  } catch (err) {
    toast(errorText(err), 'error');
  }
}

function openUrl(url: string) {
  if (backend.kind === 'desktop') {
    import('@tauri-apps/plugin-opener').then((m) => m.openUrl(url)).catch(() => window.open(url, '_blank'));
  } else window.open(url, '_blank');
}

export const cargo: SideView = {
  id: 'cargo',
  title: t('Cargo'),
  icon: 'package',
  element,
  onShow: () => load(),
  actions: [h('button.icon-button', { title: t('Reload Cargo.toml'), onclick: () => load() }, icon('refresh'))],
};

export function setupCargo() {
  workspace.onRoot.on(() => {
    latest.clear();
    load();
  });
  workspace.onSaved.on((doc) => {
    if (doc.path.toLowerCase().endsWith('cargo.toml')) load();
  });
  backend.on('fs.changed', ({ paths }: { paths: string[] }) => {
    if (paths.some((p) => p.toLowerCase().endsWith('cargo.toml'))) load();
  });
}
