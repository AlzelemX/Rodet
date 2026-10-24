#!/usr/bin/env python3
"""RUST++: Arabic next to the English in the standard library's documentation.

Every documented item in `library/{core,alloc,std}` keeps its English doc
comment; the Arabic goes right under it, in the same comment, inside a block
that tools can find:

    /// Returns the number of elements in the vector.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// يرجع عدد العناصر في المتجه.
    ///
    /// </div>

rustdoc shows both languages; rust++-analyzer shows the one you chose
(`rustpp.language`), and falls back to English where there is no Arabic.

Commands:
    coverage                         how much is translated, per crate and module
    batch FILE [path] [--limit N] [--all]
                                     write summaries still missing Arabic to FILE
                                     (`--all`: whole doc comments, not just the
                                     first paragraph), most-used modules first
    apply FILE ARABIC                put the Arabic in ARABIC (lines `N| text`,
                                     `\\n` for new lines) into the doc comments
                                     listed in FILE
    full FILE path [--limit N]       write the prose of whole doc comments under
                                     `path` (code examples and `# Examples`
                                     sections left out: the editor shows the
                                     English ones) for a full translation, also
                                     for items that only have an Arabic summary
    apply-full FILE ARABIC           put full translations (blocks that start
                                     with a line `@@ N`) in place of the Arabic
                                     the doc comments in FILE have now
"""

import os
import re
import sys
from collections import Counter, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
LIBRARY = os.path.join(ROOT, "library")
CRATES = ["core", "alloc", "std"]
OPEN = '<div lang="ar" dir="rtl">'
CLOSE = "</div>"
WIDTH = 100

# Modules people meet first get translated first.
PRIORITY = [
    "std/src/io", "alloc/src/vec", "alloc/src/string.rs", "alloc/src/str.rs", "core/src/option.rs",
    "core/src/result.rs", "std/src/collections", "alloc/src/collections", "core/src/iter",
    "core/src/str", "core/src/fmt", "std/src/fs.rs", "std/src/env.rs", "std/src/process.rs",
    "std/src/thread", "std/src/sync", "alloc/src/boxed.rs", "alloc/src/rc.rs", "alloc/src/sync.rs",
    "core/src/num", "core/src/slice", "core/src/cmp.rs", "core/src/clone.rs", "core/src/convert",
    "core/src/ops", "core/src/macros", "std/src/macros.rs", "alloc/src/macros.rs", "core/src/char",
    "core/src/default.rs", "core/src/marker.rs", "core/src/mem", "std/src/path.rs", "std/src/time.rs",
    "std/src/net", "core/src/time.rs", "core/src/cell.rs", "core/src/hash", "std/src/error.rs",
    "core/src/error.rs", "alloc/src/fmt.rs", "alloc/src/borrow.rs", "core/src/borrow.rs",
    "std/src/lib.rs", "core/src/lib.rs", "alloc/src/lib.rs", "core/src/primitive_docs.rs",
    "std/src/keyword_docs.rs",
]

