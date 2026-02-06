# User Guide

Complete guide to using forensic-db for graph database operations.

## Ingestion Commands

### Ingest All

Scan a directory tree and process all archive bundles found:

```bash
libra-forensic-db ingest-all --start-path <path-to-archive> --archive-content transaction
```

Options:
- `--start-path` (`-d`) - Root directory to scan for archives (required)
- `--archive-content` (`-c`) - Type: `transaction` or `account_state`
- `--batch-size` (`-b`) - Records per batch (default: 250)

### Ingest One

Process a single archive bundle:

```bash
libra-forensic-db ingest-one --archive-dir <path-to-archive>
```

Options:
- `--archive-dir` (`-d`) - Path to specific archive directory (required)
- `--batch-size` (`-b`) - Records per batch (default: 250)

### Check

Verify archive integrity without loading to the database:

```bash
libra-forensic-db check --archive-dir <path-to-archive>
```

### Version Five Transactions

Load legacy v5 transaction data from `.tgz` archives:

```bash
libra-forensic-db version-five-tx --archive-dir <path-to-v5-archives>
```

## Database

### Local Docker DB

Start a local Neo4j instance using Docker (requires Docker to be installed):

```bash
# Start with defaults (data in ./neo4j_data, credentials neo4j/neo4j)
libra-forensic-db local-docker-db

# Start with custom settings
libra-forensic-db local-docker-db --data-dir /path/to/data --docker-image neo4j:latest
```

Options:
- `--data-dir` - Local directory for persistent data (default: `./neo4j_data`)
- `--docker-image` - Neo4j image tag (default: `neo4j:5.12.0`)

## Enrichment Commands

### Enrich Exchange

Add off-chain exchange order data:

```bash
libra-forensic-db enrich-exchange --exchange-json <path-to-json>
```

Expected JSON format:

```json
[
  {
    "user": 1,
    "orderType": "Sell",
    "amount": "40000.000",
    "price": "0.00460",
    "created_at": "2024-05-12T15:25:14.991Z",
    "filled_at": "2024-05-14T15:04:13.000Z",
    "accepter": 3768
  }
]
```

### Enrich Exchange Onramp

Link exchange onboarding addresses to exchange user IDs:

```bash
libra-forensic-db enrich-exchange-onramp --onboarding-json <path-to-json>
```

Expected JSON format:

```json
[
  {
    "user_id": 189,
    "onramp_address": "01F3B9C815FEB654718DE5D53CD665699A2B80951B696939E2D9EC27D0126BAD"
  }
]
```

Note: The address string is flexible - upper or lowercase, with or without `0x` prefix.

### Enrich Whitepages

Map blockchain addresses to known owners:

```bash
libra-forensic-db enrich-whitepages --owner-json <path-to-json>
```

Expected JSON format:

```json
[
  {
    "address": "0xABC123...",
    "owner": "Alice"
  }
]
```

Fields: `address` (hex string), `owner` (name string), `address_note` (optional).

## Analytics Commands

### Exchange RMS

Calculate exchange risk management statistics:

```bash
# Display stats only
libra-forensic-db analytics exchange-rms

# Persist results to the database
libra-forensic-db analytics exchange-rms --persist
```

### Trades Matching

Match on-chain deposits to off-chain exchange trades. Requires at least one of `--replay-balances` or `--match-simple-dumps`:

```bash
# Replay balances for top N accounts (slow, thorough)
libra-forensic-db analytics trades-matching \
    --start-day 2024-01-07 \
    --end-day 2024-01-15 \
    --replay-balances 10

# Match simple dump patterns (fast)
libra-forensic-db analytics trades-matching \
    --start-day 2024-01-07 \
    --end-day 2024-01-15 \
    --match-simple-dumps 1.01

# Clear cached results first
libra-forensic-db analytics trades-matching \
    --start-day 2024-01-07 \
    --end-day 2024-01-15 \
    --replay-balances 10 \
    --clear-cache
```

Results are cached locally and output as JSON. Use `--clear-cache` to reset.

## Logging

Set log level via environment variable:

```bash
export RUST_LOG=info    # Standard logging (recommended)
export RUST_LOG=debug   # Debug output
export RUST_LOG=trace   # Very verbose tracing
```

## Common Issues

### Connection Failures

Verify Neo4j is running and credentials are correct. Check that `LIBRA_GRAPH_DB_URI`, `LIBRA_GRAPH_DB_USER`, and `LIBRA_GRAPH_DB_PASS` environment variables are set, or pass credentials via CLI flags.

### Memory Issues

Reduce `--batch-size` (e.g., to 100) for large datasets on memory-constrained systems.

### Performance

- Use a local Neo4j instance for best performance
- The tool automatically creates indexes on first run
- Batch operations are used internally via `UNWIND` Cypher clauses

See [Local Testing](../development/local-testing.md) for development setup.

See [CLI Reference](../technical/cli-reference.md) for complete command documentation.
