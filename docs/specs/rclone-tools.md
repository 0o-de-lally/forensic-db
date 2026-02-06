# Rclone Tools Specification

## Overview

This specification details the tooling required to integrate `rclone` for managing data mirrors and backups within the `forensic-db` project. The goal is to provide a robust, verified way to inspect, download, and manage data from external mirrors (initially Cloudflare R2).

## Components

### 1. Mirror Inspection Script

**Script Path**: `scripts/inspect_cf_mirror.sh`

**Purpose**:
To verify connectivity and list contents of the Cloudflare R2 mirror without requiring the user to manually configure `rclone` with keys if the bucket is public, or using provided credentials.

**Requirements**:
- **Inputs**:
  - Mirror Name/URL (defaulting to the project's configured mirror).
  - (Optional) Credentials if private.
- **Outputs**:
  - List of available snapshots/archives.
  - Verification of access (exit code 0 on success).
- **Dependencies**:
  - `rclone` (must be installed and available in PATH).
  - `jq` (optional, for parsing JSON output if used).

**Behavior**:
1. Check for `rclone` installation.
2. Configure a temporary or on-the-fly `rclone` remote for the Cloudflare R2 endpoint.
3. List files in the bucket.
4. Filter/display relevant archive files (e.g., `.tar.gz`, `.zst`).

### 2. Download Command (Future)

**Command**: `cargo run -- rclone-download` (Proposed)

**Purpose**:
To automate the download and verification of data snapshots from the mirror.

## Usage

```bash
# Inspect the default mirror
./scripts/inspect_cf_mirror.sh

# Inspect a specific bucket (if supported)
./scripts/inspect_cf_mirror.sh --bucket my-bucket
```

## Implementation Details

The `inspect_cf_mirror.sh` script utilizes `rclone lsjson` or `rclone lsd` to fetch directory listings. It handles the `S3` compatible API of Cloudflare R2.