ITEM = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:default\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?"
    r"(?:extern\s+\"[^\"]*\"\s+)?"
    r"(fn|struct|enum|trait|mod|type|static|const|union|macro_rules!|macro|impl)\b\s*(\w*)"
)
FIELD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*[:(,{]?")


def rust_files():
    for crate in CRATES:
        base = os.path.join(LIBRARY, crate, "src")
        for dirpath, _dirs, files in os.walk(base):
            for name in sorted(files):
                if name.endswith(".rs"):
                    yield os.path.join(dirpath, name)


def rel(path):
    return os.path.relpath(path, LIBRARY)


def priority(path):
    r = rel(path)
    for i, p in enumerate(PRIORITY):
        if r.startswith(p):
            return i
    return len(PRIORITY)


def skip_attr(lines, k):
    """Index just after the attribute starting at line `k` (it may span lines)."""
    depth = 0
    n = len(lines)
    while k < n:
        line = re.sub(r'"(\\.|[^"\\])*"', '""', lines[k])
        depth += line.count("[") + line.count("(") - line.count("]") - line.count(")")
        k += 1
        if depth <= 0:
            break
    return k


def doc_blocks(lines):
    """(start, end, kind, indent) for each run of `///` or `//!` lines.

    Attributes, blank lines and plain comments between doc lines are part of
    the run, and so are `#[doc = ...]` attributes right after it (the comment
    goes on in them), so `end` is past the whole comment."""
    i = 0
    n = len(lines)
    while i < n:
        m = re.match(r"^(\s*)(///|//!)( ?)(.*)$", lines[i])
        if not m or lines[i].lstrip().startswith("////"):
            i += 1
            continue
        indent, marker = m.group(1), m.group(2)
        j = i
        while j < n:
            m2 = re.match(r"^(\s*)(///|//!)", lines[j])
            if not m2 or m2.group(2) != marker or lines[j].lstrip().startswith("////"):
                k = j
                last_doc = None
                while k < n:
                    t = lines[k].strip()
                    if t.startswith(("#[", "#![")):
                        end = skip_attr(lines, k)
                        # Only attributes of the same kind continue the comment:
                        # `#![doc = ...]` after `//!`, `#[doc = ...]` after `///`.
                        bang = "!" if marker == "//!" else ""
                        if re.match(rf"#{bang}\[\s*doc\s*=", t) or re.match(rf"#{bang}\[\s*cfg_attr\(.*\bdoc\s*=", t):
                            last_doc = end
                        k = end
                    elif not t or (t.startswith("//") and not t.startswith(("///", "//!"))):
                        k += 1
                    else:
                        break
                if k > j and k < n and lines[k].lstrip().startswith(marker) and not lines[k].lstrip().startswith("////"):
                    j = k
                    continue
                if last_doc is not None:
                    j = last_doc
                break
            j += 1
        yield i, j, marker, indent
        i = j


def block_text(lines, start, end, marker):
    out = []
    for line in lines[start:end]:
        if not line.lstrip().startswith(marker):
            continue
        s = line.lstrip()[len(marker):]
        out.append(s[1:] if s.startswith(" ") else s)
    return out


def has_arabic(text_lines):
    return any(OPEN in t for t in text_lines)


def english_part(text_lines):
    """The doc comment without any Arabic block."""
    out, skip = [], False
    for t in text_lines:
        if OPEN in t:
            skip = True
            continue
        if skip and t.strip() == CLOSE:
            skip = False
            continue
        if not skip:
            out.append(t)
    while out and not out[-1].strip():
        out.pop()
    return out


def prose(text_lines):
    """The parts of a doc comment worth translating in full: no code blocks, no
    `# Examples` sections (only code and "Basic usage:"), no link definitions."""
    out = []
    fence = False
    skip_level = None
    for t in text_lines:
        st = t.strip()
        if st.startswith("```"):
            fence = not fence
            continue
        if fence:
            continue
        m = re.match(r"^(#+)\s+(.*)$", st)
        if m:
            level = len(m.group(1))
            if skip_level is not None and level <= skip_level:
                skip_level = None
            if m.group(2).strip() in ("Examples", "Example"):
                skip_level = level
                continue
        if skip_level is not None:
            continue
        if re.match(r"^\[[^\]]+\]:\s*\S+", st):
            continue
        out.append(t)
    # Squeeze blank lines.
    text = "\n".join(out)
    text = re.sub(r"\n\s*\n(\s*\n)+", "\n\n", text).strip()
    return text


def summary(text_lines):
    """The first paragraph, joined into one line."""
    out = []
    fence = False
    for t in text_lines:
        if t.strip().startswith("```"):
            break
        if not t.strip():
            if out:
                break
            continue
        if t.lstrip().startswith("#") and not out:
            # A heading first (module docs): use it, unless it is a section
            # like "Panics" that only makes sense with what follows.
            heading = t.lstrip("# ").strip()
            if heading in ("Panics", "Safety", "Examples", "Errors", "Example", "Notes", "Note", "See Also", "See also"):
                return ""
            out.append(heading)
            break
        out.append(t.strip())
    text = " ".join(out)
    # A paragraph of link definitions only (`[a]: b [c]: d`) says nothing.
    if re.fullmatch(r"(\s*\[[^\]]+\]:\s*\S+)+\s*", text):
        return ""
    return text


def item_after(lines, end):
    """What the doc comment documents: (kind, name, line index, hidden)."""
    k = end
    depth = 0
    hidden = False
    while k < len(lines) and k < end + 40:
        s = lines[k].strip()
        if not s or s.startswith("//"):
            k += 1
            continue
        if s.startswith("#[") or s.startswith("#!") or depth:
            hidden |= "doc(hidden)" in s
            depth += s.count("[") - s.count("]")
            k += 1
            continue
        m = ITEM.match(lines[k])
        if m:
            return m.group(1).rstrip("!"), m.group(2) or "", k, hidden
        m = FIELD.match(lines[k])
        if m:
            return "field", m.group(1), k, hidden
        return "other", "", k, hidden
    return "other", "", end, hidden


def is_public(lines, kind, at, hidden):
    """Whether users of the standard library can see the item."""
    if hidden:
        return False
    line = lines[at] if at < len(lines) else ""
    if re.match(r"^\s*pub\s", line) or re.match(r"^\s*(#\[[^\]]*\]\s*)*macro_rules!", line):
        return True
    if re.match(r"^\s*pub\(", line):
        return False
    if kind in ("impl", "macro"):
        return True
    # Items of a trait, or of a trait impl, are as public as the trait.
    indent = len(line) - len(line.lstrip())
    if indent == 0:
        return False
    for k in range(at - 1, max(-1, at - 3000), -1):
        prev = lines[k]
        if not prev.strip() or prev.lstrip().startswith(("//", "#")):
            continue
        pi = len(prev) - len(prev.lstrip())
        if pi < indent:
            ps = prev.strip()
            if re.match(r"^(pub\s+)?(unsafe\s+)?(auto\s+)?trait\s", ps):
                return ps.startswith("pub")
            if re.match(r"^(unsafe\s+)?impl\b", ps) or (ps.startswith("impl") and " for " in ps):
                return " for " in ps or kind == "variant"
            if re.match(r"^(pub\s+)?enum\s", ps):
                return ps.startswith("pub")
            return False
    return False


def scan():
    """Every doc comment: dict with path, start, end, marker, indent, english, arabic?"""
    for path in rust_files():
        with open(path, encoding="utf-8") as f:
            lines = f.read().split("\n")
        for start, end, marker, indent in doc_blocks(lines):
            text = block_text(lines, start, end, marker)
            if marker == "//!":
                kind, name, public = "module", "", True
            else:
                kind, name, at, hidden = item_after(lines, end)
                public = is_public(lines, kind, at, hidden)
            eng = english_part(text)
            if not any(t.strip() for t in eng):
                continue
            # Docs with no English words (e.g. just `U+0041`) need no translation.
            if not re.search(r"[A-Za-z]{3,}", " ".join(eng)):
                continue
            yield {
                "path": path,
                "start": start,
                "end": end,
                "marker": marker,
                "indent": indent,
                "english": eng,
                "arabic": has_arabic(text),
                "kind": kind,
                "name": name,
                "public": public,
            }


def escape(s):
    return s.replace("\\", "\\\\").replace("\n", "\\n")


def unescape(s):
    return re.sub(r"\\(.)", lambda m: "\n" if m.group(1) == "n" else m.group(1), s)


def wrap(text, width):
    """Wrap a paragraph at spaces, never inside `code`."""
    words = re.findall(r"`[^`]*`[^\s]*|\S+", text)
    lines, cur = [], ""
    for w in words:
        if cur and len(cur) + 1 + len(w) > width:
            lines.append(cur)
            cur = w
        else:
            cur = f"{cur} {w}" if cur else w
    if cur:
        lines.append(cur)
    return lines


def arabic_block(arabic, indent, marker):
    prefix = f"{indent}{marker}"
    width = max(40, WIDTH - len(prefix) - 1)
    body = []
    in_code = False
    for line in arabic.split("\n"):
        if line.strip().startswith("```"):
            in_code = not in_code
            body.append(line)
        elif in_code or not line.strip() or line.lstrip().startswith(("-", "*", "#", "|", ">")) or re.match(r"^\s*\d+\.", line):
            body.append(line)
        else:
            # Keep the indent of a list item's later lines.
            lead = line[: len(line) - len(line.lstrip())]
            body.extend(lead + w for w in wrap(line.strip(), width - len(lead)))
    out = [f"{prefix}", f"{prefix} {OPEN}", f"{prefix}"]
    out += [f"{prefix} {b}" if b else prefix for b in body]
    out += [f"{prefix}", f"{prefix} {CLOSE}"]
    return out


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    cmd = sys.argv[1]
    if cmd == "coverage":
        per = defaultdict(Counter)
        for d in scan():
            if not d["public"]:
                continue
            parts = rel(d["path"]).split(os.sep)
            key = "/".join(parts[:3]) if len(parts) > 3 else "/".join(parts[:2])
            per[key]["all"] += 1
            per[key]["ar"] += d["arabic"]
        total = sum(c["all"] for c in per.values())
        done = sum(c["ar"] for c in per.values())
        print(f"documented public items: {done}/{total} have Arabic ({100 * done // max(total, 1)}%)")
        for crate in CRATES:
            t = sum(c["all"] for k, c in per.items() if k.startswith(crate + "/"))
            d = sum(c["ar"] for k, c in per.items() if k.startswith(crate + "/"))
            print(f"  {crate:6} {d:6}/{t}")
        if os.environ.get("DETAIL"):
            for k in sorted(per, key=lambda k: -per[k]["all"]):
                print(f"    {k:40} {per[k]['ar']:5}/{per[k]['all']}")
    elif cmd == "batch":
        out = sys.argv[2]
        args = sys.argv[3:]
        limit = 10**9
        if "--limit" in args:
            limit = int(args[args.index("--limit") + 1])
        whole = "--all" in args
        where = [a for a in args if not a.startswith("--") and not a.isdigit()]
        items = [d for d in scan() if not d["arabic"] and (d["public"] or "--private" in args)]
        if where:
            items = [d for d in items if rel(d["path"]).startswith(where[0])]
        items.sort(key=lambda d: (priority(d["path"]), rel(d["path"]), d["start"]))
        seen = set()
        n = 0
        with open(out, "w", encoding="utf-8") as f:
            for d in items:
                text = "\n".join(d["english"]) if whole else summary(d["english"])
                if not text or text in seen:
                    continue
                seen.add(text)
                n += 1
                f.write(f"{n}|{d['kind']} {d['name']}|{escape(text)}\n")
                if n >= limit:
                    break
        print(f"{n} texts in {out}")
    elif cmd == "apply":
        batch = {}
        whole = False
        for line in open(sys.argv[2], encoding="utf-8"):
            n, _what, en = line.rstrip("\n").split("|", 2)
            batch[int(n)] = unescape(en)
        arabic = {}
        for line in open(sys.argv[3], encoding="utf-8"):
            line = line.rstrip("\n")
            if "|" not in line:
                continue
            n, ar = line.split("|", 1)
            if n.strip().isdigit() and ar.strip():
                arabic[batch[int(n)]] = unescape(ar.strip())
        # A text that is a whole doc comment, or a summary.
        by_path = defaultdict(list)
        for d in scan():
            if d["arabic"]:
                continue
            full = "\n".join(d["english"])
            key = full if full in arabic else summary(d["english"])
            if key in arabic:
                by_path[d["path"]].append((d, arabic[key]))
        changed = 0
        for path, items in by_path.items():
            with open(path, encoding="utf-8") as f:
                lines = f.read().split("\n")
            for d, ar in sorted(items, key=lambda x: -x[0]["end"]):
                lines[d["end"] : d["end"]] = arabic_block(ar, d["indent"], d["marker"])
                changed += 1
            with open(path, "w", encoding="utf-8") as f:
                f.write("\n".join(lines))
        print(f"added Arabic to {changed} doc comments in {len(by_path)} files")
    elif cmd == "full":
        out = sys.argv[2]
        where = sys.argv[3]
        args = sys.argv[4:]
        limit = int(args[args.index("--limit") + 1]) if "--limit" in args else 10**9
        n = 0
        with open(out, "w", encoding="utf-8") as f:
            for d in scan():
                if not d["public"] or not rel(d["path"]).startswith(where):
                    continue
                text = prose(d["english"])
                if not text:
                    continue
                n += 1
                f.write(f"@@ {n} {d['kind']} {d['name']} {rel(d['path'])}:{d['start'] + 1}\n{text}\n")
                if n >= limit:
                    break
        print(f"{n} doc comments in {out}")
    elif cmd == "apply-full":
        def blocks(path):
            res, cur, head = {}, [], None
            for line in open(path, encoding="utf-8"):
                line = line.rstrip("\n")
                if line.startswith("@@ "):
                    if head is not None:
                        res[head] = "\n".join(cur).strip()
                    head, cur = int(line.split()[1]), []
                else:
                    cur.append(line)
            if head is not None:
                res[head] = "\n".join(cur).strip()
            return res

        english = blocks(sys.argv[2])
        arabic = {english[n]: a for n, a in blocks(sys.argv[3]).items() if a and n in english}
        by_path = defaultdict(list)
        for d in scan():
            text = prose(d["english"])
            if text in arabic:
                by_path[d["path"]].append((d, arabic[text]))
        changed = 0
        for path, items in by_path.items():
            with open(path, encoding="utf-8") as f:
                lines = f.read().split("\n")
            for d, ar in sorted(items, key=lambda x: -x[0]["end"]):
                start, end = d["start"], d["end"]
                # Take out the Arabic the comment has now.
                for k in range(end - 1, start - 1, -1):
                    if lines[k].strip() == f"{d['marker']} {OPEN}":
                        c = k
                        while lines[c].strip() != f"{d['marker']} {CLOSE}":
                            c += 1
                        b0 = k - 1 if lines[k - 1].strip() == d["marker"] else k
                        del lines[b0 : c + 1]
                        end -= c + 1 - b0
                lines[end:end] = arabic_block(ar, d["indent"], d["marker"])
                changed += 1
            with open(path, "w", encoding="utf-8") as f:
                f.write("\n".join(lines))
        print(f"put full Arabic into {changed} doc comments in {len(by_path)} files")
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
