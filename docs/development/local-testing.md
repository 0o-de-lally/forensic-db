
# Local Testing

## From Docker (Built-in)

The easiest way to start a local database for testing is using the built-in subcommand:

```bash
cargo run -- local-docker-db
```

This will:
- Start a Neo4j container (default `neo4j:5.12.0`).
- Persist data to `./neo4j_data` (so you don't lose data between runs).
- Expose ports `7474` (HTTP) and `7687` (Bolt).

You can then run other commands against it using default credentials:

```bash
cargo run -- ingest-all --start-path ./tests/fixtures/
```

### Customization

You can customize the data directory and image:

```bash
cargo run -- local-docker-db --data-dir /tmp/my-neo4j-data --docker-image neo4j:latest
```

## From Neo4j Desktop
Start a Neo4j instance. Choose a password `<password>`. Allow it to create the default database namespace `neo4j`.

```
export LIBRA_GRAPH_DB_URI='neo4j://localhost'
export LIBRA_GRAPH_DB_USER='neo4j'
export LIBRA_GRAPH_DB_PASS=<password>

# optionally export trace logging
export RUST_LOG=trace
```

Import the sample exchange orders

```
cargo r enrich-exchange --exchange-json ./tests/fixtures/savedOlOrders2.json
```

## View graph

Go to Neo4j Explorer and try:
```
MATCH ()-[r:Swap]->()
RETURN COUNT(DISTINCT(r))
```

Should return `25450`

# Testing offline analytics
NOTE: you must have a fully populated DB to run these queries

Replay the funding requirement on an exchange and match to deposits. This is slow.
```
cargo r analytics trades-matching --start-day 2024-01-07 --end-day 2024-01-15 --replay-balances 10

```

Match simple dumps
```
cargo r analytics trades-matching --start-day 2024-01-07 --end-day 2024-01-15 --match-simple-dumps 1.01
```
