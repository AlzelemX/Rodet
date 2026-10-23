// Arabic for the code editor's own texts (right-click menu, find box,
// completion list…). Monaco looks its messages up by number in
// `globalThis._VSCODE_NLS_MESSAGES`; `scripts/monaco-nls.mjs` turns the
// English → Arabic table `monaco-ar.json` into numbers for the Monaco
// version in use. This module must load before Monaco.
import { arabic } from './index';
import messages from './monaco-ar.gen.json';

if (arabic) {
  const table: string[] = [];
  for (const [index, text] of Object.entries(messages as Record<string, string>)) table[Number(index)] = text;
  (globalThis as any)._VSCODE_NLS_MESSAGES = table;
  (globalThis as any)._VSCODE_NLS_LANGUAGE = 'ar';
}
