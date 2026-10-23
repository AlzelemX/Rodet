#!/usr/bin/env python3
"""Rust++: the compiler's messages and their Arabic translations.

Every message the compiler can show is written in English in its source
(`#[diag("mismatched types")]`, `msg!("...")`, `.note("...")`, …). The Arabic
for each one lives next to them in `compiler/rustc_errors/src/i18n/ar.txt`,
keyed by the exact English text; the compiler shows it when `RUSTPP_LANG=ar`.

    python3 rustpp/i18n/compiler_messages.py coverage      # how much is translated
    python3 rustpp/i18n/compiler_messages.py missing [crate] [--limit N]
    python3 rustpp/i18n/compiler_messages.py check         # keys that no longer exist, bad placeholders
    python3 rustpp/i18n/compiler_messages.py batch FILE [crate] [--limit N]  # numbered list to translate
    python3 rustpp/i18n/compiler_messages.py merge FILE ARABIC              # ARABIC: lines `N| translation`

The coverage line is what the status board shows.
"""

import os
import re
import sys
from collections import Counter, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
COMPILER = os.path.join(ROOT, "compiler")
TABLE = os.path.join(COMPILER, "rustc_errors", "src", "i18n", "ar.txt")

# Crates whose messages users never see (internal bugs, tests, the driver's
# own debugging output) are left out of the coverage count.
SKIP_CRATES = {"rustc_macros", "rustc_fluent_macro", "rustc_codegen_gcc", "rustc_codegen_cranelift"}

ATTRIBUTE = re.compile(
    r"#\[(diag|note|help|label|warning|suggestion|multipart_suggestion|note_once|help_once)\(\s*"
)
MSG_MACRO = re.compile(r"\bmsg!\(\s*")
CODE_ERR = re.compile(r"\bE\d{4}\s*,\s*")
METHOD = re.compile(
    r"\.(note|help|warn|span_label|span_note|span_help|span_warn|span_suggestion\w*|"
    r"multipart_suggestion\w*|tool_only_span_suggestion|with_note|with_help|with_span_label|"
    r"with_span_note|with_span_help|primary_message|struct_err|struct_warn|struct_span_err|"
    r"struct_span_warn|span_err|note_once|help_once|span_note_once)\("
)

# The table ends with words and phrases that go into other messages' holes.
FRAGMENTS = "# Words and phrases the compiler puts into the holes of other messages."

LOOSE_LITERAL = re.compile(r'(?m)(?:^[ \t]*|=>[ \t]*|\{[ \t]*)"')
LOOSE_FORMAT = re.compile(r"\bformat!\(\s*")


def looks_like_message(s):
    """English prose (not code, a path or a symbol name) built with `format!`."""
    plain = re.sub(r"`[^`]*`|\{[^}]*\}", " ", s)
    words = re.findall(r"\b[a-z]{2,}\b", plain)
    return len(words) >= 2 and not re.search(r"[;=]\s*$|::|->|\bfn\b|#\[|\bimpl\b|\(\)$", plain)


def read_literal(text, i):
    """The Rust string literal starting at `text[i]` (after whitespace), or None."""
    while i < len(text) and text[i] in " \t\r\n":
        i += 1
    m = re.match(r'r(#*)"', text[i:])
    if m:
        hashes = m.group(1)
        start = i + len(m.group(0))
        end = text.find('"' + hashes, start)
        return (text[start:end], end + 1 + len(hashes)) if end >= 0 else None
    if i >= len(text) or text[i] != '"':
        return None
    out = []
    j = i + 1
    while j < len(text):
        c = text[j]
        if c == "\\":
            n = text[j + 1]
            if n == "n":
                out.append("\n")
            elif n == "t":
                out.append("\t")
            elif n in "\"'\\0":
                out.append({"0": "\0"}.get(n, n))
            elif n == "\n":
                # Line continuation: skip the newline and the next line's indentation.
                j += 2
                while j < len(text) and text[j] in " \t":
                    j += 1
                continue
            elif n == "u":
                end = text.index("}", j)
                out.append(chr(int(text[j + 3 : end], 16)))
                j = end + 1
                continue
            else:
                out.append(n)
            j += 2
            continue
        if c == '"':
            return "".join(out), j + 1
        out.append(c)
        j += 1
    return None


