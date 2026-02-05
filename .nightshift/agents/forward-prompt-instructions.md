# Forward Prompt Instructions

> These instructions should be included in all agent personas to ensure proper maintenance of the forward prompt file for agent continuity.

## Agent Continuity: Forward Prompt

You must maintain the `.nightshift/forward-prompt.md` file throughout your session. This file ensures that if you are disconnected or switched out, the next agent can seamlessly continue your work.

### When to Update the Forward Prompt

Update the forward prompt file:

1. **After completing any significant step** - Document what you just finished
2. **Before starting a new task** - Update the "Next Steps" list
3. **When encountering a blocker** - Add it to the "Blockers" section
4. **Every 10-15 minutes of active work** - Keep the status current
5. **Before making a commit** - Ensure the prompt reflects post-commit state
6. **Before any natural stopping point** - Prioritize continuity

### What to Include

The forward prompt has these sections:

| Section | Purpose | Example |
|---------|---------|---------|
| **Objective** | High-level goal you're working toward | "Implement user authentication with JWT" |
| **Current Status** | What you've accomplished so far | "Created User model and login endpoint (untested)" |
| **Next Steps** | Prioritized TODO list (most important first) | "1. Write tests for login endpoint" |
| **Blockers** | Any issues preventing progress | "Need to decide on JWT vs session tokens" |
| **Context Notes** | Important context for the next agent | "Using bcrypt for password hashing per security requirements" |

### How to Update

Read the current file, modify it, save it, and optionally commit:

```bash
# Read and update the forward prompt
cat > .nightshift/forward-prompt.md << 'EOF'
# Forward Prompt

> This document describes the state of work for the next agent to continue.
> Last updated: 2026-01-17T16:30:00Z

## Objective

Implement user authentication feature

## Current Status

- Created User model with email/password fields
- Added login endpoint (POST /api/auth/login)
- Endpoint is implemented but untested

## Next Steps

1. Write unit tests for login endpoint
2. Add password hashing with bcrypt
3. Implement logout endpoint
4. Add refresh token support

## Blockers

- Need to decide: use JWT or session tokens? (waiting for architecture decision)

## Context Notes

- Using Express.js with TypeScript
- User model stored in PostgreSQL
- Security team requires bcrypt with 12 salt rounds minimum

---
<!-- FORWARD_PROMPT_METADATA
{
  "sessionId": "abc-123",
  "agentId": "engineer-1",
  "updatedAt": "2026-01-17T16:30:00Z"
}
-->
EOF

# Stage and commit (optional, but recommended before stopping)
git add .nightshift/forward-prompt.md
git commit -m "chore: update forward prompt"
```

### Critical Rules

1. **Never delete the forward prompt** - Only update it
2. **Be specific in next steps** - Vague steps like "continue implementing" are not helpful
3. **Order next steps by priority** - The first item should be what to do immediately
4. **Document blockers honestly** - Don't hide problems from the next agent
5. **Keep context notes relevant** - Only include information that affects the work

### On Session Start

When you begin a new session, **always read the forward prompt first**:

```bash
cat .nightshift/forward-prompt.md
```

Use this to understand:
- What the previous agent was working on
- What you should do next
- What problems to watch out for
- Important context you need to know

### Integration with Commits

The forward prompt should be committed along with your work:

- **Small commits**: Include forward prompt updates in the same commit
- **Before stopping work**: Always commit the forward prompt with a descriptive message
- **After being disconnected**: The next agent can find your last state in git

Example commit flow:
```bash
# Do some work...
# Update forward prompt...

git add .
git commit -m "feat: add login validation

Updated forward prompt with next steps for logout implementation."
```
