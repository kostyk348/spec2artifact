"""
spec2artifact.render — deterministic renderers, one per spec kind.

Pure functions: spec -> artifact text. No network, no LLM, no randomness beyond the spec's seed.
"""
from __future__ import annotations

import csv
import io
import json
import random
from typing import Callable

_RENDERERS: dict[str, Callable[[dict], str]] = {}


def renderer(kind: str):
    def deco(fn):
        _RENDERERS[kind] = fn
        return fn
    return deco


def kinds() -> list[str]:
    return sorted(_RENDERERS)


def render(spec: dict) -> str:
    kind = spec.get("kind")
    if kind not in _RENDERERS:
        raise KeyError(f"no renderer for kind {kind!r}; have {kinds()}")
    return _RENDERERS[kind](spec)


# ---------------------------------------------------------------- records ----
@renderer("records")
def _records(spec: dict) -> str:
    rng = random.Random(spec.get("seed", 0))
    cats = spec.get("categories") or ["default"]
    pools = spec.get("pools") or {"default": spec.get("names") or ["item"]}
    lo, hi = spec.get("price_range", [0, 0])
    dec = spec.get("decimals", 2)
    fields = spec.get("fields", ["id", "name", "category", "price"])
    rows = []
    for i in range(1, spec["n"] + 1):
        c = cats[(i - 1) % len(cats)]
        row = {"id": i, "name": rng.choice(pools[c]), "category": c,
               "price": round(rng.uniform(lo, hi), dec)}
        rows.append({f: row[f] for f in fields if f in row})
    fmt = spec.get("format", "jsonl")
    if fmt == "json":
        return json.dumps(rows, ensure_ascii=False, indent=1)
    if fmt == "csv":
        buf = io.StringIO(); w = csv.DictWriter(buf, fieldnames=fields)
        w.writeheader(); [w.writerow(r) for r in rows]
        return buf.getvalue().rstrip("\n")
    return "\n".join(json.dumps(r, ensure_ascii=False) for r in rows)


# ----------------------------------------------------------------- report ----
@renderer("report")
def _report(spec: dict) -> str:
    out = [f"# {spec.get('title', 'Report')}", ""]
    for s in spec.get("sections", []):
        out.append(f"## {s.get('heading', '')}")
        for b in s.get("bullets", []):
            out.append(f"- {b}")
        out.append("")
    if spec.get("footer"):
        out += ["---", spec["footer"]]
    return "\n".join(out).rstrip() + "\n"


# ------------------------------------------------------------------ table ----
@renderer("table")
def _table(spec: dict) -> str:
    cols = spec["columns"]
    rows = list(spec.get("rows", []))
    if spec.get("n") and spec.get("pools"):
        rng = random.Random(spec.get("seed", 0))
        for i in range(1, spec["n"] + 1):
            rows.append([str(i)] + [rng.choice(spec["pools"].get(c, ["-"])) for c in cols[1:]])
    out = [f"| {' | '.join(cols)} |", f"|{'|'.join('---' for _ in cols)}|"]
    for r in rows:
        out.append(f"| {' | '.join(str(c) for c in r)} |")
    head = f"# {spec['title']}\n\n" if spec.get("title") else ""
    return head + "\n".join(out) + "\n"


# ------------------------------------------------------------------- code ----
@renderer("code")
def _code(spec: dict) -> str:
    lang = spec.get("language", "python")
    out = []
    if spec.get("header"):
        out += [f"# {spec['header']}", ""] if lang == "python" else [f"// {spec['header']}", ""]
    out += list(spec.get("imports", []))
    if spec.get("imports"):
        out.append("")
    for f in spec.get("functions", []):
        args = ", ".join(f.get("args", []))
        doc = f.get("doc", "")
        if lang == "python":
            out += [f"def {f['name']}({args}):", f'    """{doc}"""', "    raise NotImplementedError", ""]
        elif lang in ("c", "cpp"):
            out += [f"/* {doc} */", f"int {f['name']}({args or 'void'}) {{", "    return 0;", "}", ""]
        elif lang == "rust":
            out += [f"/// {doc}", f"pub fn {f['name']}({args or ''}) {{", "    todo!()", "}", ""]
        else:
            out += [f"{f['name']}({args})  // {doc}", ""]
    return "\n".join(out).rstrip() + "\n"


