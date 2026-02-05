# Nag: Clean Build Verification

**Task**: Verify that the current codebase is in a deployable state.

**Context**:
- **Build**: Run build command (e.g., `npm run build` or `tsc`) to check for compilation errors.
- **Tests**: Run test suite (e.g., `npm test`) to ensure no regressions.
- **Lint**: Run linter (e.g., `npm run lint`) to enforce style.

**Instructions**:
1. Perform the checks above.
2. If ANY check fails, update `.nightshift/nag-status.json`:
   ```json
   { "nags": { "javascript-nag": "NOK" } }
   ```
   Then fix the issue and retry.
3. If ALL checks pass, update `.nightshift/nag-status.json`:
   ```json
   { "nags": { "javascript-nag": "OK" } }
   ```
4. Proceed with your commit.
