// The Doc Book inside the editor: the book that comes with the installed
// Rust++ (so it always matches the language version), in Arabic or English,
// with its chapters on the side.
import { Marked } from 'marked';
import * as monaco from '../editor/monaco';
import { RUSTPP } from '../editor/language';
import { backend, errorText } from '../backend';
import { arabic, isolateLatin, t } from '../i18n';
import { runCommand } from '../ui/palette';
import { h, icon } from '../util';
import { workspace } from '../workspace';

export const BOOK_TAB = 'devr-book:';
const ONLINE = 'https://alzelemx.github.io/Rust-/';

interface Chapter {
  title: string;
  path: string;
  depth: number;
}
interface Section {
  title: string;
  chapters: Chapter[];
}

let info: { root: string | null; languages: string[] } | null = null;
let language = arabic ? 'ar' : 'en';
let sections: Section[] = [];
let current = '';

const toc = h('nav.book-toc');
const page = h('article.book-page');
const scroller = h('div.book-scroll', {}, page);
const languageSwitch = h('div.book-languages');
export const bookElement = h('div.book.hidden', {},
  h('aside.book-side', {}, h('div.book-side-title', {}, icon('book'), t('Doc Book')), languageSwitch, toc),
  scroller);

const marked = new Marked({ gfm: true });

function chapters() {
  return sections.flatMap((s) => s.chapters);
}

