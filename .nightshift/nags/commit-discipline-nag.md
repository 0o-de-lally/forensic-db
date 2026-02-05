# Nag: Commit Discipline

**Task**: Commit your pending changes immediately.

**Context**:
Your working changes have exceeded acceptable limits (Risk of data loss/complexity).
- **Violation**: {{violation_reason}}
- **Stats**: {{lines_changed}} lines, {{files_changed}} files changed.

**Instructions**:
1. Review your changes (`git diff`).
2. If waiting to commit, ensure ID "commit-discipline-nag" is "NOK" in `.nightshift/nag-status.json`.
3. Stage (`git add`) and Commit (`git commit`) your work.
4. After a successful commit, you may set the status to "OK" to clear this nag:
   ```json
   { "nags": { "commit-discipline-nag": "OK" } }
   ```

**State Action**:
- Write "NOK" to pause and fix.
- Write "OK" to signal completion.
