"""
spec2artifact.spec — variant specs, one per task KIND.

The LLM fills a small SPEC for the task kind; the algebra renders the artifact. The spec shape is
VARIANT: each kind declares its own skeleton + a template the LLM is asked to fill.
"""
from __future__ import annotations

# ---- the variant spec skeletons (one per task kind) -------------------------

KINDS: dict[str, dict] = {
    "records": {                      # data / catalogue / dataset rows
        "desc": "n records with fields drawn from per-category pools",
        "skeleton": {
            "kind": "records", "n": 60, "seed": 42, "format": "jsonl",
            "fields": ["id", "name", "category", "price"],
            "categories": ["tools", "toys", "books", "food"],
            "pools": {"tools": ["Hammer", "Wrench"], "toys": ["Robot", "Kite"],
                      "books": ["Atlas", "Novel"], "food": ["Bread", "Honey"]},
            "price_range": [1, 999], "decimals": 2,
        },
    },
    "report": {                       # a fixed-shape document
        "desc": "title + sections with bullets",
        "skeleton": {"kind": "report", "title": "<title>",
                     "sections": [{"heading": "<h>", "bullets": ["<b1>", "<b2>"]}],
                     "footer": ""},
    },
    "table": {                        # a markdown table
        "desc": "columns + rows (explicit or generated from pools)",
        "skeleton": {"kind": "table", "title": "<title>",
                     "columns": ["<c1>", "<c2>"],
                     "rows": [["<r1c1>", "<r1c2>"]], "n": 0},
    },
    "code": {                         # a code scaffold
        "desc": "module with imports + function signatures/docstrings",
        "skeleton": {"kind": "code", "language": "python", "module": "<name>",
                     "imports": ["import os"], "functions": [
                         {"name": "<f>", "args": ["a", "b"], "doc": "<doc>"}],
                     "header": ""},
    },
    "list": {                         # a flat checklist
        "desc": "a ticked list",
        "skeleton": {"kind": "list", "title": "<title>",
                     "items": ["<i1>", "<i2>"], "checked": []},
    },
    "dataset": {                      # synthetic sequence task with a KNOWN algebraic label
        "desc": "n (sequence, target) pairs for a known rule (parity/mod/contains)",
        "skeleton": {"kind": "dataset", "n": 200, "seed": 42, "alphabet": ["a", "b", "c"],
                     "length": 12, "rule": {"type": "mod", "k": 3}, "format": "jsonl"},
    },
    "prose": {                        # free prose assembled from a compact plan
        "desc": "readable prose from a plan (topic + points per paragraph); template NLG",
        "skeleton": {"kind": "prose", "title": "<title>", "tone": "neutral", "seed": 42,
                     "paragraphs": [{"topic": "<topic>", "points": ["<claim 1>", "<claim 2>"]}],
                     "footer": ""},
    },
}


def kinds() -> list[str]:
    return sorted(KINDS)


def spec_template(kind: str) -> dict:
    """the VARIANT spec skeleton the LLM should fill for this task kind."""
    if kind not in KINDS:
        raise KeyError(f"unknown kind {kind!r}; known: {', '.join(kinds())}")
    import copy
    return copy.deepcopy(KINDS[kind]["skeleton"])


def validate(spec: dict) -> list[str]:
    """cheap structural validation of a spec (not of the artifact)."""
    errs: list[str] = []
    if not isinstance(spec, dict):
        return ["spec must be an object"]
    kind = spec.get("kind")
    if kind not in KINDS:
        return [f"kind must be one of {kinds()}, got {kind!r}"]
    if kind == "records":
        if not isinstance(spec.get("n"), int) or spec["n"] < 0:
            errs.append("records.n must be a non-negative int")
        if "categories" in spec and "pools" in spec:
            for c in spec["categories"]:
                if c not in spec["pools"]:
                    errs.append(f"pool missing for category {c!r}")
        pr = spec.get("price_range")
        if pr is not None and (not isinstance(pr, list) or len(pr) != 2 or pr[0] > pr[1]):
            errs.append("price_range must be [lo, hi] with lo <= hi")
    if kind == "report" and not isinstance(spec.get("sections"), list):
        errs.append("report.sections must be a list")
    if kind == "table" and not isinstance(spec.get("columns"), list):
        errs.append("table.columns must be a list")
    if kind == "code" and not isinstance(spec.get("functions"), list):
        errs.append("code.functions must be a list")
    if kind == "list" and not isinstance(spec.get("items"), list):
        errs.append("list.items must be a list")
    if kind == "dataset":
        r = spec.get("rule", {})
        if r.get("type") not in ("mod", "parity", "contains", "count"):
            errs.append("dataset.rule.type must be one of mod|parity|contains|count")
        if not isinstance(spec.get("alphabet"), list) or not spec.get("alphabet"):
            errs.append("dataset.alphabet must be a non-empty list")
        if not isinstance(spec.get("length"), int) or spec["length"] < 1:
            errs.append("dataset.length must be a positive int")
    if kind == "prose":
        if spec.get("tone", "neutral") not in ("neutral", "formal", "casual"):
            errs.append("prose.tone must be neutral|formal|casual")
        if not isinstance(spec.get("paragraphs"), list):
            errs.append("prose.paragraphs must be a list")
        else:
            for p in spec["paragraphs"]:
                if not isinstance(p.get("points"), list):
                    errs.append("prose paragraph needs a points list")
    return errs