def call_args(text, i):
    """The text of a call's arguments, from just after `(` to its `)`."""
    depth = 1
    j = i
    in_str = False
    while j < len(text) and depth:
        c = text[j]
        if in_str:
            if c == "\\":
                j += 2
                continue
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        j += 1
    return text[i : j - 1]


def crate_of(path):
    rel = os.path.relpath(path, COMPILER)
    return rel.split(os.sep)[0]


def extract():
    """{english: (kind, {crate: count}, first location)}; kind is fluent, format or plain."""
    found = {}
    for dirpath, _dirs, files in os.walk(COMPILER):
        if "/tests" in dirpath or "/test" in dirpath:
            continue
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            crate = crate_of(path)
            if crate in SKIP_CRATES:
                continue
            text = open(path, encoding="utf-8").read()

            def add(message, kind, pos):
                if not message or not message.strip():
                    return
                line = text.count("\n", 0, pos) + 1
                entry = found.setdefault(message, [kind, Counter(), f"{os.path.relpath(path, ROOT)}:{line}"])
                entry[1][crate] += 1

            for m in ATTRIBUTE.finditer(text):
                lit = read_literal(text, m.end())
                if lit:
                    add(lit[0], "fluent", m.start())
            for m in MSG_MACRO.finditer(text):
                lit = read_literal(text, m.end())
                if lit:
                    add(lit[0], "fluent", m.start())
            for m in CODE_ERR.finditer(text):
                lit = read_literal(text, m.end())
                if lit:
                    add(lit[0], "format", m.start())
            if crate == "rustc_errors" and name == "lib.rs":
                # The summary at the end: "aborting due to 2 previous errors".
                for m in re.finditer(r'Cow::from\(\s*(format!\(\s*)?', text):
                    lit = read_literal(text, m.end())
                    if lit and lit[0][:1].isalpha() or (lit and lit[0][:1] == "{"):
                        add(lit[0], "format" if m.group(1) else "plain", m.start())
                for m in re.finditer(r'let msg\d? = format!\(\s*', text):
                    lit = read_literal(text, m.end())
                    if lit:
                        add(lit[0], "format", m.start())
            if "Diag" in text or "span_label" in text:
                # Messages built ahead of time: `let msg = format!("…");`.
                for m in LOOSE_FORMAT.finditer(text):
                    lit = read_literal(text, m.end())
                    if lit and looks_like_message(lit[0]):
                        add(lit[0], "format", m.start())
                # Messages chosen ahead of time: `let msg = if … { "…" } else { "…" };`
                # and match arms (`Kind::A => "…",`).
                for m in LOOSE_LITERAL.finditer(text) if name != "options.rs" else ():
                    lit = read_literal(text, m.end() - 1)
                    if lit and looks_like_message(lit[0]) and "{" not in lit[0]:
                        add(lit[0], "plain", m.start())
            for m in METHOD.finditer(text):
                args = call_args(text, m.end())
                fm = re.search(r'format!\(\s*', args)
                lit_m = re.search(r'(?<![\w!])(r#*)?"', args)
                if fm and (not lit_m or fm.start() < lit_m.start()):
                    lit = read_literal(args, fm.end())
                    if lit:
                        add(lit[0], "format", m.start())
                elif lit_m:
                    lit = read_literal(args, lit_m.start())
                    if lit:
                        add(lit[0], "plain", m.start())
    return found


def escape(s):
    return s.replace("\\", "\\\\").replace("\n", "\\n")


def unescape(s):
    return re.sub(r"\\(.)", lambda m: "\n" if m.group(1) == "n" else m.group(1), s)


def read_table():
    table = {}
    if not os.path.exists(TABLE):
        return table
    english = None
    for raw in open(TABLE, encoding="utf-8"):
        line = raw.rstrip("\n")
        if line.startswith("en: "):
            english = unescape(line[4:])
        elif line.startswith("ar: ") and english is not None:
            table[english] = unescape(line[4:])
            english = None
    return table


