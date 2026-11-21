# Contributing

- Keep platform-specific probing behind small adapters.
- Keep the Rust core free of UI concerns.
- Unknown compatibility is the default; do not invent test results.
- No bundled game files, proprietary runtime binaries, secrets or user credentials.
- Add tests for safety-sensitive behavior.
- Run `cargo fmt --manifest-path core/Cargo.toml --all -- --check`, `cargo test --manifest-path core/Cargo.toml --workspace`, and `swift build --package-path apps/macos`.
