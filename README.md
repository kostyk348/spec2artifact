# spec2artifact

**The LLM writes a small, task-variant SPEC; the algebra renders the whole artifact.**
Fewer tokens, lower latency, exact output.

## The idea (and what is honestly *not* new)

Modern practice often asks an LLM to emit the whole artifact. For *structured* artifacts (data,
configs, fixed-shape documents, scaffolds) that wastes tokens: the model spends generation on
things a program can compute for free.

`spec2artifact` splits the work:

| role | who | cost |
|---|---|---|
| **what to produce** (the spec) | the LLM | small, ~90 tokens, **constant in artifact size** |
| **how to render it** (the body) | deterministic renderer | ~microseconds, **0 LLM tokens** |

**Not fundamentally new.** "Separate content from presentation" (templates, DSLs, codegen),
data-to-text/NLG, JSON-schema-constrained generation and grammar-constrained decoding all exist.
What this tool packages is the *measured* split plus **variant specs**: the spec shape changes
with the task kind, and the renderer is exact by construction.

## Variant specs

| kind | what the LLM fills | what the renderer produces |
|---|---|---|
| `records` | n, seed, categories + name pools, ranges | n records (jsonl / csv / json) |
| `report` | title, sections, bullets | Markdown |
| `table` | columns, rows or pools | Markdown table |
| `code` | language, module, imports, signatures+docs | a code scaffold |
| `list` | title, items, checked | a ticked list |

## Measured (live, one subagent = one LLM call)

Generating a 60-record catalogue:

| path | time | output tokens |
|---|---|---|
| LLM writes the whole catalogue | 18.0 s | ~1054 |
| LLM writes the spec, the tool renders | 8.1 s | ~90 + 0.07 ms render |

**91% fewer tokens, 2.2× faster** (the LLM-call overhead dominates at this size).
Because the spec is constant in `n`, the saving grows:

```
n=60    render 0.09 ms   ~960 tokens the LLM never wrote
n=600   render 0.46 ms   ~9726
n=6000  render 3.92 ms   ~98788
```

## Use

```bash
spec2artifact kinds
spec2artifact template records     # the VARIANT spec skeleton to hand to the LLM
spec2artifact validate spec.json
spec2artifact render spec.json -o out.jsonl
```

As a library:

```python
from spec2artifact import spec as S, render
s = S.spec_template("records"); s["n"] = 1000
open("catalog.jsonl", "w").write(render.render(s))
```

As an MCP tool: `python -m spec2artifact.mcp_server` (tools: `spec_kinds`, `spec_template`,
`spec_validate`, `spec_render`).

## Caveats

* The output is only as rich as the spec; open-ended inventiveness lives in the (small) spec.
* Best for **structured** artifacts. For free prose the spec must be rich, which eats tokens.
* Deterministic: same spec (incl. `seed`) → same artifact.

Tests: `PYTHONPATH=. python3 tests/test_render.py`
