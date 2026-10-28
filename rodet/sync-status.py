#!/usr/bin/env python3
"""Copies STATUS.md into README.md, between the STATUS markers.

Run by .github/workflows/readme-status.yml after every push that changes
STATUS.md. Run it by hand to see the result locally.
"""

import pathlib
import re

root = pathlib.Path(__file__).resolve().parent.parent
status = (root / "STATUS.md").read_text(encoding="utf-8").strip()
readme_path = root / "README.md"
readme = readme_path.read_text(encoding="utf-8")

# Drop the "edit this file" comment: it is meant for STATUS.md only.
status = re.sub(r"\A<!--.*?-->\s*", "", status, flags=re.S)

start, end = "<!-- STATUS:START -->", "<!-- STATUS:END -->"
if start not in readme or end not in readme:
    raise SystemExit(f"README.md has no {start} ... {end} markers")

before = readme[: readme.index(start) + len(start)]
after = readme[readme.index(end) :]
new = (
    f"{before}\n<!-- Generated from STATUS.md: edit that file, not this table. -->\n\n"
    f"{status}\n\n{after}"
)

if new != readme:
    readme_path.write_text(new, encoding="utf-8")
    print("README.md updated")
else:
    print("README.md already up to date")
