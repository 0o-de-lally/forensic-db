# New Module Development Workflow

Use this command when starting development of a new module or significant feature.

## Overview

This SOP implements a structured **Plan → Stub → Implement** cycle to maximize context, minimize hallucinations, and ensure quality.

## When to Use This Command

- Starting a new module or package
- Building a significant new feature
- Implementing a complex architectural change
- When the task requires multiple sessions/agents to complete

## Workflow

### 1. Planning Phase (High-Capability Agent)

Create a comprehensive plan before writing code.

**Create Plan Document:**

- Location: `docs/plan/<module-name>-plan.md`
- Include:
    - Product features being built
    - Technical implementation details (libraries, patterns, architecture)
    - Prioritized task list (incremental, risk-first)

**Feasibility Review:**

- Review for technical correctness and viability
- Ensure sufficient context for execution
- Edit the plan directly (don't create separate review doc)

### 2. Living Plan Protocol

The plan is **not static** - it's the source of truth.

**Update Triggers:**

- Task completed
- Blocker found
- New information learned

**Actions:**

- Mark items: `[x] Complete`, `[ ] Blocked`
- Remove/descope items no longer needed
- Refactor plan if implementation reveals it was wrong

### 3. Stubbing Phase (High-Capability Agent)

Structure the codebase before implementing logic.

**CRITICAL**: All new documentation (including the module plan and README) must follow the rules in:
👉 `docs/documentation-rules.md`

**Generate Stubs:**

- Create all file structures, classes, function signatures
- Add module `README.md` (Note: Only `README.md` is allowed outside `./docs/`)
- Write extensive inline comments (JSDoc/Docstring)
- Ensure code compiles/lints (use `throw new Error("Not implemented")`)

**Stub Tests:**

- Create test files matching source files
- Write test case signatures describing expected behavior

**Commit and Update:**

- Commit the stubs
- Mark "Stubbing" phase complete in plan

### 4. TDD Implementation Phase (Standard Agent)

Fill in the blanks using Test-Driven Development.

**Write Tests First:**

- Ensure test exists and fails before implementing
- Tests serve as documentation

**Implement Logic:**

- Write minimum code to pass test
- Maintain inline documentation
- Link comments to design docs/plan where applicable

**Cycle:**

- Red (Fail test) → Green (Pass test) → Refactor
- Update plan after each logical unit

### 5. Iteration Loop

Repeat:

1. Check Plan
2. Pick next high-priority/high-risk task
3. Implement (TDD)
4. Update Plan
5. Commit

## Summary Checklist

- [ ] **Plan**: Created with risk-prioritized tasks
- [ ] **Review**: Plan validated for feasibility
- [ ] **Stub**: Structure created with verbose comments & compiling code
- [ ] **Implement**: TDD approach (Tests → Code → Refactor)
- [ ] **Maintain**: Plan updated after every significant step

## Agent Profiles

- **High-Capability**: Planning & Stubbing (Claude Opus, GPT-4o, Gemini Pro)
- **Standard**: Implementation (Claude Sonnet, GPT-4o-mini, Gemini Flash)

## Example Usage

```bash
# OpenCode
/new-module user-authentication

# Other agents
"I need to build a new user authentication module.
Follow the new-module-development workflow from .nightshift/commands/"
```
