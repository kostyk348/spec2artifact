<div align="center">

# spec2artifact

### The LLM writes the spec. The algebra writes the artifact.

**A deterministic Rust renderer that turns a ~90-token variant spec into a full
structured artifact — data, reports, tables, code scaffolds — for 0 LLM tokens.**

[![Release](https://img.shields.io/github/v/release/kostyk348/spec2artifact?sort=semver&color=0ea5e9&label=release)](https://github.com/kostyk348/spec2artifact/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/kostyk348/spec2artifact/ci.yml?branch=master&label=CI)](https://github.com/kostyk348/spec2artifact/actions/workflows/ci.yml)
[![tests](https://img.shields.io/badge/tests-52%20passing-22c55e)](#development)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](CONTRIBUTING.md)

[English](README.md) · [Русский](README.ru.md)

</div>

---

## What is spec2artifact?

Modern practice often asks an LLM to emit the whole artifact. For *structured* artifacts (data,
configs, fixed-shape documents, scaffolds) that wastes tokens: the model spends generation on
things a program can compute for free.

`spec2artifact` splits the work:

| role | who | cost |
|---|---|---|
| **what to produce** (the spec) | the LLM | small, ~90 tokens, **constant in artifact size** |
| **how to render it** (the body) | deterministic renderer | microseconds, **0 LLM tokens** |

**Not fundamentally new.** "Separate content from presentation" (templates, DSLs, codegen),
data-to-text/NLG, JSON-schema-constrained generation and grammar-constrained decoding all exist.
What this tool packages is the *measured* split plus **variant specs**: the spec shape changes
with the task kind, and the renderer is exact by construction.

## Variant specs

| kind | what the LLM fills | what the renderer produces |
|---|---|---|
| `chart` | chart type + inline data, or an `of` source inside a bundle | a deterministic Unicode chart (bars / line / histogram / sparkline) |
| `map` | variant + seed + dimensions (+ biome names) | a seeded procedural map: terrain / dungeon / maze, with an exact legend |
| `tree` | nested labels | a Unicode box-drawing tree (├── └──) |
| `bundle` | named parts (any kinds below) + `{ref:...}` placeholders | a multi-part artifact, **self-consistent by construction** |
| `records` | n, seed, categories + name pools, ranges, distributions | n records (jsonl / csv / json) |
| `report` | title, sections, bullets | Markdown |
| `table` | columns, rows or pools | Markdown table |
| `code` | language, module, imports, signatures+docs | a code scaffold |
| `list` | title, items, checked | a ticked list |
| `dataset` | n, alphabet, length, rule | a synthetic benchmark with a KNOWN label |
| `prose` | topic + points per paragraph, tone | readable template NLG |

## Bundles: artifacts that cannot disagree with themselves

A `bundle` spec names several parts — each a full spec of any other kind — and
parts cite each other's **data** through `{ref:part=NAME,stat=STAT[,field=FIELD]}`
placeholders:

```json
{"kind": "bundle", "seed": 42, "title": "Q1 Sales Review",
 "parts": {
   "orders":  {"kind": "records", "n": 600, "seed": 7, "...": "..."},
   "summary": {"kind": "report",
     "sections": [{"heading": "Totals", "bullets": [
       "Orders analyzed: {ref:part=orders,stat=count}.",
       "Total revenue: {ref:part=orders,stat=sum,field=price}.",
       "Best-selling category: {ref:part=orders,stat=top,field=category}."]}]}
 }}
```

The renderer topologically orders the parts by their references (a reference
cycle is a validation error, reported as a path), renders the data parts
first, computes exact statistics over the generated rows — `count`, `sum`,
`min`, `max`, `mean`, `top` (most frequent value), `unique` — and substitutes
them into the dependent parts. **The numbers in the prose are computed from
the data, so prose and data cannot disagree.** Hallucinated inconsistency is
impossible by construction. Example: `examples/bundle_report.{json,md}` — a
~160-token spec renders a 600-order dataset plus a summary that cites its
exact totals (8.8k output tokens, 0 of them LLM-written).

## For games, for code, for everything

**`map`** — seeded procedural worlds in game artifacts. Three variants, all
locked to the spec's seed: `terrain` (two-octave value noise classified into
five biomes), `dungeon` (non-overlapping rooms joined by L corridors) and
`maze` (a *perfect* maze — every cell reachable, exactly one path between any
two). Each body ends with a legend whose counts are computed from the
rendered grid, so the picture and its legend cannot disagree:

```
█████████████████████
█         █         █
█████████ █████ █ █ █
█       █     █ █ █ █      █ wall — perfect maze: every cell reachable,
█ ███████████ ███ █ █      single component, 79 open cells
```

**`tree`** — project layouts, org charts and tech trees drawn in exact
box-drawing characters (`├── └── │`), nested labels in, deterministic art out.

**`chart: line`** — time series join bars, histograms and sparklines:
Bresenham segments with `●` markers, from inline `[x, y]` pairs or another
part's rows.

Two showcase bundles prove the range:

- `examples/game_world.json` — a dungeon starter kit: a seeded catacomb map,
  400 weighted loot rolls, a rarity pivot, an economy sheet whose figures are
  expressions over the same rows, and two charts. **649 spec tokens → 14 754
  artifact tokens (95.6% written by nobody).**
- `examples/project_skeleton.json` — a kickoff artifact for a new repo:
  repository tree, the core module's API, a milestone roadmap, a velocity
  line chart and a standup template.

## The algebra: `{expr:...}` — the artifact computes itself

Every string in any artifact may embed expressions:

```
"Revenue incl. VAT: {expr:ref('orders','sum','price') * 1.2}."
"Target gap:      {expr:2 * ref('orders','sum','price') - ref('orders','sum','price')}."
"List price:      {expr:min(99 * 3 * 0.9, 250)}."
```

A safe, deterministic, hand-rolled expression language — arithmetic with the
usual precedence, comparisons, `&&`/`||`, `if(c,a,b)`, `min/max/abs/floor/
ceil/round/sqrt/pow/clamp`, and exactly one data source: `ref(part, stat[,
field])`, which reads the exact statistics computed over another part's
generated rows. No eval, no network, no dependencies. The LLM writes intent;
the algebra does the arithmetic — **in every kind, standalone too**.

## Charts and rollup tables computed from the bundle's own data

Two data-driven kinds close the loop — pictures and pivots that cite the same
rows the prose cites:

- **`chart`** — `bars`, `histogram`, `sparkline`, rendered in Unicode block
  characters. Inline `"data": [[label, value], ...]`, or — inside a bundle —
  `"of": {"part": "orders", "by": "category", "stat": "sum", "field":
  "price"}` computed over another part's generated rows.
- **`table.from`** — a rollup table: `{"kind": "table", "from": {"part":
  "orders", "by": "category", "cols": {"orders": "count", "revenue":
  "sum:price", "avg check": "mean:price", "best seller": "top:name"}},
  "sort": "value", "sort_by": "revenue"}`.

One spec renders a dataset, prose that cites its exact totals and derived
quantities, a pivot table and charts of the same data — **nothing can
disagree with anything**:

```console
$ spec2artifact tokens examples/algebra_demo.json
spec ≈356 tokens | artifact ≈9346 tokens | LLM wrote 96.2% fewer tokens
```

Example: `examples/algebra_demo.{json,md}`.

## Distributions in records

`records` goes beyond uniform ranges, still from one `seed`:

- `category_weights`: `{"tools": 4, "food": 3}` — weighted categorical mix
  instead of round-robin;
- `price_dist`: `uniform` (default) | `normal` (`price_mean`, `price_sd`) |
  `lognormal` (`price_mean` is the true mean);
- `sum_to`: the price field is split so it sums to EXACTLY this total
  (budgets, baskets) — the shape comes from the distribution, the residual
  is distributed in integer cents.

## Measured (live, one subagent = one LLM call)

Generating a 60-record catalogue:

| path | time | output tokens |
|---|---|---|
| LLM writes the whole catalogue | 18.0 s | ~1054 |
| LLM writes the spec, the tool renders | 8.1 s | ~90 + render |

**91% fewer tokens, 2.2× faster** (the LLM-call overhead dominates at this size).
Because the spec is constant in `n`, the saving grows. Renderer speed, measured on the Rust
binary (single process call, release build, includes process startup):

```
n=60     ~0.9 ms    ~960 tokens the LLM never wrote
n=600    ~2.0 ms    ~9726
n=6000   ~13.7 ms   ~98788
n=60000  ~128 ms    ~988k
```

## Use

```bash
spec2artifact kinds
spec2artifact template records     # the VARIANT spec skeleton to hand to the LLM
spec2artifact validate spec.json
spec2artifact render spec.json -o out.jsonl
```

As a Rust library:

```rust
use spec2artifact::{spec, render};

let mut s = spec::spec_template("records").unwrap();
s["n"] = 1000.into();
let artifact = render::render(&s).unwrap();
assert_eq!(artifact.lines().count(), 1000);
```

As an MCP tool: `spec2artifact mcp` (stdio JSON-RPC; tools: `spec_kinds`, `spec_template`,
`spec_validate`, `spec_render`). The tool manifest is frozen — `spec2artifact fingerprint`
prints its stable sha256 so agent prompt-cache drift is detectable:

```bash
$ spec2artifact fingerprint
tools=4 sha256=…
```

## Install

```bash
cargo install --path .          # from a checkout
# or grab the prebuilt binary from Releases
```

No dependencies beyond a Rust toolchain (1.75+); the binary is a single ~1 MB static file.

## Determinism

Same spec (including `seed`) → byte-identical artifact, on any machine, any release.
The RNG is SplitMix64 seeded directly from the spec; the stream is platform-independent.
Note: seeds are **not** compatible with the retired Python implementation (Mersenne Twister) —
same spec, different (but equally deterministic) stream.

## Development

```bash
cargo fmt --check                          # formatting
cargo clippy --all-targets -- -D warnings  # lints, zero tolerance
cargo test                                 # 52 tests: renderers, validation, CLI, MCP over stdio
cargo build --release                      # the binary
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the PR checklist and
[CHANGELOG.md](CHANGELOG.md) for release history.

## Caveats

* The output is only as rich as the spec; open-ended inventiveness lives in the (small) spec.
* Best for **structured** artifacts. For free prose the spec must be rich, which eats tokens.
* Deterministic: same spec (incl. `seed`) → same artifact.
