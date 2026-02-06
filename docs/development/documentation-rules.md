# Documentation Rules

This document outlines the rules for documentation files in this repository. These rules are enforced by a git pre-push hook.

## 1. Location Structure

- **Substantial Documentation:** All substantial documentation files must reside in the appropriate subfolder of the root `./docs/` directory.
- **Root and Modules:** Outside of the `./docs/` directory, only `README.md` files are permitted. These should strictly summarize the contents of that directory (e.g., describing a module) and link back to canonical documents within `./docs/`.

## 2. Naming Conventions

### Standard: Kebab-case

All documentation files (located within `./docs/` subdirectories) must use **kebab-case** (lowercase letters separated by hyphens).

- **Correct:** `my-feature-specs.md`, `getting-started.md`
- **Incorrect:** `MyFeatureSpecs.md`, `getting_started.md`, `GETTING_STARTED.md`

### Reasoning

1. **URL Compatibility:** Hyphens are treated as word separators in URLs, making them SEO-friendly and cleaner when served as static pages. Underscores are often treated as joining characters.
2. **OS Compatibility:** Lowercase naming prevents "works on my machine" issues where case-insensitive file systems (Windows/macOS) mask broken links that fail on case-sensitive systems (Linux/CI).

### Exceptions

The following files are exempt from the kebab-case rule and should use standard conventions:

- `README.md`: Must be capitalized to stand out in directory listings.
- `LICENSE`: Standard license file.
- `CONTRIBUTING.md`: GitHub standard community file.
- `CODE_OF_CONDUCT.md`: GitHub standard community file.

## 3. Enforcement

A git pre-push hook runs a Node.js script to verify these rules. If a violation is detected, the push will be rejected, and you will be referred to this document.
