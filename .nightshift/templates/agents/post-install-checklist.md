# Post-Install Checklist Template

This template guides an agent through the Nightshift post-installation process.

## Template Variables

- `{{project.name}}` - Project name
- `{{project.root}}` - Project root directory
- `{{opencode.json}}` - Path to opencode.json
- `{{hooks.dir}}` - Path to scripts/hooks directory
- `{{git.branch}}` - Current git branch name
- `{{date}}` - Current date in YYYYMMDD format

## Agent Instructions

### Phase 1: Validate Installation

1. **Verify opencode.json exists and is valid**

    ```
    cat {{opencode.json}}
    ```

    - Confirm `$schema` is set to `https://opencode.ai/config.json`
    - Confirm `agent` section contains: engineer, planner, curator, git-supervisor, pm-supervisor, finance-supervisor

2. **Verify commands directory**

    ```
    ls -la {{project.root}}/.opencode/command/
    ```

    - Confirm command files are symlinked

3. **Report to user**
    ```
    ✅ OpenCode configuration is valid
    Agents: {{agents.list}}
    Commands: {{commands.count}} files linked
    ```

### Phase 2: Restart OpenCode Client

Tell the user:

```
🔄 Please restart your OpenCode client now.
   - Close and reopen, or reload the window
   - Then run this checklist again to continue
```

### Phase 3: Bootstrap GitBrain

1. **Create semantic branch**

    ```
    BRANCH_NAME="ns/session/nightshift-bootstrap-{{date}}"
    git checkout -b ${BRANCH_NAME}
    git push -u origin ${BRANCH_NAME}
    ```

2. **Verify branch**
    ```
    git branch --show-current
    ```

### Phase 4: Install Git Hooks

1. **Copy hooks from canonical location**

    ```
    HOOKS_DIR="{{hooks.dir}}"
    for hook in "${HOOKS_DIR}"/*.js; do
        if [ -f "$hook" ]; then
            cp "$hook" "{{project.root}}/.git/hooks/"
            chmod +x "{{project.root}}/.git/hooks/$(basename $hook)"
        fi
    done
    ```

2. **Customize hooks for project**

    Edit `{{hooks.dir}}/pre-commit.js` to add project-specific checks:
    - Linting rules for your codebase
    - Test commands
    - Documentation checks

    Edit `{{hooks.dir}}/pre-push.js` to add project-specific checks:
    - Secret scanning
    - API documentation verification
    - Schema validation

### Phase 5: Documentation Indexing

1. **Run curator agent**

    ```
    opencode --agent curator
    ```

2. **Curator will:**
    - Index `START_HERE.md` anchor
    - Build knowledge graph from documentation fractal
    - Update `.opencode/command/docs-index.md`

## Summary Output

```
============================================
✅ Nightshift Bootstrapping Complete!
============================================

📋 Summary:
   • Branch: {{git.branch}}
   • Config: {{opencode.json}}
   • Hooks: {{hooks.dir}}/

📝 Next Steps:
   1. Run 'bun run dev' to start development
   2. Use /commands in OpenCode (type / to see options)
   3. Edit scripts/hooks/ to customize git hooks

🔗 docs/development/agent-bootstrap-instructions.md
```
