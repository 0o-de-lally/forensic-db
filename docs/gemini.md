# Nightshift Protocol

You are operating under the **Nightshift Protocol** - a methodology for semi-autonomous AI engineering.

## CRITICAL: Read Full Protocol

**Immediately read `.nightshift/AGENTS.md` for the complete protocol.**

## Session Start Checklist

1. Read `.nightshift/state/forward-prompt.md` for prior context
2. Read `.nightshift/state/nag-status.json` for current nag states
3. Follow the Nightshift Protocol from `.nightshift/AGENTS.md`

## Before Every Commit

1. Run quality checks (build, test, lint)
2. Update `.nightshift/state/nag-status.json` with results:
    ```json
    { "sessionId": "your-id", "nags": { "javascript-nag": "OK" }, "lastUpdated": "ISO-8601" }
    ```
3. If any nag is `NOK`, fix it before committing (git hooks will block)
4. Use Git-Brain commit format from `.nightshift/commands/git-brain-commit.md`

## Key Resources

| Resource                 | Path                                        |
| ------------------------ | ------------------------------------------- |
| **Core Protocol**        | `.nightshift/AGENTS.md`                     |
| **Engineer Persona**     | `.nightshift/agents/engineer.md`            |
| **Planner Persona**      | `.nightshift/agents/planner.md`             |
| **Git-Brain Commit SOP** | `.nightshift/commands/git-brain-commit.md`  |
| **Nag Update SOP**       | `.nightshift/commands/update-nag-status.md` |
| **Nag Status**           | `.nightshift/state/nag-status.json`         |
| **Forward Prompt**       | `.nightshift/state/forward-prompt.md`       |

## Operating Principles

1. **Autonomy**: Solve problems independently; only escalate when truly blocked
2. **Safety**: Never run destructive commands without certainty
3. **Quality**: Enforce standards via Nags (build, test, lint)
4. **Git Discipline**: Always work in the provided branch
5. **Continuity**: Maintain forward-prompt for session handoff

## Workflow Loop

```
1. Research  ->  Gather context, read forward-prompt
2. Plan      ->  Outline steps before coding
3. Execute   ->  Write code, update forward-prompt regularly
4. Verify    ->  Run checks, update nag-status, commit
```

## Lazy Loading

Load files on a need-to-know basis:

- Read `.nightshift/agents/engineer.md` when you need persona guidance
- Read `.nightshift/commands/git-brain-commit.md` when committing
- Read `.nightshift/commands/new-module-development.md` when starting new modules
- Read `.nightshift/nags/javascript-nag.md` when running quality checks

## Quick Commands

When asked to perform these operations:

- **"/session-start"**: Read forward-prompt and nag-status, summarize prior context
- **"/nag-check"**: Run build/test/lint, update nag-status.json
- **"/forward-prompt"**: Update `.nightshift/state/forward-prompt.md` with current state
- **"/git-brain-commit"**: Follow `.nightshift/commands/git-brain-commit.md` protocol
- **"/new-module"**: Follow `.nightshift/commands/new-module-development.md` workflow (Plan → Stub → Implement)

## Gemini CLI Specific

- Use `/save` to checkpoint important conversation states
- Use `/chat new` to start fresh when switching tasks
- The forward-prompt in `.nightshift/state/forward-prompt.md` enables context continuity between sessions
