# User Guide

Complete guide to using forensic-db for graph database operations.

## Commands

### Ingest All

Process all archives in a directory:

```bash
libra-forensic-db ingest-all --start-path <path-to-archive> --archive-content transaction
```

### Ingest Specific

Process specific archive types:

```bash
libra-forensic-db ingest --start-path <path> --content-type <type>
```

### Local Docker DB

Start a persistent local Neo4j instance using Docker (requires Docker to be installed):

```bash
# Start with defaults (data in ./neo4j_data)
libra-forensic-db local-docker-db

# Start with custom settings
libra-forensic-db local-docker-db --data-dir /path/to/data --docker-image neo4j:latest
```

### Enrich Exchange

Add exchange transaction data:

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

### Analytics

Run analytics queries:

```bash
libra-forensic-db analytics trades-matching \
    --start-day 2024-01-07 \
    --end-day 2024-01-15 \
    --replay-balances 10
```

### Scan

Scan the graph database:

```bash
libra-forensic-db scan <options>
```

## Logging

Set log level via environment variable:

```bash
export RUST_LOG=info    # Standard logging
export RUST_LOG=trace   # Detailed tracing
export RUST_LOG=debug   # Debug output
```

## Common Issues

### Connection Failures

Verify Neo4j is running and credentials are correct.

### Memory Issues

Process archives in smaller batches for large datasets.

### Performance

- Use local Neo4j instance for best performance
- Index frequently queried fields
- Batch operations where possible

See [local testing](local-testing.md) for development setup.
