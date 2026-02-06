# Getting Started

A comprehensive guide to setting up and running the forensic-db ETL system.

## Prerequisites

### Required
- **Rust** 1.78+ and Cargo
- **Neo4j** 5.x (local, Docker, or cloud-hosted)
- **Git** for cloning repositories
- **Disk Space** - Archive sizes vary:
  - v5: ~10 GB compressed
  - v6: ~50 GB compressed
  - v7: ~100 GB compressed

### Optional
- **Docker** (recommended for easy Neo4j setup)
- **Unzip** utility (for manual archive extraction)

## Installation

### 1. Build from Source

```bash
# Clone the repository
git clone https://github.com/0o-de-lally/forensic-db
cd forensic-db

# Build release binary
cargo build --release

# Install to PATH (optional)
cp ./target/release/libra-forensic-db ~/.cargo/bin/
```

### 2. Verify Installation

```bash
libra-forensic-db --version
libra-forensic-db --help
```

## Database Setup

You have three options for running Neo4j:

### Option A: Built-in Docker Command (Recommended)

The easiest way to get started:

```bash
libra-forensic-db local-docker-db --data-dir ./neo4j_data
```

This will:
- Pull the Neo4j Docker image (if not present)
- Start a Neo4j instance on ports 7474 (HTTP) and 7687 (Bolt)
- Store data in `./neo4j_data`
- Default credentials: `neo4j` / `neo4j` (override with `--db-username` and `--db-password`)

Access the Neo4j Browser at `http://localhost:7474`

### Option B: Manual Docker Setup

For more control over Docker configuration:

```bash
docker run \
    --name forensic-neo4j \
    --restart always \
    --publish=7474:7474 \
    --publish=7687:7687 \
    --env NEO4J_AUTH=neo4j/your-password \
    --volume=/path/to/your/data:/data \
    neo4j:5.25.1-community
```

**Common Docker configurations:**

```bash
# No authentication (development only)
--env NEO4J_AUTH=none

# With authentication
--env NEO4J_AUTH=neo4j/your-password

# Increase memory (for large datasets)
--env NEO4J_dbms_memory_heap_initial__size=4G \
--env NEO4J_dbms_memory_heap_max__size=8G \
--env NEO4J_dbms_memory_pagecache_size=4G
```

### Option C: Neo4j Desktop or Cloud

**Neo4j Desktop:**
1. Download from https://neo4j.com/download/
2. Create a new database
3. Set credentials
4. Start the database
5. Note the connection URI (usually `neo4j://localhost:7687`)

**Neo4j AuraDB (Cloud):**
1. Sign up at https://neo4j.com/cloud/aura/
2. Create a free instance
3. Download credentials
4. Use the provided `neo4j+s://` URI

## Configuration

### Environment Variables (Recommended)

Create a `.env` file or export variables:

```bash
export LIBRA_GRAPH_DB_URI='neo4j://localhost:7687'
export LIBRA_GRAPH_DB_USER='neo4j'
export LIBRA_GRAPH_DB_PASS='your-password'
export RUST_LOG=info  # or: debug, trace, warn, error
```

**Connection URI formats:**
- Local: `neo4j://localhost:7687`
- Secure: `neo4j+s://example.databases.neo4j.io`
- Legacy: `bolt://localhost:7687`

### Command Line Arguments

Override environment variables for a single command:

```bash
libra-forensic-db \
    --db-uri 'neo4j+s://example.databases.neo4j.io' \
    --db-username 'neo4j' \
    --db-password 'your-password' \
    <subcommand>
```

### Additional CLI Options

```bash
--threads <n>         # Max parallel tasks (default: CPU count)
--clear-queue         # Force clear processing queue
```

## Getting Archive Data

### Clone the Archive Repository

The 0L blockchain archives are stored in Git repositories with separate branches for each version. Some archives are also available on S3-compatible mirrors:

```bash
# Clone v6 archives (recommended for most users)
git clone https://github.com/0LNetworkCommunity/epoch-archive-mainnet \
  --depth 1 \
  --branch v6 \
  epoch-archives

# Or clone v5 archives
git clone https://github.com/0LNetworkCommunity/epoch-archive-mainnet \
  --depth 1 \
  --branch v5 \
  epoch-archives

# Or clone v7 archives (latest)
git clone https://github.com/0LNetworkCommunity/epoch-archive-mainnet \
  --depth 1 \
  --branch v7 \
  epoch-archives
```

**Pro tip:** Use `--depth 1` to avoid downloading full Git history (saves time and space).

### Alternative Archive Sources

If GitHub is slow or unavailable, use alternative mirrors (e.g., Cloudflare R2):

See https://github.com/0LNetworkCommunity/libra-archive-mirrors for available mirrors and instructions.

### Archive Structure

Each version has two types of archives:

```
epoch-archives/
├── transaction/          # Transaction records
│   ├── 0000-0099/
│   ├── 0100-0199/
│   └── ...
└── account_state/        # Account state snapshots
    ├── 0000-0099/
    ├── 0100-0199/
    └── ...
```

**Note:** Future versions will auto-extract gzip files. Currently, you may need to unzip archives manually if they're compressed.

## Your First Ingestion

### 1. Start the Database

```bash
libra-forensic-db local-docker-db --data-dir ./neo4j_data
```

### 2. Ingest Transaction Data

```bash
# Navigate to your archive directory
cd epoch-archives

# Set log level for progress monitoring
export RUST_LOG=info

# Ingest all transaction archives
libra-forensic-db ingest-all \
  --start-path . \
  --archive-content transaction
```

Progress will be logged to stdout based on the `RUST_LOG` level.

### 3. Query Your Data

Open Neo4j Browser at `http://localhost:7474` and run:

```cypher
// Count total nodes
MATCH (n) RETURN count(n);

// Count transactions
MATCH (tx:Transaction) RETURN count(tx);

// Find recent transactions
MATCH (a:Account)-[tx:Tx]->(b:Account)
RETURN a, tx, b
ORDER BY tx.timestamp DESC
LIMIT 10;
```

### 4. Ingest Account State (Optional)

For balance history and state analysis:

```bash
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content account_state
```

## Performance Tuning

### Batch Size

Adjust `--batch-size` based on your system:
- **Default:** 250
- **More memory available:** 1000-5000

```bash
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --batch-size 5000
```

### Parallel Processing

Control parallelism with `--threads`:

```bash
libra-forensic-db --threads 8 ingest-all \
  --start-path ./epoch-archives
```

### Neo4j Memory Configuration

For large datasets, increase Neo4j memory (Docker):

```bash
docker run \
    --publish=7474:7474 --publish=7687:7687 \
    --env NEO4J_AUTH=none \
    --env NEO4J_dbms_memory_heap_max__size=8G \
    --env NEO4J_dbms_memory_pagecache_size=4G \
    --volume=/path/to/data:/data \
    neo4j:5.25.1-community
```

## Troubleshooting

### Connection Errors

```
Error: Failed to connect to Neo4j
```

**Solutions:**
1. Verify Neo4j is running: `docker ps` or check Neo4j Desktop
2. Check connection URI format: `neo4j://` or `neo4j+s://`
3. Verify credentials (username/password)
4. Check firewall settings on ports 7474 and 7687

### Out of Memory

```
Error: OOM or Java heap space
```

**Solutions:**
1. Increase Neo4j heap size (see Performance Tuning)
2. Reduce `--batch-size`
3. Reduce `--threads`

### Archive Not Found

```
Error: No archives found at path
```

**Solutions:**
1. Verify archive path is correct
2. Ensure you're in the correct branch (v5, v6, or v7)
3. Check that archive files are fully downloaded (not just git stubs)

### Slow Ingestion

**Common causes:**
- Network-attached storage (NAS) - use local SSD
- Low memory allocation for Neo4j
- Too many parallel threads causing contention
- Disk I/O bottleneck

**Solutions:**
1. Use local SSD for Neo4j data directory
2. Increase Neo4j memory allocation
3. Reduce `--threads` to avoid contention
4. Monitor with `RUST_LOG=debug` to identify bottlenecks

## Next Steps

- **Query Examples:** [Sample CQL Queries](../technical/sample-cql.md)
- **Add Off-chain Data:** [User Guide - Enrichment](user-guide.md#enrichment)
- **Run Analytics:** [User Guide - Analytics](user-guide.md#analytics)
- **Deep Dive:** [Architecture Documentation](../technical/architecture.md)

## Common Workflows

### Full Chain Analysis

```bash
# 1. Start database
libra-forensic-db local-docker-db --data-dir ./neo4j_data

# 2. Ingest transactions
libra-forensic-db ingest-all --start-path ./epoch-archives --archive-content transaction

# 3. Ingest account states
libra-forensic-db ingest-all --start-path ./epoch-archives --archive-content account_state

# 4. Add exchange data (if available)
libra-forensic-db enrich-exchange --exchange-json ./exchange-orders.json

# 5. Run analytics
libra-forensic-db analytics exchange-rms --persist
```

### Investigating Specific Epochs

```bash
# Process only epoch 100-199
libra-forensic-db ingest-one --archive-dir ./epoch-archives/transaction/0100-0199

# Verify before loading
libra-forensic-db check --archive-dir ./epoch-archives/transaction/0100-0199
```

### Development/Testing Setup

```bash
# Quick setup for testing with small dataset
libra-forensic-db local-docker-db --data-dir ./test_db
libra-forensic-db ingest-one --archive-dir ./epoch-archives/transaction/0000-0099
```

## Support

- **Issues:** https://github.com/0o-de-lally/forensic-db/issues
- **Documentation:** [docs/](../README.md)
- **Neo4j Help:** https://neo4j.com/docs/