# ------------------------------------------------------------------- list ----
@renderer("list")
def _list(spec: dict) -> str:
    done = set(spec.get("checked", []))
    out = ([f"# {spec['title']}", ""] if spec.get("title") else [])
    for i in spec.get("items", []):
        out.append(f"- [{'x' if i in done else ' '}] {i}")
    return "\n".join(out) + "\n"


# ---------------------------------------------------------------- dataset ----
@renderer("dataset")
def _dataset(spec: dict) -> str:
    """n (sequence, target) pairs for a KNOWN rule -- an exact benchmark for algebraic learners."""
    rng = random.Random(spec.get("seed", 0))
    alpha = spec["alphabet"]; L = spec["length"]; rule = spec.get("rule", {})
    t, k = rule.get("type", "mod"), rule.get("k", 2)
    rows = []
    for i in range(1, spec["n"] + 1):
        seq = [rng.choice(alpha) for _ in range(L)]
        if t == "parity":
            y = sum(alpha.index(c) for c in seq) % 2
        elif t == "mod":
            y = sum(alpha.index(c) for c in seq) % k
        elif t == "contains":
            y = int(rule.get("sym", alpha[0]) in seq)
        else:                                   # count of the first symbol, mod k
            y = sum(c == alpha[0] for c in seq) % k
        rows.append({"id": i, "sequence": "".join(seq), "target": int(y)})
    fmt = spec.get("format", "jsonl")
    if fmt == "json":
        return json.dumps(rows, ensure_ascii=False, indent=1)
    if fmt == "csv":
        buf = io.StringIO(); w = csv.DictWriter(buf, fieldnames=["id", "sequence", "target"])
        w.writeheader(); [w.writerow(r) for r in rows]
        return buf.getvalue().rstrip("\n")
    return "\n".join(json.dumps(r, ensure_ascii=False) for r in rows)


# ------------------------------------------------------------------ prose ----
# Free prose assembled from a plan. Honest scope: this is TEMPLATE NLG -- it renders the spec's
# claims into readable sentences with tone-specific connectives. It never invents a fact; the
# content is exactly what the spec supplies.
_TONES: dict[str, dict] = {
    "neutral": {"lead": ["{t} is worth a moment.", "On {t}, a few things stand out.", "Consider {t}."],
                "join": ["Also,", "Moreover,", "In addition,", "Beyond that,"],
                "close": ["That is the gist.", "This is the core of it.", "So it stands."]},
    "formal": {"lead": ["The matter of {t} warrants attention.", "With respect to {t}, several observations apply."],
               "join": ["Furthermore,", "In addition,", "Consequently,"],
               "close": ["This concludes the point.", "Such is the position."]},
    "casual": {"lead": ["So, {t} -- here's the thing.", "About {t}:", "Let's talk {t}."],
               "join": ["Plus,", "And,", "Also,"],
               "close": ["Anyway, that's it.", "That's the deal."]},
}


@renderer("prose")
def _prose(spec: dict) -> str:
    rng = random.Random(spec.get("seed", 0))
    tone = _TONES.get(spec.get("tone", "neutral"), _TONES["neutral"])
    out: list[str] = []
    if spec.get("title"):
        out += [f"# {spec['title']}", ""]
    for p in spec.get("paragraphs", []):
        sents = [rng.choice(tone["lead"]).format(t=p.get("topic", "it"))]
        for i, pt in enumerate(p.get("points", [])):
            pre = (rng.choice(tone["join"]) + " ") if i > 0 else ""
            sents.append(pre + pt.rstrip(".") + ".")
        if rng.random() < 0.6:
            sents.append(rng.choice(tone["close"]))
        out += [" ".join(sents), ""]
    if spec.get("footer"):
        out += ["---", spec["footer"]]
    return "\n".join(out).rstrip() + "\n"
