# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-01

### Added

- Initial public release of Veto.
- Detection of unmerged Git conflict markers: `<<<<<<<`, `>>>>>>>`, `|||||||`.
- Detection of critical source markers: `TODO_CRITICAL`, `FIXME_CRITICAL`, `XXX_CRITICAL`.
- Interactive terminal menu with a built-in tutorial (Russian localization).
- CLI mode: `veto <PATH> [--output <FILE>] [--quiet]`.
- Structured JSON reports with a per-file error tree under `errors/` (interactive mode)
  and a single `errors.json` (CLI mode).
- Parallel file scanning using `rayon` for maximum throughput.
- Automatic skipping of binary files via null-byte probing.
- Default exclude list: `.git`, `node_modules`, `target`, `.venv`, `dist`, `build`, `errors`.
- CI pipeline (format, clippy, tests) and weekly security audit via `cargo-audit`.
- Cross-platform release workflow producing binaries for Linux, Windows and macOS.

### Exit codes

- `0` — no critical bugs detected.
- `1` — critical bugs detected (build vetoed).
- `2` — execution error (I/O failure, invalid arguments, etc.).

[Unreleased]: https://github.com/datekt/veto/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/datekt/veto/releases/tag/v0.1.0