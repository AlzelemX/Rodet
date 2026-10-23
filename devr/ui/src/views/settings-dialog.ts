// The settings dialog (Ctrl+,).
import { type Settings, settings, updateSettings } from '../settings';
import { showDialog } from '../ui/notify';
import { h } from '../util';

function toggle(key: keyof Settings, title: string, detail: string) {
  const input = h('input', { type: 'checkbox', checked: !!settings[key] });
  input.addEventListener('change', () => updateSettings({ [key]: input.checked } as Partial<Settings>));
  return h('div.setting', {}, h('div.text', {}, h('b', {}, title), h('span', {}, detail)), h('label.switch', {}, input, h('span')));
}

function number(key: 'fontSize' | 'tabSize', title: string, detail: string, min: number, max: number) {
  const input = h('input', { type: 'number', min, max, value: String(settings[key]), style: { width: '64px' } });
  input.addEventListener('change', () => {
    const value = Math.min(max, Math.max(min, Number(input.value) || settings[key]));
    updateSettings({ [key]: value } as Partial<Settings>);
  });
  return h('div.setting', {}, h('div.text', {}, h('b', {}, title), h('span', {}, detail)), input);
}

export function openSettings() {
  const theme = h('select', {},
    h('option', { value: 'dark', selected: settings.theme === 'dark' }, 'Dark'),
    h('option', { value: 'light', selected: settings.theme === 'light' }, 'Light'));
  theme.addEventListener('change', () => updateSettings({ theme: theme.value as 'dark' | 'light' }));
  showDialog('Settings', h('div.settings', {},
    h('div.setting', {}, h('div.text', {}, h('b', {}, 'Theme'), h('span', {}, 'Colors of the whole editor.')), theme),
    number('fontSize', 'Font size', 'Size of the code text.', 9, 32),
    number('tabSize', 'Tab size', 'Spaces for one indentation level.', 1, 8),
    toggle('returnMarks', 'Return marks ↩', 'Show ↩ after the value a function gives back (the last line without `;`).'),
    toggle('speedScore', 'Speed score', 'Show the speed score of the file in the corner, with tips.'),
    toggle('typeHints', 'Type hints', 'Show the types Rust++ works out, after `let` names.'),
    toggle('parameterHints', 'Parameter names', 'Show parameter names at calls without named arguments.'),
    toggle('checkOnSave', 'Check on save', 'Run the compiler on save and show its errors and fixes.'),
    toggle('autoSave', 'Auto save', 'Save files a moment after you stop typing.'),
    toggle('minimap', 'Minimap', 'The small overview of the file on the right.'),
    toggle('wordWrap', 'Word wrap', 'Wrap long lines.')), true);
}
