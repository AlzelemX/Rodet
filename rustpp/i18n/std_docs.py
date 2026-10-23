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


def doc_blocks(lines):
    """(start, end, kind, indent, prefix) for each run of `///` or `//!` lines."""
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
                break
            j += 1
        yield i, j, marker, indent
        i = j


def block_text(lines, start, end, marker):
    out = []
    for line in lines[start:end]:
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
            # A heading first (module docs): use it.
            out.append(t.lstrip("# ").strip())
            break
        out.append(t.strip())
    return " ".join(out)


def item_after(lines, end):
    """What the doc comment documents: (kind, name)."""
    k = end
    depth = 0
    while k < len(lines) and k < end + 40:
        s = lines[k].strip()
        if not s or s.startswith("//"):
            k += 1
            continue
        if s.startswith("#[") or s.startswith("#!") or depth:
            depth += s.count("[") - s.count("]")
            k += 1
            continue
        m = ITEM.match(lines[k])
        if m:
            return m.group(1).rstrip("!"), m.group(2) or ""
        m = FIELD.match(lines[k])
        if m:
            return "field", m.group(1)
        return "other", ""
    return "other", ""


def scan():
    """Every doc comment: dict with path, start, end, marker, indent, english, arabic?"""
    for path in rust_files():
        with open(path, encoding="utf-8") as f:
            lines = f.read().split("\n")
        for start, end, marker, indent in doc_blocks(lines):
            text = block_text(lines, start, end, marker)
            kind, name = ("module", "") if marker == "//!" else item_after(lines, end)
            eng = english_part(text)
            if not any(t.strip() for t in eng):
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
            body.extend(wrap(line, width))
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
            parts = rel(d["path"]).split(os.sep)
            key = "/".join(parts[:3]) if len(parts) > 3 else "/".join(parts[:2])
            per[key]["all"] += 1
            per[key]["ar"] += d["arabic"]
        total = sum(c["all"] for c in per.values())
        done = sum(c["ar"] for c in per.values())
        print(f"documented items: {done}/{total} have Arabic ({100 * done // max(total, 1)}%)")
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
        items = [d for d in scan() if not d["arabic"]]
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
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
