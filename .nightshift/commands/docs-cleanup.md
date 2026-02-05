# Knowledge Base Gardening Protocol

## Purpose

To perform "Steady State Gardening" on the repository's documentation. This process ensures the Knowledge Base remains accurate, relevant, and navigable without destroying historical context.

## Philosophy

- **Non-Destructive**: We rarely delete knowledge; we deprecated it.
- **Cascade Updates**: Changes in high-level vision must flow down to implementation guides.
- **Navigable**: Dead links are the enemy. Every node must be reachable.

## The Gardening Standard (Compliance Rules)

All documentation must strictly adhere to the project's formatting and location rules.

**CRITICAL**: Before proceeding, read and comply with the rules defined in:
👉 `docs/documentation-rules.md`

## Steps

### 1. Audit Phase (The Scrub)

- **Scan**: Walk through the `docs/` directory (or user-specified target).
- **Gather Intelligence**: For each file, check its Git history to determine "Liveness":
    - **Freshness**: When was it last changed? (`git log -1 --format=%cd <file>`)
        - _Recent_ = Likely Source of Truth.
        - _Ancient_ = High scrutiny needed.
    - **Frequency**: How often has it been edited? (`git rev-list --count HEAD <file>`)
        - _High Count_ = Battled-tested, reviewed, high value.
        - _Low Count (1-2)_ = "Fire and forget" draft, likely obsolete.
- **Rate Relevance**: Combine content analysis with Git intelligence:
    - **High**: Recently updated, frequently edited, core to current Vision/Architecture.
    - **Low**: Old (>6 months), few edits, refers to abandoned features.
    - **Reference**: Historical context that shouldn't be deleted but isn't "live".

### 2. Pruning Phase (Deprecation)

If a file is deemed **Low Relevance** or **Obsolete**:

1.  **Do Not Delete**: Unless it is truly garbage/temp.
2.  **Rename**: Append `.deprecated.md` to the filename. Ensure kebab-case is maintained.
3.  **Header**: Add a deprecation notice at the top of the file.
    ```markdown
    > **⚠️ DEPRECATED**: This document is preserved for historical context.
    > See [New Document Name](./path/to/new.md) for the current version.
    ```

### 3. Connection Phase (Repair)

After renaming files, you must fix the "Web of Links":

1.  **Check Indices**: Update `docs/project-index.md` or `README.md`.
    - Move deprecated links to an "Archive" section or remove them if irrelevant.
2.  **Fix Broken Links**: `grep` for references to the renamed files and update them to point to the new `.deprecated.md` path (or the replacement file).

### 4. Expansion Phase (Fractal Filling)

If new code or features have been added:

1.  **Identify Gaps**: Are there key directories without a `README.md`?
2.  **Create Stubs**: Generate minimal documentation for new areas, linking back to the parent.

## Checklist

- [ ] Have I identified obsolete files?
- [ ] Did I rename them with `.deprecated.md` instead of deleting?
- [ ] Have I added a deprecation banner to the old files?
- [ ] Did I update `docs/project-index.md` to reflect the clean-up?
- [ ] Did I verify that no dead links were created?
- [ ] Did I follow `docs/documentation-rules.md` for all changes?