/** Reads SUMMARY.md: `# Section` headings and `- [Title](path.md)` lines. */
function parseSummary(text: string): Section[] {
  const result: Section[] = [{ title: '', chapters: [] }];
  for (const line of text.split('\n')) {
    const heading = line.match(/^#+\s+(.+)/);
    if (heading && !/^#\s+Summary/i.test(line)) {
      result.push({ title: heading[1].trim(), chapters: [] });
      continue;
    }
    const item = line.match(/^(\s*)[-*]?\s*\[(.+?)\]\((.+?\.md)\)/);
    if (item) {
      result[result.length - 1].chapters.push({ title: item[2], path: item[3].replace(/^\.\//, ''), depth: Math.floor(item[1].length / 2) });
    }
  }
  return result.filter((s) => s.chapters.length);
}

function dirOf(path: string) {
  const i = path.lastIndexOf('/');
  return i < 0 ? '' : path.slice(0, i + 1);
}

/** `../basics/a.md` seen from `start/b.md` → `basics/a.md`. */
function resolve(from: string, link: string) {
  const parts = (dirOf(from) + link).split('/');
  const out: string[] = [];
  for (const part of parts) {
    if (part === '..') out.pop();
    else if (part && part !== '.') out.push(part);
  }
  return out.join('/');
}

function renderLanguages() {
  const names: Record<string, string> = { ar: 'العربية', en: 'English' };
  languageSwitch.replaceChildren(...(info?.languages ?? []).map((lang) =>
    h(lang === language ? 'button.chip.on' : 'button.chip', { onclick: () => switchBookLanguage(lang) }, names[lang] ?? lang)));
}

function renderToc() {
  toc.replaceChildren();
  for (const section of sections) {
    if (section.title) toc.appendChild(h('div.book-section', {}, language === 'ar' ? isolateLatin(section.title) : section.title));
    for (const chapter of section.chapters) {
      toc.appendChild(h(chapter.path === current ? 'a.book-link.active' : 'a.book-link', {
        style: { paddingInlineStart: `${12 + chapter.depth * 12}px` },
        onclick: () => showChapter(chapter.path),
      }, language === 'ar' ? isolateLatin(chapter.title) : chapter.title));
    }
  }
}

async function switchBookLanguage(lang: string) {
  language = lang;
  await loadSummary();
  const keep = chapters().some((c) => c.path === current) ? current : chapters()[0]?.path ?? '';
  await showChapter(keep);
}

async function loadSummary() {
  bookElement.setAttribute('dir', language === 'ar' ? 'rtl' : 'ltr');
  const text = await backend.call<string>('book.read', { language, path: 'SUMMARY.md' });
  sections = parseSummary(text);
  renderLanguages();
}

function openUrl(url: string) {
  if (backend.kind === 'desktop') {
    import('@tauri-apps/plugin-opener').then((m) => m.openUrl(url)).catch(() => window.open(url, '_blank'));
  } else window.open(url, '_blank');
}

/** Shows a chapter: Markdown → HTML, Rust++ code colored like the editor. */
async function showChapter(path: string) {
  current = path;
  renderToc();
  let text: string;
  try {
    text = await backend.call<string>('book.read', { language, path });
  } catch (err) {
    page.replaceChildren(h('p', {}, errorText(err)));
    return;
  }
  page.innerHTML = await marked.parse(text);
  // Code: colored with the editor's own colors (code is always left to right).
  for (const block of page.querySelectorAll('pre > code')) {
    const pre = block.parentElement!;
    pre.setAttribute('dir', 'ltr');
    const lang = (block.className.match(/language-(\w+)/)?.[1] ?? 'rust').toLowerCase();
    if (['rust', 'rs', 'rustpp'].includes(lang) || !block.className) {
      block.innerHTML = await monaco.editor.colorize(block.textContent ?? '', RUSTPP, { tabSize: 4 });
    }
  }
  for (const code of page.querySelectorAll(':not(pre) > code')) code.setAttribute('dir', 'ltr');
  if (language === 'ar') {
    // "Rust++" inside Arabic sentences would show as "++Rust".
    const walker = document.createTreeWalker(page, NodeFilter.SHOW_TEXT);
    for (let node = walker.nextNode(); node; node = walker.nextNode()) {
      if (!(node.parentElement?.closest('pre, code'))) (node as Text).data = isolateLatin((node as Text).data);
    }
  }
  for (const img of page.querySelectorAll('img')) {
    const src = img.getAttribute('src') ?? '';
    if (/^[a-z]+:/i.test(src)) continue;
    backend.call<{ dataUrl: string }>('book.asset', { language, path: resolve(path, src) })
      .then((a) => (img.src = a.dataUrl))
      .catch(() => img.remove());
  }
  for (const a of page.querySelectorAll('a')) {
    const href = a.getAttribute('href') ?? '';
    a.addEventListener('click', (e) => {
      e.preventDefault();
      if (/^https?:/i.test(href)) openUrl(href);
      else if (href.startsWith('#')) page.querySelector(href)?.scrollIntoView();
      else {
        const [file, hash] = href.split('#');
        if (file.endsWith('.md')) showChapter(resolve(path, file)).then(() => hash && page.querySelector(`#${CSS.escape(hash)}`)?.scrollIntoView());
        else openUrl(new URL(resolve(path, file), ONLINE).toString());
      }
    });
  }
  // Previous / next chapter.
  const list = chapters();
  const i = list.findIndex((c) => c.path === path);
  const prev = list[i - 1];
  const next = list[i + 1];
  page.appendChild(h('div.book-nav', {},
    prev ? h('button.btn', { onclick: () => showChapter(prev.path) }, icon(language === 'ar' ? 'arrow-right' : 'arrow-left'), prev.title) : h('span'),
    next ? h('button.btn.primary', { onclick: () => showChapter(next.path) }, next.title, icon(language === 'ar' ? 'arrow-left' : 'arrow-right')) : h('span')));
  scroller.scrollTop = 0;
}

function renderMissing() {
  sections = [];
  toc.replaceChildren();
  languageSwitch.replaceChildren();
  bookElement.setAttribute('dir', arabic ? 'rtl' : 'ltr');
  page.replaceChildren(
    h('h1', {}, t('Doc Book')),
    h('p', {}, t('The Rust++ on this computer doesn’t include the Doc Book yet. Update Rust++ to read it here, matching your version of the language.')),
    h('div.book-nav', {},
      h('button.btn.primary', { onclick: () => runCommand('rustpp.install') }, icon('cloud-download'), t('Update Rust++')),
      h('button.btn', { onclick: () => openUrl(arabic ? `${ONLINE}ar/` : ONLINE) }, icon('link-external'), t('Read online'))));
}

/** Opens the Doc Book tab (at `chapter`, or where it was). */
export async function openBook(chapter?: string) {
  workspace.openPage(BOOK_TAB, t('Doc Book'), 'book');
  if (!info) {
    info = await backend.call<{ root: string | null; languages: string[] }>('book.info').catch(() => ({ root: null, languages: [] }));
    if (!info.languages.includes(language)) language = info.languages.includes('en') ? 'en' : info.languages[0] ?? language;
  }
  if (!info.root) {
    renderMissing();
    return;
  }
  try {
    if (!sections.length) await loadSummary();
    await showChapter(chapter ?? (current || chapters()[0]?.path || 'introduction.md'));
  } catch (err) {
    page.replaceChildren(h('p', {}, errorText(err)));
  }
}

/** Forget the book (after Rust++ was installed or updated). */
export function reloadBook() {
  info = null;
  sections = [];
  if (workspace.isPage(BOOK_TAB)) openBook();
}

export function setupBook(host: HTMLElement) {
  host.appendChild(bookElement);
  workspace.onActive.on((path) => bookElement.classList.toggle('hidden', path !== BOOK_TAB));
}
