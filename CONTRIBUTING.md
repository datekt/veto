# Contributing to Veto

Thank you for your interest in improving Veto! This document describes how to set up the project, run the test suite, and submit high-quality pull requests.

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable channel)
- `rustfmt` and `clippy` components: `rustup component add rustfmt clippy`

## Getting Started

```bash
git clone https://github.com/datekt/veto
cd veto
cargo build
cargo test
```

## Project Layout

- `src/args.rs` — CLI argument parsing and default exclude list.
- `src/errors.rs` — error/report data structures and JSON report writer.
- `src/interactive.rs` — interactive terminal UI and tutorial.
- `src/parser.rs` — file scanning and pattern matching logic.
- `src/lib.rs` — public scanning API.
- `src/main.rs` — entry point that dispatches CLI vs interactive mode.
- `tests/integration_tests.rs` — end-to-end tests for the scanning pipeline.

## Local Checks (must pass before submitting a PR)

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --verbose
cargo audit
```

## Coding Conventions

- Formatting is enforced by `rustfmt.toml` (max width 120, 4 spaces).
- Prefer small, focused functions over large monolithic blocks.
- Public items should have doc comments.
- New detections must ship with a matching integration test.
- Every user-visible change goes into `CHANGELOG.md` under `[Unreleased]`.

## Submitting a Pull Request

1. Fork the repository and create a topic branch from `main`.
2. Make your changes and commit them with a clear, imperative message (e.g., `add detection for XXX_CRITICAL`).
3. Push the branch and open a pull request using the provided template.
4. Ensure CI passes; maintainers will review as soon as possible.

## Reporting Bugs

Please use the GitHub issue templates (bug report / feature request).
For security issues, follow `.github/SECURITY.md` instead of opening a public issue.

## License

By contributing, you agree that your contributions will be licensed under the MIT License (see `LICENSE`).