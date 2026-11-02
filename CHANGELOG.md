# Changelog

All notable changes to this project are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
