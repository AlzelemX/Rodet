// The settings dialog (Ctrl+,).
import { language, switchLanguage, t, type Language } from '../i18n';
import { type Settings, settings, updateSettings } from '../settings';
import { showDialog } from '../ui/notify';
import { themeName, themes } from '../theme';
import { h } from '../util';

function row(title: string, detail: string, control: HTMLElement) {
  return h('div.setting', {}, h('div.text', {}, h('b', {}, t(title)), h('span', {}, t(detail))), control);
}

function toggle(key: keyof Settings, title: string, detail: string) {
  const input = h('input', { type: 'checkbox', checked: !!settings[key] });
  input.addEventListener('change', () => updateSettings({ [key]: input.checked } as Partial<Settings>));
  return row(title, detail, h('label.switch', {}, input, h('span')));
}

function number(key: 'fontSize' | 'tabSize', title: string, detail: string, min: number, max: number) {
  const input = h('input', { type: 'number', min, max, value: String(settings[key]), style: { width: '64px' } });
  input.addEventListener('change', () => {
    const value = Math.min(max, Math.max(min, Number(input.value) || settings[key]));
    updateSettings({ [key]: value } as Partial<Settings>);
  });
  return row(title, detail, input);
}

export function openSettings() {
  const lang = h('select', {},
    h('option', { value: 'ar', selected: language === 'ar' }, 'العربية'),
    h('option', { value: 'en', selected: language === 'en' }, 'English'));
  lang.addEventListener('change', () => {
    const next = lang.value as Language;
    updateSettings({ language: next });
    // Saved first, then the window reloads in the new language.
    setTimeout(() => switchLanguage(next), 150);
  });
  const theme = h('select', {},
    [...themes.values()].map((th) => h('option', { value: th.id, selected: settings.theme === th.id }, themeName(th))));
  theme.addEventListener('change', () => updateSettings({ theme: theme.value }));
  showDialog(t('Settings'), h('div.settings', {},
    row('Language', 'The language of the editor, its messages and the Doc Book. The window reloads.', lang),
    row('Theme', 'Colors of the whole editor. Plugins can add more.', theme),
    number('fontSize', 'Font size', 'Size of the code text.', 9, 32),
    number('tabSize', 'Tab size', 'Spaces for one indentation level.', 1, 8),
    toggle('returnMarks', 'Return marks ↩', 'Show ↩ after the value a function gives back (the last line without `;`).'),
    toggle('speedScore', 'Speed score', 'Show the speed score of the file next to Run, with tips.'),
    toggle('typeHints', 'Type hints', 'Show the types Rodet works out, after `let` names.'),
    toggle('parameterHints', 'Parameter names', 'Show parameter names at calls without named arguments.'),
    toggle('checkOnSave', 'Check on save', 'Run the compiler on save and show its errors and fixes.'),
    toggle('autoSave', 'Auto save', 'Save files a moment after you stop typing, so errors update by themselves.'),
    toggle('minimap', 'Minimap', 'The small overview of the file on the right.'),
    toggle('wordWrap', 'Word wrap', 'Wrap long lines.')), true);
}
