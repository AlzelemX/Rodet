// Builds ui/src/i18n/monaco-ar.gen.json: { messageNumber: arabic } for the
// installed monaco-editor, from the English → Arabic table monaco-ar.json.
// Monaco numbers its messages when it is built, so this runs before every
// build (`npm run build` / `npm run dev`).
import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const esm = join(here, '..', 'node_modules', 'monaco-editor', 'esm', 'vs');
const table = JSON.parse(readFileSync(join(here, '..', 'ui', 'src', 'i18n', 'monaco-ar.json'), 'utf8'));
const out = join(here, '..', 'ui', 'src', 'i18n', 'monaco-ar.gen.json');

// localize(123, "text"…) and localize2(123, 'text'…)
const call = /localize2?\(\s*(\d+)\s*,\s*("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')/g;

function* files(dir) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) yield* files(path);
    else if (name.endsWith('.js')) yield path;
  }
}

function unquote(literal) {
  if (literal.startsWith("'")) literal = '"' + literal.slice(1, -1).replace(/\\'/g, "'").replace(/"/g, '\\"') + '"';
  try {
    return JSON.parse(literal);
  } catch {
    return null;
  }
}

const result = {};
const used = new Set();
for (const file of files(esm)) {
  const source = readFileSync(file, 'utf8');
  for (const m of source.matchAll(call)) {
    const english = unquote(m[2]);
    if (english === null) continue;
    // Menu labels carry mnemonics: "Go to &&Definition".
    const key = english in table ? english : english.replace(/&&/g, '');
    if (key in table) {
      result[m[1]] = table[key];
      used.add(key);
    }
  }
}
writeFileSync(out, JSON.stringify(result));
const unused = Object.keys(table).filter((k) => !used.has(k));
console.log(`monaco-nls: ${Object.keys(result).length} Arabic messages${unused.length ? `; not in this Monaco: ${unused.length}` : ''}`);
if (process.argv.includes('--verbose')) for (const k of unused) console.log('  unused:', k);
