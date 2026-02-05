# Architecture

Overview of the forensic-db system architecture, design decisions, and engineering tradeoffs.

## Product Vision

Forensic-db is designed to be the "Hubble Telescope" for the Libra blockchain. While standard block explorers provide a view of the *current* state or individual transactions, they lack the ability to perform complex, deep-time forensic analysis across the entire history of the network.

The system ingests raw backup archives (from genesis to present) and transforms them into a property graph. This allows investigators to answer questions like:
- "Find all accounts that received funds from a specific mixer 5 hops away."
- "Identify circular trading patterns indicating wash trading."
- "Reconstruct the flow of funds from a hacked wallet across thousands of transactions."

## System Overview

forensic-db is an ETL (Extract, Transform, Load) system that processes Libra blockchain backup archives into a graph database using Open Cypher.

## Data Flow

```
Backup Archives → Extract → Transform → Load → Graph Database
```

## Engineering Tradeoffs

### 1. Graph Database (Neo4j) vs. Relational (PostgreSQL) vs. Time-Series
**Decision**: Use a Property Graph Database (Neo4j).
- **Tradeoff**: Graph databases are generally slower for simple aggregations (e.g., "sum of all transfers") compared to columnar or relational DBs. They also have higher storage overhead for relationships.
- **Justification**: The core value of forensic analysis lies in traversing relationships (Flow of Funds). Multi-hop queries (A -> B -> C -> D) are exponential in cost in SQL (many JOINs) but linear/constant in Graph DBs (pointer chasing). This native representation of "transfer" as an edge is critical for the product's mission.

### 2. Rust vs. Python/Scripting
**Decision**: Rust for the ETL pipeline.
- **Tradeoff**: Slower development velocity and higher learning curve compared to Python.
- **Justification**:
    - **Performance**: Ingesting terabytes of blockchain history requires maximizing CPU and I/O throughput. Rust's zero-cost abstractions and control over memory layout allow for highly optimized parallel processing.
    - **Safety**: The type system prevents entire classes of runtime errors (null pointers, race conditions) that would be catastrophic in a long-running ingestion process (e.g., crashing after 3 days of loading).
    - **Concurrency**: Rust's ownership model allows for aggressive parallelization of the "Extract" and "Transform" phases without fear of data races.

### 3. Docker vs. Native Database
**Decision**: Provide first-class Docker support for the database backend (`local-docker-db`).
- **Tradeoff**: Slight I/O performance penalty due to virtualization/overlayfs.
- **Justification**: Reproducibility and Developer Experience (DX). Requiring every analyst to install and configure a specific version of Neo4j/Java is a high barrier to entry. Docker ensures a consistent environment matching production specs.

### 4. Batch Loading vs. Transactional Consistency
**Decision**: Use batched writes (`UNWIND` cypher clauses) with optimistic concurrency.
- **Tradeoff**: If a batch fails, we must handle partial state or idempotency manually. Real-time consistency is sacrificed for throughput.
- **Justification**: Inserting nodes one-by-one is orders of magnitude too slow for initial sync. We optimize for "Bulk Import" speed. The system is designed to be idempotent; re-running a batch should result in the same graph state (using `MERGE` instead of `CREATE` where appropriate).

## Core Components

### Extraction Layer

- `extract_transactions.rs` - Transaction record extraction
- `extract_snapshot.rs` - Account state snapshot extraction
- `extract_exchange_orders.rs` - Exchange order extraction
- `json_rescue_v5_extract.rs` - V5 JSON rescue extraction

### Loading Layer

- `load.rs` - Core loading operations
- `load_account_state.rs` - Account state ingestion
- `load_tx_cypher.rs` - Transaction Cypher generation
- `load_exchange_orders.rs` - Exchange order ingestion

### Enrichment Layer

- `enrich_exchange_onboarding.rs` - Exchange onboarding data
- `enrich_whitepages.rs` - Whitepages integration

### Analytics

- `analytics/` - Analytics modules for graph analysis
- `warehouse_cli.rs` - CLI for warehouse operations

### Support Services

- `neo4j_init.rs` - Database initialization
- `queue.rs` - Queue management
- `scan.rs` - Graph scanning utilities
- `batch_tx_type.rs` - Batch transaction type processing
- `decode_entry_function.rs` - Entry function decoding

## Schema

### Node Types

- `Account` - Blockchain accounts
- `Transaction` - On-chain transactions
- `SwapAccount` - Swap-related accounts
- `Owner` - Account ownership

### Relationship Types

- `Tx` - Transaction relationships
- `Swap` - Swap relationships
- `Owns` - Ownership relationships
- `OnRamp` - On-ramp relationships

See [sample CQL queries](sample-cql.md) for example patterns.
