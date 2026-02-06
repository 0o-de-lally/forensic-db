# Configuration Guide

Configuration options for forensic-db.

## Environment Variables

### Database Connection

| Variable | Description | Default |
|----------|-------------|---------|
| `LIBRA_GRAPH_DB_URI` | Neo4j connection URI | None (required) |
| `LIBRA_GRAPH_DB_USER` | Database username | None (required) |
| `LIBRA_GRAPH_DB_PASS` | Database password | None (required) |

### Logging

| Variable | Description | Values |
|----------|-------------|--------|
| `RUST_LOG` | Log level | `error`, `warn`, `info`, `debug`, `trace` |

## Command Line Options

### Global Options

```
--db-uri <URI>       (-r)  Database connection URI
--db-username <USER> (-u)  Database username
--db-password <PASS> (-p)  Database password
--threads <N>        (-t)  Max parallel tasks
--clear-queue        (-q)  Force clear processing queue
--help                     Show help
--version                  Show version
```

### Ingest Commands

```
ingest-all                   Scan and process all archives in a directory
  --start-path <PATH>  (-d)   Root directory of archives
  --archive-content <TYPE> (-c) Content type: transaction | account_state
  --batch-size <N>     (-b)   Records per batch (default: 250)

ingest-one                   Process a single archive bundle
  --archive-dir <PATH> (-d)   Path to archive directory
  --batch-size <N>     (-b)   Records per batch (default: 250)

check                        Verify archive integrity
  --archive-dir <PATH> (-d)   Path to archive directory
```

### Enrich Commands

```
enrich-exchange              Add exchange order data
  --exchange-json <PATH>       JSON file with swap records
  --batch-size <N>             Records per batch (default: 250)

enrich-exchange-onramp       Link onboarding addresses to exchange IDs
  --onboarding-json <PATH>     JSON file with onramp mappings

enrich-whitepages            Map account addresses to known owners
  --owner-json <PATH>          JSON file with ownership data
```

### Analytics Commands

```
analytics exchange-rms       Calculate exchange risk management stats
  --persist                    Commit results to database

analytics trades-matching    Match on-chain deposits to exchange trades
  --start-day <DATE>           Start date (exclusive) YYYY-MM-DD
  --end-day <DATE>             End date (exclusive) YYYY-MM-DD
  --replay-balances <N>        Top N accounts for balance replay
  --match-simple-dumps <TOL>   Tolerance for dump matching (>=1.0)
  --clear-cache                Clear local matching cache
```

### Database Commands

```
local-docker-db              Start local Neo4j via Docker
  --data-dir <PATH>            Data directory (default: ./neo4j_data)
  --docker-image <IMAGE>       Docker image (default: neo4j:5.12.0)
```

## Neo4j Configuration

### Recommended Docker Settings

For large datasets, increase Neo4j memory when running Docker manually:

```bash
docker run \
    --publish=7474:7474 --publish=7687:7687 \
    --env NEO4J_AUTH=neo4j/your-password \
    --env NEO4J_dbms_memory_heap_initial__size=4G \
    --env NEO4J_dbms_memory_heap_max__size=8G \
    --env NEO4J_dbms_memory_pagecache_size=4G \
    --volume=/path/to/data:/data \
    neo4j:5.25.1-community
```

### Automatic Indexes

`forensic-db` automatically creates the following indexes and constraints on first run:

```cypher
-- Constraints
CREATE CONSTRAINT unique_address IF NOT EXISTS FOR (n:Account) REQUIRE n.address IS UNIQUE
CREATE CONSTRAINT unique_tx_hash IF NOT EXISTS FOR ()-[r:Transfer]-() REQUIRE r.tx_hash IS UNIQUE

-- Indexes
CREATE TEXT INDEX hex_addr IF NOT EXISTS FOR (n:Account) ON (n.address)
CREATE INDEX tx_timestamp IF NOT EXISTS FOR ()-[r:Tx]-() ON (r.block_datetime)
CREATE INDEX tx_function IF NOT EXISTS FOR ()-[r:Tx]-() ON (r.tx_hash)
CREATE INDEX swap_account_id IF NOT EXISTS FOR (n:SwapAccount) ON (n.swap_id)
CREATE INDEX swap_time IF NOT EXISTS FOR ()-[r:Swap]-() ON (r.filled_at)
CREATE INDEX snapshot_account_id IF NOT EXISTS FOR (n:Snapshot) ON (n.address, n.epoch, n.version)
```

You do not need to create these manually.

## Performance Tuning

### Batch Size

Adjust `--batch-size` based on available memory. The default is 250, which is conservative. Increase for faster ingestion if memory allows:

```bash
libra-forensic-db ingest-all --start-path ./archives --batch-size 1000
```

### Parallel Processing

The `--threads` flag controls parallelism for the `version-five-tx` command. For `ingest-all`, parallelism is managed internally.

## Troubleshooting

### Connection Issues

1. Verify Neo4j is running (`docker ps` or check Neo4j Desktop)
2. Check credentials match (env vars or CLI flags)
3. Test network connectivity to the database port (default: 7687)

### Memory Errors

1. Reduce `--batch-size` (try 100)
2. Increase Neo4j heap size (see Docker settings above)
3. Process data in smaller chunks using `ingest-one`

### Performance Issues

1. Indexes are created automatically - verify with `SHOW INDEXES` in Neo4j Browser
2. Reduce log verbosity (`RUST_LOG=warn`)
3. Use a local database instance rather than remote
