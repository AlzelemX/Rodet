// @ts-check
// TODOs: shows every TODO / FIXME of the open project in a side view.
// Uses: a side view, a command with a key, a status item, the backend's search.

/** @param {import('../../devr').Devr} devr */
export function activate(devr) {
  const { h, icon } = devr.ui;
  devr.addTranslations({
    'TODOs': 'المهام',
    'TODOs: Refresh': 'المهام: تحديث',
    'No TODO or FIXME in this project.': 'لا يوجد TODO أو FIXME في هذا المشروع.',
    'Open a folder first.': 'افتح مجلدًا أولًا.',
    '{n} TODOs': '{n} مهام',
  });

  const list = h('div', { style: { padding: '4px 0' } });
  const status = devr.ui.addStatusItem({ icon: 'checklist', tooltip: devr.t('TODOs'), onClick: () => devr.ui.showSide('todos.view') });

  async function refresh() {
    list.replaceChildren();
    const root = devr.workspace.root;
    if (!root) {
      list.append(h('div.empty-state', {}, devr.t('Open a folder first.')));
      status.set('');
      return;
    }
    /** @type {{ path: string, line: number, text: string }[]} */
    const found = [];
    const result = await devr.backend.call('search.text', { root, query: '\\b(TODO|FIXME)\\b', regex: true, caseSensitive: true });
    for (const file of result.files) {
      for (const m of file.matches) found.push({ path: file.path, line: m.line, text: m.preview.trim() });
    }
    status.set(found.length ? String(found.length) : '');
    if (!found.length) {
      list.append(h('div.empty-state', {}, devr.t('No TODO or FIXME in this project.')));
      return;
    }
    for (const item of found) {
      const name = item.path.slice(root.length + 1);
      list.append(h('div.search-match', {
        style: { padding: '3px 12px', cursor: 'pointer' },
        title: `${name}:${item.line}`,
        onclick: () => devr.workspace.open(item.path, { line: item.line }),
      }, icon('circle-small'), ' ', item.text, h('span', { style: { color: 'var(--fg-faint)', marginInlineStart: '6px' } }, `${name}:${item.line}`)));
    }
  }

  devr.ui.addSideView({
    id: 'todos.view',
    title: devr.t('TODOs'),
    icon: 'checklist',
    element: h('div', { style: { overflow: 'auto' } }, list),
    onShow: refresh,
    actions: [h('button.icon-button', { title: devr.t('TODOs: Refresh'), onclick: refresh }, icon('refresh'))],
  });
  devr.commands.register({ id: 'todos.refresh', title: devr.t('TODOs: Refresh'), icon: 'checklist', key: 'Ctrl+Alt+T', run: refresh });
  devr.workspace.onRoot(refresh);
  devr.workspace.onSave(refresh);
  refresh();
}