def placeholders(s):
    return sorted(set(re.findall(r"\{\$(\w+)", s)))


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "coverage"
    found = extract()
    table = read_table()
    if cmd == "coverage":
        total = len(found)
        done = sum(1 for k in found if k in table)
        uses = sum(sum(c.values()) for _, c, _ in found.values())
        uses_done = sum(sum(c.values()) for k, (_, c, _) in found.items() if k in table)
        print(f"compiler messages: {done}/{total} translated ({100 * done // max(total, 1)}%), "
              f"{uses_done}/{uses} places in the code")
        per = defaultdict(lambda: [0, 0])
        for k, (_, c, _) in found.items():
            for crate in c:
                per[crate][1] += 1
                per[crate][0] += k in table
        for crate, (d, t) in sorted(per.items(), key=lambda x: -x[1][1])[: int(os.environ.get("TOP", "40"))]:
            print(f"  {crate:32} {d:5}/{t:<5}")
    elif cmd == "missing":
        crate = next((a for a in sys.argv[2:] if not a.startswith("--")), None)
        limit = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else 10**9
        n = 0
        for k, (kind, c, loc) in sorted(found.items(), key=lambda x: x[1][2]):
            if k in table or (crate and crate not in c):
                continue
            print(f"en: {escape(k)}")
            n += 1
            if n >= limit:
                break
    elif cmd == "batch":
        out = sys.argv[2]
        crate = next((a for a in sys.argv[3:] if not a.startswith("--") and not a.isdigit()), None)
        limit = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else 10**9
        items = [k for k, (kind, c, loc) in sorted(found.items(), key=lambda x: x[1][2])
                 if k not in table and (not crate or crate in c)][:limit]
        with open(out, "w", encoding="utf-8") as f:
            for n, k in enumerate(items, 1):
                kind = found[k][0]
                f.write(f"{n}|{kind}| {escape(k)}\n")
        print(f"{len(items)} messages in {out}")
    elif cmd == "merge":
        batch = {}
        for line in open(sys.argv[2], encoding="utf-8"):
            n, kind, en = line.rstrip("\n").split("|", 2)
            batch[int(n)] = (unescape(en[1:]), kind)
        added = 0
        new = []
        if True:
            for line in open(sys.argv[3], encoding="utf-8"):
                line = line.rstrip("\n")
                if not line.strip() or "|" not in line:
                    continue
                n, ar = line.split("|", 1)
                en, kind = batch.get(int(n.strip()), (None, None))
                ar = ar.strip()
                if kind == "fluent":
                    # Fluent: lines after the first must be indented.
                    ar = re.sub(r"\\n(?=[^ \\])", r"\\n    ", ar)
                if en is None or not ar or en in table:
                    continue
                if not set(placeholders(unescape(ar))) <= set(placeholders(en)):
                    print(f"skipped, unknown holes: {n}: {escape(en)}  →  {ar}")
                    continue
                new.append(f"\nen: {escape(en)}\nar: {ar}\n")
                table[en] = ar
                added += 1
        # New messages go before the words-and-phrases section at the end.
        text = open(TABLE, encoding="utf-8").read()
        at = text.find(FRAGMENTS)
        at = len(text) if at < 0 else at
        head = text[:at].rstrip("\n") + "\n"
        tail = text[at:]
        text = head + "".join(new) + ("\n" + tail if tail else "")
        open(TABLE, "w", encoding="utf-8").write(text)
        print(f"added {added}")
    elif cmd == "check":
        bad = 0
        text = open(TABLE, encoding="utf-8").read()
        at = text.find(FRAGMENTS)
        fragments = set(re.findall(r"^en: (.*)$", text[at:], re.M)) if at >= 0 else set()
        for k, v in table.items():
            if k not in found and escape(k) not in fragments:
                print(f"no longer in the compiler: {escape(k)}")
            if not set(placeholders(v)) <= set(placeholders(k)):
                print(f"unknown placeholders: {escape(k)}  →  {escape(v)}")
                bad += 1
        sys.exit(1 if bad else 0)
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
