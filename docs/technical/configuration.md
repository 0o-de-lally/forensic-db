# Configuration Guide

Comprehensive configuration options for forensic-db.

## Environment Variables

### Database Connection

| Variable | Description | Default |
|----------|-------------|---------|
| `LIBRA_GRAPH_DB_URI` | Neo4j connection URI | None (required) |
| `LIBRA_GRAPH_DB_USER` | Database username | `neo4j` |
| `LIBRA_GRAPH_DB_PASS` | Database password | None (required) |

### Logging

| Variable | Description | Values |
|----------|-------------|--------|
| `RUST_LOG` | Log level | `error`, `warn`, `info`, `debug`, `trace` |

## Command Line Options

### Global Options

```
--db-uri <URI>           Database connection URI
--db-username <USER>     Database username
--db-password <PASS>     Database password
--help                   Show help
--version                Show version
```

### Ingest Commands

```
ingest-all              Process all archives
  --start-path <PATH>    Root directory of archives
  --archive-content <TYPE>  Content type to process

ingest                  Process specific archive
  --start-path <PATH>   Archive path
  --content-type <TYPE>  Content type
```

### Enrich Commands

```
enrich-exchange         Add exchange data
  --exchange-json <PATH>  JSON file path

enrich-whitepages       Add whitepages data
  --whitepages-json <PATH>
```

### Analytics Commands

```
analytics trades-matching
  --start-day <DATE>    Start date (YYYY-MM-DD)
  --end-day <DATE>      End date (YYYY-MM-DD)
  --replay-balances <N> Replay balance count
  --match-simple-dumps <RATIO>  Simple dump matching ratio
```

## Neo4j Configuration

### Recommended Settings

```
dbms.memory.heap.initial_size=2G
dbms.memory.heap.max_size=4G
dbms.transaction.log.size=100M
dbms.security.auth_enabled=false  # For local development
```

### Indexes

Create indexes for optimal query performance:

```cypher
CREATE INDEX account_address IF NOT EXISTS FOR (n:Account) ON (n.address)
CREATE INDEX transaction_version IF NOT EXISTS FOR (n:Transaction) ON (n.version)
```

## Performance Tuning

### Batch Size

Adjust batch sizes based on available memory:

```bash
export LIBRA_BATCH_SIZE=10000
```

### Parallel Processing

Set number of parallel workers:

```bash
export LIBRA_WORKERS=4
```

## Troubleshooting

### Connection Issues

1. Verify Neo4j is running
2. Check credentials
3. Test network connectivity

### Memory Errors

1. Reduce batch sizes
2. Increase Neo4j heap size
3. Process data in smaller chunks

### Performance Issues

1. Add database indexes
2. Reduce log verbosity
3. Use local database instance
