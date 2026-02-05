# Architecture

Overview of the forensic-db system architecture and component design.

## System Overview

forensic-db is an ETL (Extract, Transform, Load) system that processes Libra blockchain backup archives into a graph database using Open Cypher.

## Data Flow

```
Backup Archives → Extract → Transform → Load → Graph Database
```

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
