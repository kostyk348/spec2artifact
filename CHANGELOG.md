# Changelog

All notable changes to this project are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [1.1.0] — 2026-11-03

### Added

- **`bundle` kind — multi-part artifacts with cross-part references**: named
  parts of any kind, `{ref:part=NAME,stat=STAT[,field=FIELD]}` placeholders,
  topological ordering (a reference cycle is a validation error reported as a
  path), exact statistics computed over the generated DATA (`count`, `sum`,
  `min`, `max`, `mean`, `top`, `unique`) — prose cites the data it describes,
  so the artifact is self-consistent by construction. Markdown (default,
  data parts fenced) and `plain` formats; deterministic seed inheritance for
  parts without their own seed. Example: `examples/bundle_report.{json,md}`
  (~160-token spec -> 600-record dataset + summary citing exact totals,
  8.8k output tokens, 0 LLM-written).
- **`records` distributions** (backward-compatible, same seed by default):
  `category_weights` (weighted categorical mix), `price_dist: normal |
  lognormal` (`price_mean`/`price_sd`; lognormal's `price_mean` is the true
  mean), `sum_to` (exact-sum splitting in integer cents).

### Fixed

- (none — default rendering is byte-identical to 1.0.0; covered by the test suite)

## [1.0.0] — 2026-11-02

### Added — Rust port (breaking: replaces the Python implementation)

- Single Rust crate (`spec2artifact`, lib + bin): same seven kinds, same CLI surface,
  same MCP stdio JSON-RPC server — now with zero runtime dependencies beyond Rust.
- Deterministic PRNG documented as part of the contract: SplitMix64 seeded from the spec,
  platform-independent, byte-identical output for the same spec on any machine.
  Seeds are NOT compatible with the Python implementation's Mersenne Twister stream.
- `spec2artifact fingerprint` — stable sha256 of the frozen MCP tools manifest
  (prompt-cache drift guard, replaces `mcp_fingerprint.py`).
- 24 tests: renderer exactness and determinism, validation rejections, CLI exit codes,
  full MCP flow over a real stdio session, garbage/notification tolerance.
- CapsuleCraft-level project quality: CI matrix (Linux/macOS/Windows) with fmt/clippy/test
  plus CLI and MCP smoke steps, README in English and Russian, CONTRIBUTING, changelog,
  MIT license, PR/issue templates.

### Changed

- CLI: `python -m spec2artifact.mcp_server` → `spec2artifact mcp`; malformed spec files
  now exit with a readable `error: …` message instead of a traceback; exit codes are
  documented (0 ok, 1 render error, 2 invalid spec / unknown kind).
- CSV output uses clean `\n` line endings (the Python `csv` module emitted `\r\n`).

### Removed

- The Python package (`spec2artifact/`, `mcp_run.py`, `mcp_fingerprint.py`,
  `pyproject.toml`, pytest suite) — fully replaced by the Rust crate.

[1.0.0]: https://github.com/kostyk348/spec2artifact/releases/tag/v1.0.0
