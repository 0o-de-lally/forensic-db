# Architecture

Overview of the forensic-db system architecture, design decisions, and engineering tradeoffs.

## The Indexing Challenge

Indexing blockchains derived from the Diem/Libra architecture (like Aptos and 0L) presents unique challenges compared to EVM-based chains. These networks do not use standard databases but rely on a specialized data structure known as the **Jellyfish Merkle Tree (JMT)** backed by **RocksDB**.

### Key Difficulties:

1.  **Proprietary Storage Format**: The state is stored in a highly optimized, versioned Merkle tree designed for cryptographic verification, not queryability. There is no SQL interface or standard index to query "all transactions by user X".
2.  **Opaque Data Blobs**: Much of the meaningful data is serialized in Move-specific binary formats (BCS). Decoding this requires access to the exact ABI (Application Binary Interface) of the modules *at that specific block height*, which changes over time as the network upgrades.
3.  **State vs. History**: The JMT is optimized for proving the *current* state of an account. Reconstructing the *history* of an account requires replaying every single transaction from genesis, as intermediate states are often pruned or hard to look up efficiently.

`forensic-db` solves this by bypassing the live node entirely. Instead of querying a JSON-RPC API (which is slow and rate-limited), it ingests the raw **Backup Archives**—the immutable snapshots of the ledger used for disaster recovery. This allows for high-throughput, offline processing of the entire chain history.

## System Overview

forensic-db is an ETL (Extract, Transform, Load) system that processes these Libra/Diem backup archives into a graph database (Neo4j) using Open Cypher.

## Data Flow

```
Backup Archives (JMT/RocksDB snapshots) → Extract (BCS Decoding) → Transform (Graph Model) → Load (Cypher) → Neo4j
```

## Engineering Tradeoffs

### 1. Graph Database (Neo4j) vs. Relational (PostgreSQL) vs. Time-Series
**Decision**: Use a Property Graph Database (Neo4j).
- **Tradeoff**: Graph databases are generally slower for simple aggregations (e.g., "sum of all transfers") compared to columnar or relational DBs. They also have higher storage overhead for relationships.
- **Justification**: The core value of forensic analysis lies in traversing relationships (Flow of Funds). Multi-hop queries (A -> B -> C -> D) are exponential in cost in SQL (many JOINs) but linear/constant in Graph DBs (pointer chasing). This native representation of "transfer" as an edge is critical for tracing funds through mixers or complex laundering schemes.

### 2. Rust vs. Python/Scripting
**Decision**: Rust for the ETL pipeline.
- **Tradeoff**: Slower development velocity and higher learning curve compared to Python.
- **Justification**:
    - **Performance**: Ingesting terabytes of binary ledger history requires maximizing CPU and I/O throughput. Rust's zero-cost abstractions allow for highly optimized parallel processing of BCS-encoded data.
    - **Type Safety**: The Libra/Diem type system is rigorous. Rust's type system mirrors this, preventing entire classes of deserialization errors that would otherwise crash long-running ingestion jobs.
    - **Concurrency**: Rust's ownership model allows for aggressive parallelization of the "Extract" and "Transform" phases without fear of data races.

### 3. Backup Archives vs. JSON-RPC
**Decision**: Ingest from Backup Archives (offline).
- **Tradeoff**: Higher latency (must wait for archives to be generated) and higher complexity (must implement low-level decoding logic).
- **Justification**: JSON-RPC is unsuitable for bulk historical analysis. It is too slow (HTTP overhead), often incomplete (nodes prune history), and rate-limited. Archives provide the "ground truth" raw bytes, allowing us to build a complete, verifiable index of the chain's entire lifecycle.

### 4. Docker vs. Native Database
**Decision**: Provide first-class Docker support for the database backend (`local-docker-db`).
- **Tradeoff**: Slight I/O performance penalty due to virtualization/overlayfs.
- **Justification**: Reproducibility and Developer Experience (DX). Requiring every analyst to install and configure a specific version of Neo4j/Java is a high barrier to entry. Docker ensures a consistent environment matching production specs.

### 5. Batch Loading vs. Transactional Consistency
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
