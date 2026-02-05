# Nightshift Git Hooks

Pure JavaScript git hooks for enforcing quality gates.

## Files

| File | Purpose |
|------|---------|
| `pre-commit` | Blocks commits if nags are incomplete |
| `commit-msg` | Validates commit message format |

## Installation

```bash
# Install git hooks
cp templates/hooks/pre-commit .git/hooks/
cp templates/hooks/commit-msg .git/hooks/
chmod +x .git/hooks/pre-commit .git/hooks/commit-msg

# Create nag status file
mkdir -p .nightshift
echo '{"sessionId":"manual","nags":{},"lastUpdated":""}' > .nightshift/nag-status.json
```

## How It Works

1. The pre-commit hook reads `.nightshift/nag-status.json`
2. If any nag has `completed: false`, the commit is blocked
3. After running quality checks, update the nag status file (see `templates/commands/update-nag-status.md`)
4. The commit proceeds

## Bypass

For human-supervised commits:

```bash
NIGHTSHIFT_BYPASS=1 git commit -m "wip: checkpoint"
```

