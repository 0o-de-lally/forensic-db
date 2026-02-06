# Source Code Index

## Entry Points
- [src/main.rs](../src/main.rs): CLI binary entry point.
- [src/lib.rs](../src/lib.rs): Library crate root.
- [src/warehouse_cli.rs](../src/warehouse_cli.rs): CLI command definitions and execution logic.

## ETL Core
- [src/scan.rs](../src/scan.rs): Archive discovery and manifest parsing.
- [src/load.rs](../src/load.rs): High-level ingestion orchestration.
- [src/extract_transactions.rs](../src/extract_transactions.rs): Transaction data extraction.
- [src/extract_snapshot.rs](../src/extract_snapshot.rs): State snapshot extraction.
- [src/load_tx_cypher.rs](../src/load_tx_cypher.rs): Neo4j transaction loading logic.
- [src/load_account_state.rs](../src/load_account_state.rs): Neo4j account state loading logic.

## Data Enrichment
- [src/enrich_exchange_onboarding.rs](../src/enrich_exchange_onboarding.rs): Exchange ID mapping.
- [src/enrich_whitepages.rs](../src/enrich_whitepages.rs): Account ownership mapping.
- [src/load_exchange_orders.rs](../src/load_exchange_orders.rs): Off-chain order ingestion.

## Schemas
- [src/schema_transaction.rs](../src/schema_transaction.rs): Transaction record structures.
- [src/schema_account_state.rs](../src/schema_account_state.rs): Account state record structures.
- [src/schema_exchange_orders.rs](../src/schema_exchange_orders.rs): Exchange order record structures.

## Database & Utilities
- [src/neo4j_init.rs](../src/neo4j_init.rs): Database connection and index initialization.
- [src/queue.rs](../src/queue.rs): Task queue management for resumable loading.
- [src/util.rs](../src/util.rs): Common helper functions.
- [src/unzip_temp.rs](../src/unzip_temp.rs): Archive decompression utilities.

## Analytics
- [src/analytics/](../src/analytics/README.md): Higher-level data analysis modules.

[Back to Project Index](project-index.md)
