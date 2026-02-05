# Knowledge Base Indexing Protocol

## Purpose

To maintain the **"Documentation Fractal"** while adhering to strict repository formatting rules. In the Nightshift methodology, the codebase is a traversable graph where every node provides context. This ensuring agents never get "lost" and can always trace a path back to the root vision.

## Triggers

- **Post-Dev**: Run immediately after a feature is completed/merged.
- **Idle State**: Run continuously when agents are waiting for human input.

## The Fractal Standard (Compliance Rules)

All documentation must strictly adhere to the project's formatting and location rules.

**CRITICAL**: Before proceeding, read and comply with the rules defined in:
👉 `docs/documentation-rules.md`

### Core Requirements:
1.  **Root-to-Leaf Connectivity**: Every file must be reachable via links starting from the root `README.md`.
2.  **Bi-Directional Navigation**: Every `README.md` must link "Up" to its parent and "Down" to its children or canonical docs in `./docs/`.
3.  **Code-Adjacent Documentation**: Docstrings/Rustdoc must be used in source files for implementation details.

## Steps

### 1. The Crawl (Topology Check)

Start at the project root `README.md` and perform a depth-first traversal:

- **Check Directory Identity**: Does every major directory have a `README.md`?
    - _Action_: If missing, create one. It must summarize the directory and link back to the parent and relevant canonical docs in `./docs/`.
- **Enforce Location Rules**: Are there any `.md` files outside `./docs/` that are NOT named `README.md`?
    - _Action_: Move them to `./docs/`, rename to kebab-case, and update links.

### 2. Code Interface Gardening

Walk through modified source files:

- **Public API Audit**: Ensure exported functions, structs, and traits have Rustdoc (`///`) comments focusing on intent.
- **In-line Logic**: Explain the *Why* for complex blocks.

### 3. Link & Naming Verification

- **Dead Link Hunt**: Scan all markdown files for broken relative links.
- **Kebab-Case Check**: Ensure all files in `./docs/` (except `README.md` and standard GitHub files) are lowercase and hyphenated.
- **Canonical Linking**: Ensure module `README.md` files link to the relevant deeper documentation in `./docs/`.

### 4. Index Maintenance

Maintain the following indices in `./docs/`:
- `project-index.md`: High-level map of the entire project.
- `source-index.md`: Granular map of the `src/` directory.

## Checklist

- [ ] Is the root `README.md` the clear starting point?
- [ ] Are there ZERO markdown files outside `./docs/` other than `README.md`?
- [ ] Do all files in `./docs/` follow kebab-case?
- [ ] Are all new exported code symbols documented?
- [ ] Do all directory `README.md` files have bi-directional links?
