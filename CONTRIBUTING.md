# Contributing to spec2artifact

Thanks for helping make the LLM→spec→artifact split better. The rules are short on purpose.

## Ground truth

- The renderer is **pure**: `spec -> artifact`, no network, no clock, no environment.
  Any randomness comes only from the spec's `seed`.
- **Determinism is the product.** If your change alters rendered output, that is a breaking
  change: say so loudly in the PR and bump the minor version.
- The MCP tools manifest is **frozen**. Adding/renaming a tool breaks agent prompt caches.
  If you must, change `tools` and the fingerprint in one commit and explain the one-time
  cache cost in the PR.

## Dev environment

A stable Rust toolchain (1.75+) is everything:

```bash
cargo test
```

## Before you open a PR

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] New behaviour has a test (renderers are table-driven and cheap to test).
- [ ] README/README.ru.md updated if the CLI surface, kinds or docs changed.

## PR style

Small, single-purpose PRs with a conventional-commit title (`feat:`, `fix:`, `docs:`,
`ci:`, `refactor:`). The PR template asks for the type, the determinism impact and the
checks — fill it in and you are done.
