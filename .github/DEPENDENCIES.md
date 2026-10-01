# Dependencies

Veto intentionally keeps its dependency tree small and audit-friendly.
Every crate below is actively maintained and reviewed on a regular basis.

## Runtime dependencies

| Crate       | Version | Purpose                                                       |
| ----------- | ------- | ------------------------------------------------------------- |
| `clap`      | 4.5     | Command-line argument parsing (derive API).                   |
| `serde`     | 1.0     | Serialization/deserialization framework (derive feature).     |
| `serde_json`| 1.0     | JSON reader/writer used for `errors.json` reports.            |
| `walkdir`   | 2.5     | Recursive, iterator-based directory traversal.                |
| `chrono`    | 0.4     | Timestamping of scan reports (`scanned_at` field).            |
| `rayon`     | 1.10    | Data-parallel file scanning for maximum throughput.           |

## Security

Dependency vulnerabilities are automatically monitored by
[`cargo-audit`](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
on every push to `main` and weekly via `.github/workflows/audit.yml`.

## Updating dependencies

1. Run `cargo update`.
2. Run `cargo test` and `cargo clippy -- -D warnings`.
3. Run `cargo audit` locally.
4. Open a pull request with the updated `Cargo.lock`.