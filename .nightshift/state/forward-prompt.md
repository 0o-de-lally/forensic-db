# Forward Prompt

## Current Session

**Session ID**: session-001
**Started**: Thu Feb 05 2026
**Last Updated**: Thu Feb 05 2026

## Objective

Implement a new CLI subcommand to run a persistent Neo4j instance using Docker. This avoids local Neo4j installation while ensuring data persistence across container restarts.

## Current Status

- **Completed**: Implemented `LocalDockerDb` subcommand.
- **Completed**: Documentation updated (Usage + Architecture/Tradeoffs + Mirrors).
- **Tooling**: Created `scripts/inspect_cf_mirror.sh` to validate access to community data mirrors.

## Next Steps

- Merge PR.

## Blockers

None.

## Context Notes

- **Command**: `cargo run -- local-docker-db`
- **Mirrors**: Use `scripts/inspect_cf_mirror.sh` to check Cloudflare mirror status.
