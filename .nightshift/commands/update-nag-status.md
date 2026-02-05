# Command: Update Nag Status

This command template explains how to update the `.nightshift/nag-status.json` file after completing quality checks.

## Nag Status File Location

```
.nightshift/nag-status.json
```

## File Format

```json
{
  "sessionId": "agent-session-id",
  "nags": {
    "javascript-nag": {
      "completed": false,
      "completedAt": null,
      "checks": {
        "build": false,
        "test": false,
        "lint": false
      }
    }
  },
  "lastUpdated": "2026-01-17T12:00:00.000Z"
}
```

## When to Update

Update the nag status after running your quality checks:

1. **After build passes**: Set `checks.build: true`
2. **After tests pass**: Set `checks.test: true`  
3. **After lint passes**: Set `checks.lint: true`
4. **When all checks pass**: Set `completed: true` and `completedAt: <timestamp>`

## Example: Mark Nags Complete

After running `npm run build && npm test && npm run lint`:

```json
{
  "sessionId": "your-session-id",
  "nags": {
    "javascript-nag": {
      "completed": true,
      "completedAt": "2026-01-17T16:50:00.000Z",
      "checks": {
        "build": true,
        "test": true,
        "lint": true
      }
    }
  },
  "lastUpdated": "2026-01-17T16:50:00.000Z"
}
```

## Example: Reset Nags for New Work

When starting a new task, reset the nags:

```json
{
  "sessionId": "new-session-id",
  "nags": {
    "javascript-nag": {
      "completed": false,
      "completedAt": null,
      "checks": {
        "build": false,
        "test": false,
        "lint": false
      }
    }
  },
  "lastUpdated": "2026-01-17T17:00:00.000Z"
}
```

## Project-Specific Nags

| Project Type | Nag Name | Checks |
|--------------|----------|--------|
| JavaScript/TypeScript | `javascript-nag` | build, test, lint |
| Python | `python-nag` | test, lint, typecheck |
| Rust | `rust-nag` | build, test, clippy |
| Go | `go-nag` | build, test, vet |

## Important

- The pre-commit hook reads this file and blocks commits if any nag has `completed: false`
- Always update `lastUpdated` when modifying the file
- Use ISO 8601 format for timestamps
