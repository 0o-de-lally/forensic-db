# forensic-db

[![License](https://img.shields.io/badge/license-NOASSERTION-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)](https://www.rust-lang.org)

An ETL system for processing Libra blockchain backup archives into a Neo4j graph database for forensic analysis and flow-of-funds investigation.

## Overview

`forensic-db` ingests raw backup archives from the 0L/Libra blockchain (JMT/RocksDB snapshots) and transforms them into a queryable property graph. This enables powerful multi-hop traversals for tracing funds, analyzing trading patterns, and investigating on-chain behavior.

**Key Features:**
- 📊 **Graph-native storage** - Built for relationship traversal (A→B→C→D)
- 🚀 **High-throughput ingestion** - Processes archives offline without RPC bottlenecks
- 🔍 **Open Cypher queries** - Compatible with Neo4j, AWS Neptune, Memgraph
- 🔧 **Enrichment support** - Add off-chain data (exchange orders, ownership mappings)
- 🐳 **Docker-first** - Start a local Neo4j instance with one command

**Use Cases:**
- Flow-of-funds analysis
- Exchange deposit/withdrawal tracking
- Shill trader identification
- Account relationship mapping
- On-chain forensics

## Quick Start

### 1. Install

```bash
cargo build --release
cp ./target/release/libra-forensic-db ~/.cargo/bin/
```

### 2. Start Database

Use the built-in Docker command to launch a local Neo4j instance:

```bash
libra-forensic-db local-docker-db --data-dir ./neo4j_data
```

Or use an existing Neo4j instance (see [Configuration](#configuration)).

### 3. Get Archive Data

Clone the backup archives repository:

```bash
# Clone v6 archives (or v5, v7 depending on your needs)
git clone https://github.com/0LNetworkCommunity/epoch-archive-mainnet \
  --depth 1 --branch v6
```

**Alternative mirrors:** https://github.com/0LNetworkCommunity/libra-archive-mirrors

### 4. Ingest Data

```bash
# Set log level
export RUST_LOG=info

# Ingest transaction archives
libra-forensic-db ingest-all \
  --start-path ./epoch-archive-mainnet \
  --archive-content transaction
```

### 5. Query the Graph

Connect to Neo4j at `http://localhost:7474` and run Cypher queries:

```cypher
// Find all transactions from an account
MATCH (a:Account {address: "0x123..."})-[tx:Tx]->(b:Account)
RETURN a, tx, b
LIMIT 50;

// Trace flow of funds (3 hops)
MATCH path = (a:Account)-[:Tx*1..3]->(b:Account)
WHERE a.address = "0x123..."
RETURN path;
```

See [Sample CQL Queries](docs/technical/sample-cql.md) for more examples.

## Commands

### Core Operations

```bash
# Ingest all archives in a directory
libra-forensic-db ingest-all --start-path <archive-root> --archive-content transaction

# Ingest a single archive
libra-forensic-db ingest-one --archive-dir <path-to-archive>

# Verify archive integrity
libra-forensic-db check --archive-dir <path-to-archive>

# Start local Neo4j with Docker
libra-forensic-db local-docker-db --data-dir ./neo4j_data
```

### Enrichment & Analytics

Add off-chain data or run analysis:

```bash
# Add exchange orders (see docs for JSON schema)
libra-forensic-db enrich-exchange --exchange-json <orders.json>

# Map account ownership
libra-forensic-db enrich-whitepages --owner-json <owners.json>

# Run exchange analytics
libra-forensic-db analytics exchange-rms --persist

# Match trades to deposits
libra-forensic-db analytics trades-matching \
  --start-day 2024-01-01 --end-day 2024-12-31
```

See [API Reference](docs/technical/api-reference.md) for complete command documentation.

## Configuration

### Environment Variables

```bash
export LIBRA_GRAPH_DB_URI='neo4j://localhost:7687'
export LIBRA_GRAPH_DB_USER='neo4j'
export LIBRA_GRAPH_DB_PASS='your-password'
export RUST_LOG=info  # or debug, trace
```

### Command Line

All commands accept database credentials as flags:

```bash
libra-forensic-db \
  --db-uri 'neo4j+s://example.databases.neo4j.io' \
  --db-username 'neo4j' \
  --db-password 'your-password' \
  <subcommand>
```

Additional options:
- `--threads <n>` - Max parallel tasks (default: CPU count)
- `--clear-queue` - Force clear the processing queue

## Architecture

**Why Graph?** Traditional SQL databases require expensive JOINs for multi-hop queries. Graph databases represent transfers as edges, making flow-of-funds analysis orders of magnitude faster.

**Why Rust?** Processing terabytes of BCS-encoded Merkle tree snapshots requires maximum throughput and type safety. Rust's concurrency model enables aggressive parallelization without data races.

**Why Offline Archives?** JSON-RPC is too slow, incomplete (pruned history), and rate-limited for bulk forensic analysis. Backup archives provide the "ground truth" raw bytes for the entire chain.

See [Architecture Deep Dive](docs/technical/architecture.md) for design decisions and tradeoffs.

## Documentation

### User Guides
- [Getting Started](docs/product/getting-started.md) - Detailed setup instructions
- [User Guide](docs/product/user-guide.md) - Complete usage guide

### Technical Docs
- [Architecture](docs/technical/architecture.md) - System design and tradeoffs
- [API Reference](docs/technical/api-reference.md) - Code documentation
- [Sample CQL Queries](docs/technical/sample-cql.md) - Query cookbook
- [Configuration](docs/technical/configuration.md) - Advanced configuration

### Developer Docs
- [Developer Guide](docs/development/developer-guide.md) - Contributing guidelines
- [Source Index](docs/development/source-index.md) - Codebase overview
- [Local Testing](docs/development/local-testing.md) - Testing strategies

## Project Structure

```
forensic-db/
├── src/
│   ├── extract_*.rs      # Extract layer (decode archives)
│   ├── load_*.rs         # Load layer (write to Neo4j)
│   ├── enrich_*.rs       # Enrichment (off-chain data)
│   ├── analytics/        # Analysis modules
│   └── warehouse_cli.rs  # CLI entry point
├── docs/
│   ├── product/          # User-facing guides
│   ├── technical/        # Architecture & specs
│   └── development/      # Developer resources
└── tests/
```

## Contributing

See [Developer Guide](docs/development/developer-guide.md) for contribution guidelines.

## License

NOASSERTION - See repository for details

## Links

- **Archive Repository:** https://github.com/0LNetworkCommunity/epoch-archive-mainnet
- **Archive Mirrors:** https://github.com/0LNetworkCommunity/libra-archive-mirrors
- **0L Network:** https://openlibra.io/
