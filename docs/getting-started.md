# Getting Started

A comprehensive guide to setting up and running the forensic-db ETL system.

## Prerequisites

- Rust 1.70+ and Cargo
- Neo4j 5.x (local or Docker)
- Git
- Unzip utility

## Installation

### From Source

```bash
cargo build --release
cp ./target/release/libra-forensic-db ~/.cargo/bin/
```

### Using Docker Neo4j

```bash
docker run \
    --restart always \
    --publish=7474:7474 --publish=7687:7687 \
    --env NEO4J_AUTH=none \
    --volume=/path/to/your/data:/data \
    neo4j:5.25.1-community
```

## Configuration

### Environment Variables

```bash
export LIBRA_GRAPH_DB_URI='neo4j://localhost'
export LIBRA_GRAPH_DB_USER='neo4j'
export LIBRA_GRAPH_DB_PASS='your-password'
export RUST_LOG=info
```

### Command Line Arguments

```bash
libra-forensic-db --db-uri 'neo4j+s://example.databases.neo4j.io' \
    --db-username 'neo4j' \
    --db-password 'your-password' \
    <subcommand>
```

## Quick Start

1. Clone the backup archive repository
2. Unzip all files
3. Run the ingestion process
4. Explore the graph

See [README](../README.md) for detailed instructions.
