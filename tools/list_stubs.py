#!/usr/bin/env python3
import os, re, sys, json

ROOT = os.path.dirname(os.path.abspath(__file__))
PROJ = os.path.abspath(os.path.join(ROOT, ".."))
SRC = os.path.join(PROJ, "src")

patterns = [
    r"\btodo!\s*\(",
    r"\bunimplemented!\s*\(",
    r"//\s*TODO\b",
    r"/\*\s*TODO.*?\*/",
    r"\bpanic!\s*\(\s*\"unimplemented",
    r"//\s*STUB\b",
    r"/\*\s*STUB.*?\*/",
]

regexes = [re.compile(p, re.IGNORECASE | re.DOTALL) for p in patterns]

def rel(p): 
    rp = p.replace(PROJ + os.sep, "")
    return rp

matches = {}
for root, _, files in os.walk(SRC):
    for f in files:
        if not f.endswith(".rs"): 
            continue
        path = os.path.join(root, f)
        try:
            txt = open(path, "r", encoding="utf-8", errors="ignore").read()
        except Exception:
            continue
        total = 0
        per = []
        for rx in regexes:
            for m in rx.finditer(txt):
                total += 1
                per.append((rx.pattern, m.start()))
        lines = [ln for ln in txt.splitlines() if ln.strip() and not ln.strip().startswith("//")]
        if total == 0 and len(lines) <= 10:
            total = 1
            per.append(("HEURISTIC_MINIMAL_FILE", 0))
        if total > 0:
            matches[path] = per

out_path = os.path.join(PROJ, "STUBS_REPORT.md")
with open(out_path, "w", encoding="utf-8") as f:
    f.write("# Stub Tracker\n\n")
    f.write("This file was auto-generated to help you finish the Rust port.\n\n")
    f.write("## How this list was built\n")
    f.write("- Marked files containing `todo!()`, `unimplemented!()`, `panic(\"unimplemented\")`, or comments `TODO`/`STUB`.\n")
    f.write("- Also flagged very minimal files (≤10 non-empty, non-comment lines) as likely stubs.\n\n")
    f.write("## Actionable checklist\n")
    for path, per in sorted(matches.items(), key=lambda kv: kv[0]):
        markers = sorted(set(m[0] for m in per))
        f.write(f"- [ ] `{rel(path)}` — markers: {', '.join(markers)}\n")
    f.write(f"\n**Total suspected stub files: {len(matches)}**\n")
print(f"Updated {out_path} with {len(matches)} suspected stub files.")
