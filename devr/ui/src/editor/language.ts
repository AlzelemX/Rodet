// Rodet and TOML coloring before the language server answers, and the
// DEVR+ color themes (close to VS Code's Dark Modern and Light Modern).
import * as monaco from './monaco';

// The language id is `rust` so the language server's ```rust code blocks in
// hovers are colored with it too.
export const RODET = 'rust';

const keywords = [
  'as', 'async', 'await', 'const', 'crate', 'dyn', 'enum', 'extern', 'fn', 'impl', 'let', 'mod',
  'move', 'mut', 'pub', 'ref', 'self', 'Self', 'static', 'struct', 'super', 'trait', 'type',
  'union', 'unsafe', 'use', 'where', 'data', 'gen',
];
const controlKeywords = [
  'break', 'continue', 'else', 'for', 'if', 'in', 'loop', 'match', 'return', 'while', 'yield', 'try',
];
const builtinTypes = [
  'i8', 'i16', 'i32', 'i64', 'i128', 'isize', 'u8', 'u16', 'u32', 'u64', 'u128', 'usize',
  'f16', 'f32', 'f64', 'f128', 'bool', 'char', 'str',
];

const rodet: monaco.languages.IMonarchLanguage = {
  defaultToken: '',
  keywords,
  controlKeywords,
  builtinTypes,
  constants: ['true', 'false', 'None', 'Some', 'Ok', 'Err'],
  tokenizer: {
    root: [
      // Rodet: `&{x, y}` borrow sources, `&_a` lifetimes.
      [/&(?=\s*\{)/, { token: 'operator', next: '@borrowSources' }],
      [/&(?=\s*_[a-z]\w*\b\s+[\w&(\[*<'])/, { token: 'operator', next: '@rppLifetime' }],
      [/'[a-zA-Z_]\w*(?!')/, 'lifetime'],
      [/#!?\[/, { token: 'attribute', next: '@attribute' }],
      [/\b(?:r#)?[a-z_]\w*!(?=\s*[([{])/, 'macro'],
      [/\b[a-z_]\w*(?=\s*(?:::<.*>)?\()/, {
        cases: { '@keywords': 'keyword', '@controlKeywords': 'keyword.control', '@default': 'function' },
      }],
      [/\b[A-Z][A-Z0-9_]+\b/, 'constant'],
      [/\b[A-Z]\w*\b/, { cases: { '@constants': 'constant.language', '@default': 'type' } }],
      [/\b[a-z_]\w*\b/, {
        cases: {
          '@keywords': 'keyword',
          '@controlKeywords': 'keyword.control',
          '@builtinTypes': 'type.builtin',
          '@constants': 'constant.language',
          '@default': 'identifier',
        },
      }],
      { include: '@whitespace' },
      [/b?r(#*)"/, { token: 'string', next: '@rawString.$1' }],
      [/b?"/, { token: 'string', next: '@string' }],
      [/b?'(?:\\(?:[nrt0\\'"]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\})|[^\\'])'/, 'string.char'],
      [/\b0x[0-9a-fA-F_]+(?:[iu](?:8|16|32|64|128|size))?\b/, 'number'],
      [/\b0o[0-7_]+(?:[iu](?:8|16|32|64|128|size))?\b/, 'number'],
      [/\b0b[01_]+(?:[iu](?:8|16|32|64|128|size))?\b/, 'number'],
      [/\b\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?(?:[iuf](?:8|16|32|64|128|size))?\b/, 'number'],
      [/[{}()[\]]/, '@brackets'],
      [/->|=>|::|\.\.=?|[=!<>]=|&&|\|\||[-+*/%^&|<>!=?]=?/, 'operator'],
      [/[;,.:]/, 'delimiter'],
    ],
    borrowSources: [
      [/\s+/, ''],
      [/\{/, 'borrow.brace'],
      [/self\b/, 'borrow.source'],
      [/[a-z_]\w*/, 'borrow.source'],
      [/,/, 'delimiter'],
      [/\}/, { token: 'borrow.brace', next: '@pop' }],
      [/./, { token: '', next: '@pop' }],
    ],
    rppLifetime: [
      [/\s+/, ''],
      [/_[a-z]\w*/, { token: 'lifetime', next: '@pop' }],
    ],
    attribute: [
      [/\]/, { token: 'attribute', next: '@pop' }],
      [/"/, { token: 'string', next: '@string' }],
      [/[^\]"]+/, 'attribute'],
    ],
    whitespace: [
      [/[ \t\r\n]+/, ''],
      [/\/\/[/!].*$/, 'comment.doc'],
      [/\/\/.*$/, 'comment'],
      [/\/\*/, { token: 'comment', next: '@blockComment' }],
    ],
    blockComment: [
      [/[^/*]+/, 'comment'],
      [/\/\*/, { token: 'comment', next: '@push' }],
      [/\*\//, { token: 'comment', next: '@pop' }],
      [/[/*]/, 'comment'],
    ],
    string: [
      [/\{\{|\}\}/, 'string.escape'],
      [/\{[a-zA-Z_]?[\w.]*(?::[^}]*)?\}/, 'string.format'],
      [/\\(?:[nrt0\\'"]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\}|\n)/, 'string.escape'],
      [/[^\\"{}]+/, 'string'],
      [/[{}]/, 'string'],
      [/"/, { token: 'string', next: '@pop' }],
      [/\\./, 'string.escape.invalid'],
    ],
    rawString: [
      [/"(#*)/, {
        cases: { '$1==$S2': { token: 'string', next: '@pop' }, '@default': 'string' },
      }],
      [/[^"]+/, 'string'],
    ],
  },
  brackets: [
    { open: '{', close: '}', token: 'delimiter.curly' },
    { open: '[', close: ']', token: 'delimiter.square' },
    { open: '(', close: ')', token: 'delimiter.parenthesis' },
  ],
};

const toml: monaco.languages.IMonarchLanguage = {
  defaultToken: '',
  tokenizer: {
    root: [
      [/^\s*\[\[?[^\]]+\]\]?/, 'type'],
      [/#.*$/, 'comment'],
      [/[A-Za-z0-9_.-]+(?=\s*=)/, 'variable'],
      [/"""/, { token: 'string', next: '@mlString' }],
      [/"([^"\\]|\\.)*"/, 'string'],
      [/'[^']*'/, 'string'],
      [/\b(true|false)\b/, 'constant.language'],
      [/\d{4}-\d{2}-\d{2}([T ][\d:.]+)?(Z|[+-]\d{2}:\d{2})?/, 'number'],
      [/[+-]?\d[\d_]*(\.\d+)?([eE][+-]?\d+)?/, 'number'],
      [/[{}[\]]/, '@brackets'],
      [/=/, 'operator'],
    ],
    mlString: [
      [/"""/, { token: 'string', next: '@pop' }],
      [/./, 'string'],
    ],
  },
};

export function registerLanguages() {
  monaco.languages.register({
    id: RODET,
    extensions: ['.rs'],
    aliases: ['Rodet', 'rust', 'rs'],
  });
  monaco.languages.setMonarchTokensProvider(RODET, rodet);
  monaco.languages.setLanguageConfiguration(RODET, {
    comments: { lineComment: '//', blockComment: ['/*', '*/'] },
    brackets: [
      ['{', '}'],
      ['[', ']'],
      ['(', ')'],
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"', notIn: ['string'] },
      { open: '/*', close: ' */', notIn: ['string'] },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: '<', close: '>' },
    ],
    indentationRules: {
      increaseIndentPattern: /^.*\{[^}"']*$|^.*\([^)"']*$|^.*\[[^\]"']*$/,
      decreaseIndentPattern: /^\s*[}\])]/,
    },
    onEnterRules: [
      { beforeText: /^\s*\/\/\/.*$/, action: { indentAction: monaco.languages.IndentAction.None, appendText: '/// ' } },
      { beforeText: /^\s*\/\/!.*$/, action: { indentAction: monaco.languages.IndentAction.None, appendText: '//! ' } },
    ],
    folding: { markers: { start: /^\s*\/\/\s*#?region\b/, end: /^\s*\/\/\s*#?endregion\b/ } },
    wordPattern: /(-?\d*\.\d\w*)|([^`~!@#$%^&*()\-=+[{\]}\\|;:'",.<>/?\s]+)/g,
  });

  monaco.languages.register({ id: 'toml', extensions: ['.toml'], aliases: ['TOML'], filenames: ['Cargo.lock'] });
  monaco.languages.setMonarchTokensProvider('toml', toml);
  monaco.languages.setLanguageConfiguration('toml', {
    comments: { lineComment: '#' },
    brackets: [
      ['{', '}'],
      ['[', ']'],
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '"', close: '"' },
    ],
  });
}

export function languageOf(path: string): string {
  const lower = path.toLowerCase();
  if (lower.endsWith('.rs')) return RODET;
  if (lower.endsWith('.toml') || lower.endsWith('cargo.lock')) return 'toml';
  if (lower.endsWith('.md')) return 'markdown';
  return 'plaintext';
}

// ---- Themes ----

interface Palette {
  base: 'vs' | 'vs-dark';
  bg: string;
  fg: string;
  keyword: string;
  control: string;
  type: string;
  builtin: string;
  func: string;
  macro: string;
  string: string;
  number: string;
  comment: string;
  lifetime: string;
  param: string;
  variable: string;
  constant: string;
  attribute: string;
  borrow: string;
  selection: string;
  lineHighlight: string;
  gutter: string;
  gutterActive: string;
  widget: string;
  border: string;
  accent: string;
}

const dark: Palette = {
  base: 'vs-dark',
  bg: '#1f1f1f',
  fg: '#cccccc',
  keyword: '#569cd6',
  control: '#c586c0',
  type: '#4ec9b0',
  builtin: '#4ec9b0',
  func: '#dcdcaa',
  macro: '#4fc1ff',
  string: '#ce9178',
  number: '#b5cea8',
  comment: '#6a9955',
  lifetime: '#ff9e5e',
  param: '#9cdcfe',
  variable: '#9cdcfe',
  constant: '#4fc1ff',
  attribute: '#c8c8c8',
  borrow: '#ff9e5e',
  selection: '#264f78',
  lineHighlight: '#2a2d2e80',
  gutter: '#6e7681',
  gutterActive: '#cccccc',
  widget: '#202020',
  border: '#2b2b2b',
  accent: '#e8590c',
};

const light: Palette = {
  base: 'vs',
  bg: '#ffffff',
  fg: '#3b3b3b',
  keyword: '#0000ff',
  control: '#af00db',
  type: '#267f99',
  builtin: '#267f99',
  func: '#795e26',
  macro: '#0070c1',
  string: '#a31515',
  number: '#098658',
  comment: '#008000',
  lifetime: '#b5470b',
  param: '#001080',
  variable: '#001080',
  constant: '#0070c1',
  attribute: '#616161',
  borrow: '#b5470b',
  selection: '#add6ff',
  lineHighlight: '#f5f5f5',
  gutter: '#6e7681',
  gutterActive: '#171184',
  widget: '#f8f8f8',
  border: '#e5e5e5',
  accent: '#c2410c',
};

function theme(p: Palette): monaco.editor.IStandaloneThemeData {
  const c = (color: string) => color.replace('#', '');
  return {
    base: p.base,
    inherit: true,
    rules: [
      { token: '', foreground: c(p.fg) },
      { token: 'keyword', foreground: c(p.keyword) },
      { token: 'keyword.control', foreground: c(p.control) },
      { token: 'type', foreground: c(p.type) },
      { token: 'type.builtin', foreground: c(p.builtin) },
      { token: 'function', foreground: c(p.func) },
      { token: 'macro', foreground: c(p.macro) },
      { token: 'string', foreground: c(p.string) },
      { token: 'string.char', foreground: c(p.string) },
      { token: 'string.escape', foreground: c(p.constant) },
      { token: 'string.format', foreground: c(p.param) },
      { token: 'number', foreground: c(p.number) },
      { token: 'comment', foreground: c(p.comment) },
      { token: 'comment.doc', foreground: c(p.comment) },
      { token: 'lifetime', foreground: c(p.lifetime), fontStyle: 'italic' },
      { token: 'borrow.brace', foreground: c(p.borrow), fontStyle: 'bold' },
      { token: 'borrow.source', foreground: c(p.borrow), fontStyle: 'italic' },
      { token: 'attribute', foreground: c(p.attribute) },
      { token: 'constant', foreground: c(p.constant) },
      { token: 'constant.language', foreground: c(p.keyword) },
      { token: 'identifier', foreground: c(p.variable) },
      { token: 'operator', foreground: c(p.fg) },
      { token: 'variable', foreground: c(p.variable) },

      // Semantic tokens from rodet-analyzer.
      { token: 'namespace', foreground: c(p.type) },
      { token: 'struct', foreground: c(p.type) },
      { token: 'enum', foreground: c(p.type) },
      { token: 'union', foreground: c(p.type) },
      { token: 'interface', foreground: c(p.type) },
      { token: 'typeAlias', foreground: c(p.type) },
      { token: 'typeParameter', foreground: c(p.type) },
      { token: 'builtinType', foreground: c(p.builtin) },
      { token: 'selfTypeKeyword', foreground: c(p.keyword) },
      { token: 'enumMember', foreground: c(p.constant) },
      { token: 'function', foreground: c(p.func) },
      { token: 'method', foreground: c(p.func) },
      { token: 'macro', foreground: c(p.macro) },
      { token: 'macroBang', foreground: c(p.macro) },
      { token: 'parameter', foreground: c(p.param) },
      { token: 'variable', foreground: c(p.variable) },
      { token: 'property', foreground: c(p.variable) },
      { token: 'const', foreground: c(p.constant) },
      { token: 'static', foreground: c(p.constant) },
      { token: 'constParameter', foreground: c(p.constant) },
      { token: 'lifetime', foreground: c(p.lifetime), fontStyle: 'italic' },
      { token: 'label', foreground: c(p.lifetime) },
      { token: 'keyword', foreground: c(p.keyword) },
      { token: 'selfKeyword', foreground: c(p.keyword) },
      { token: 'boolean', foreground: c(p.keyword) },
      { token: 'string', foreground: c(p.string) },
      { token: 'character', foreground: c(p.string) },
      { token: 'escapeSequence', foreground: c(p.constant) },
      { token: 'formatSpecifier', foreground: c(p.constant) },
      { token: 'number', foreground: c(p.number) },
      { token: 'comment', foreground: c(p.comment) },
      { token: 'attribute', foreground: c(p.attribute) },
      { token: 'attributeBracket', foreground: c(p.attribute) },
      { token: 'builtinAttribute', foreground: c(p.attribute) },
      { token: 'derive', foreground: c(p.type) },
      { token: 'unresolvedReference', foreground: c(p.fg) },
      { token: '*.mutable', fontStyle: 'underline' },
      { token: '*.unsafe', foreground: 'f14c4c' },
      { token: '*.controlFlow', foreground: c(p.control) },
      { token: 'keyword.controlFlow', foreground: c(p.control) },
    ],
    colors: {
      'editor.background': p.bg,
      'editor.foreground': p.fg,
      'editor.selectionBackground': p.selection,
      'editor.lineHighlightBackground': p.lineHighlight,
      'editor.lineHighlightBorder': '#00000000',
      'editorLineNumber.foreground': p.gutter,
      'editorLineNumber.activeForeground': p.gutterActive,
      'editorCursor.foreground': p.accent,
      'editorWidget.background': p.widget,
      'editorWidget.border': p.border,
      'editorSuggestWidget.background': p.widget,
      'editorSuggestWidget.border': p.border,
      'editorSuggestWidget.selectedBackground': p.base === 'vs-dark' ? '#04395e' : '#e4e6f1',
      'editorHoverWidget.background': p.widget,
      'editorHoverWidget.border': p.border,
      'editorIndentGuide.background1': p.base === 'vs-dark' ? '#404040' : '#d3d3d3',
      'editorIndentGuide.activeBackground1': p.base === 'vs-dark' ? '#707070' : '#939393',
      'editorInlayHint.foreground': p.base === 'vs-dark' ? '#969696' : '#808080',
      'editorInlayHint.background': p.base === 'vs-dark' ? '#2a2a2a' : '#f0f0f0',
      'editorGutter.background': p.bg,
      'minimap.background': p.bg,
      'scrollbarSlider.background': p.base === 'vs-dark' ? '#79797966' : '#64646466',
      'focusBorder': p.accent,
      'editorBracketMatch.border': p.accent + '99',
      'editorBracketMatch.background': '#00000000',
      'peekViewTitle.background': p.widget,
      'peekViewEditor.background': p.base === 'vs-dark' ? '#1a1a1a' : '#f8f8f8',
      'peekViewResult.background': p.widget,
      'peekView.border': p.accent,
    },
  };
}

export function registerThemes() {
  monaco.editor.defineTheme('devr-dark', theme(dark));
  monaco.editor.defineTheme('devr-light', theme(light));
}
