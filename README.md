<div align="center">

# spec2artifact

### The LLM writes the spec. The algebra writes the artifact.

**A deterministic Rust renderer that turns a ~90-token variant spec into a full
structured artifact — data, reports, tables, code scaffolds — for 0 LLM tokens.**

[![Release](https://img.shields.io/github/v/release/kostyk348/spec2artifact?sort=semver&color=0ea5e9&label=release)](https://github.com/kostyk348/spec2artifact/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/kostyk348/spec2artifact/ci.yml?branch=master&label=CI)](https://github.com/kostyk348/spec2artifact/actions/workflows/ci.yml)
[![tests](https://img.shields.io/badge/tests-24%20passing-22c55e)](#development)
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
| `records` | n, seed, categories + name pools, ranges | n records (jsonl / csv / json) |
| `report` | title, sections, bullets | Markdown |
| `table` | columns, rows or pools | Markdown table |
| `code` | language, module, imports, signatures+docs | a code scaffold |
| `list` | title, items, checked | a ticked list |
| `dataset` | n, alphabet, length, rule | a synthetic benchmark with a KNOWN label |
| `prose` | topic + points per paragraph, tone | readable template NLG |

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
cargo test                                 # 24 tests: renderers, validation, CLI, MCP over stdio
cargo build --release                      # the binary
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the PR checklist and
[CHANGELOG.md](CHANGELOG.md) for release history.

## Caveats

* The output is only as rich as the spec; open-ended inventiveness lives in the (small) spec.
* Best for **structured** artifacts. For free prose the spec must be rich, which eats tokens.
* Deterministic: same spec (incl. `seed`) → same artifact.
