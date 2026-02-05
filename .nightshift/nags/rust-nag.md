# Nag: Rust Quality Verification

**Task**: Verify that the current Rust codebase passes build, test, and lint checks.

**Context**:
- **Build**: Run `cargo build` to check for compilation errors.
- **Tests**: Run `cargo test` to ensure no regressions.
- **Lint**: Run `cargo clippy` and `cargo fmt --check` to enforce style and best practices.

**Instructions**:
1. Perform the checks above.
2. If ANY check fails, update `.nightshift/state/nag-status.json`:
   ```json
   { "nags": { "rust-nag": "NOK" } }
   ```
   Then fix the issue and retry.
3. If ALL checks pass, update `.nightshift/state/nag-status.json`:
   ```json
   { "nags": { "rust-nag": "OK" } }
   ```
4. Proceed with your commit.
