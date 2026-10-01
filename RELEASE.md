# Release Guide

This document describes how to prepare, build, and publish a new release of Veto
for Windows, Linux, and macOS.

---

## Overview

Veto is a single cross-platform Rust codebase. It is compiled separately for each
target OS, producing three native binaries from the same source code:

| OS      | Binary                      | Extension |
| ------- | --------------------------- | --------- |
| Windows | `veto-windows-x86_64.exe`   | `.exe`    |
| Linux   | `veto-linux-x86_64`         | none      |
| macOS   | `veto-macos-x86_64`         | none      |

Only Windows uses the `.exe` extension. On Linux and macOS, executables are
identified by the `chmod +x` permission bit, not by their name.

---

## Prerequisites

Before releasing, make sure the following are in place:

- `main` branch is up to date and all CI checks pass (green).
- `CHANGELOG.md` has an entry for the version being released.
- `Cargo.toml` has the correct `version` field.
- `.github/workflows/release.yml` is present and committed.
- Repository setting **Settings → Actions → General → Workflow permissions** is
  set to **Read and write permissions** (otherwise the release step will fail).

---

## Pre-release Checklist

Run all of these locally and confirm they pass:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --verbose
```

Optionally, verify dependency security:

```bash
cargo audit
```

If any of these fail, fix them before proceeding.

---

## Step 1 — Bump the Version

Update the version in `Cargo.toml`:

```toml
[package]
name = "veto"
version = "0.2.0"
```

Update `CHANGELOG.md` by moving the `[Unreleased]` section into a new versioned
section with the release date:

```markdown
## [0.2.0] - 2026-11-15

### Added
- ...
```

Commit the changes:

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "release: v0.2.0"
git push origin main
```

---

## Step 2 — Tag the Release

Create an annotated tag matching the version (must start with `v`):

```bash
git tag -a v0.2.0 -m "Veto 0.2.0"
git push origin v0.2.0
```

Pushing the tag automatically triggers the **Release Binaries** workflow in
GitHub Actions.

---

## Step 3 — Monitor the Build

1. Open the repository on GitHub.
2. Go to the **Actions** tab.
3. Open the running **Release Binaries** workflow.
4. Three jobs run in parallel:

   - `Build for x86_64-unknown-linux-gnu` (ubuntu-latest)
   - `Build for x86_64-pc-windows-msvc` (windows-latest)
   - `Build for x86_64-apple-darwin` (macos-latest)

   Each takes about 3–5 minutes.

---

## Step 4 — Verify the Release

Once all jobs complete:

1. Go to the **Releases** page.
2. Open the newly created release (named after the tag, e.g. `v0.2.0`).
3. Confirm the following assets are attached:

   ```
   veto-linux-x86_64
   veto-windows-x86_64.exe
   veto-macos-x86_64
   ```

4. Download each and sanity-check it:

   - **Windows:** run `veto-windows-x86_64.exe --version` in PowerShell.
   - **Linux:** `chmod +x veto-linux-x86_64 && ./veto-linux-x86_64 --version`
   - **macOS:** `chmod +x veto-macos-x86_64 && xattr -d com.apple.quarantine veto-macos-x86_64 && ./veto-macos-x86_64 --version`

   All three should print `veto 0.2.0`.

---

## Step 5 — Publish the Release Notes

GitHub creates a draft release automatically. Edit it:

1. Add a short summary of the release.
2. Paste the relevant section of `CHANGELOG.md`.
3. Mention any breaking changes.
4. Click **Publish release**.

---

## Step 6 — Post-release

1. In `CHANGELOG.md`, add a fresh `## [Unreleased]` section at the top.
2. Update the compare links at the bottom of `CHANGELOG.md`:

   ```markdown
   [Unreleased]: https://github.com/datekt/veto/compare/v0.2.0...HEAD
   [0.2.0]: https://github.com/datekt/veto/releases/tag/v0.2.0
   ```

3. Commit and push:

   ```bash
   git add CHANGELOG.md
   git commit -m "changelog: start 0.3.0 development"
   git push origin main
   ```

---

## Manual Release (Fallback)

If GitHub Actions is unavailable, build locally.

### Windows

```powershell
cargo build --release
```

Result: `.\target\release\veto.exe`. Rename to `veto-windows-x86_64.exe`.

### Linux

```bash
cargo build --release
```

Result: `./target/release/veto`. Rename to `veto-linux-x86_64`.

### macOS

```bash
cargo build --release
```

Result: `./target/release/veto`. Rename to `veto-macos-x86_64`.

> Note: A macOS binary **cannot** be produced from a Windows or Linux machine.
> Apple's SDK is required and is only licensed for use on Apple hardware.

### Cross-compilation from Linux

If you have Docker and [`cross`](https://github.com/cross-rs/cross) installed:

```bash
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
```

macOS still requires a Mac or a macOS-runner in CI.

Then manually create a GitHub release and upload the three binaries.

---

## Exit Codes

Users and CI pipelines rely on these exit codes:

| Code | Meaning                                                    |
| ---- | ---------------------------------------------------------- |
| `0`  | No critical bugs detected — build passed.                  |
| `1`  | Critical bugs detected — build vetoed.                     |
| `2`  | Execution error (I/O failure, invalid arguments, etc.).    |

Any change to these codes is a **breaking change** and must be documented in
`CHANGELOG.md`.

---

## Rollback

If a release is broken:

1. Do **not** delete the tag — it may already be referenced elsewhere.
2. Mark the release as **pre-release** on GitHub.
3. Publish a hotfix release with a patch version bump (e.g. `v0.2.1`).
4. Add a note in the broken release's description pointing to the fix.

---

## Versioning Policy

Veto follows [Semantic Versioning](https://semver.org/):

- **MAJOR** — breaking changes to the CLI, JSON schema, or exit codes.
- **MINOR** — new detections, new flags, backwards-compatible features.
- **PATCH** — bug fixes, docs, internal refactors.

Pre-1.0 (`0.x.y`), the API is considered unstable, but breaking changes should
still be clearly documented in `CHANGELOG.md`.