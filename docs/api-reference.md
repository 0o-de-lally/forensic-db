# API Reference

Generated documentation for forensic-db Rust modules.

## Core Modules

### lib.rs

Main library entry point containing public API exports.

### main.rs

CLI application entry point with command registration.

## Extraction Modules

### extract_transactions.rs

```rust
pub async fn extract_transactions(
    path: &Path,
    batch_size: usize
) -> Result<Vec<Transaction>, Error>
```

Extracts transaction records from backup archives.

### extract_snapshot.rs

```rust
pub async fn extract_snapshot(
    path: &Path,
    version: u64
) -> Result<AccountState, Error>
```

Extracts account state snapshots at a specific version.

### extract_exchange_orders.rs

```rust
pub fn extract_exchange_orders(
    json_path: &Path
) -> Result<Vec<ExchangeOrder>, Error>
```

Parses exchange order JSON files.

### json_rescue_v5_extract.rs

```rust
pub async fn extract_v5_json(
    path: &Path
) -> Result<V5Data, Error>
```

Handles V5 JSON rescue extraction.

## Loading Modules

### load.rs

```rust
pub async fn load_cypher(
    session: &Session,
    cypher: &str,
    params: &Map
) -> Result<Summary, Error>
```

Executes Cypher queries against the database.

### load_account_state.rs

```rust
pub async fn load_account_state(
    state: &AccountState,
    session: &Session
) -> Result<(), Error>
```

Loads account state into the graph.

### load_tx_cypher.rs

```rust
pub fn generate_tx_cypher(
    tx: &Transaction
) -> String
```

Generates Cypher query for transaction loading.

### load_exchange_orders.rs

```rust
pub async fn load_exchange_orders(
    orders: &[ExchangeOrder],
    session: &Session
) -> Result<(), Error>
```

Loads exchange orders into the graph.

## Enrichment Modules

### enrich_exchange_onboarding.rs

```rust
pub async fn enrich_onboarding(
    data: &OnboardingData,
    session: &Session
) -> Result<(), Error>
```

Adds exchange onboarding data to the graph.

### enrich_whitepages.rs

```rust
pub async fn enrich_whitepages(
    data: &WhitepagesData,
    session: &Session
) -> Result<(), Error>
```

Enriches accounts with whitepages data.

## Analytics Modules

Located in `src/analytics/` directory.

### Warehouse CLI

### warehouse_cli.rs

```rust
pub async fn run_analytics(
    config: &AnalyticsConfig
) -> Result<AnalyticsResult, Error>
```

Executes analytics workflows.

## Support Modules

### neo4j_init.rs

```rust
pub async fn initialize_db(
    uri: &str,
    user: &str,
    pass: &str
) -> Result<DatabaseConnection, Error>
```

Initializes Neo4j database connection.

### queue.rs

```rust
pub struct WorkQueue {
    // Queue for managing async work
}

impl WorkQueue {
    pub async fn push(&self, item: WorkItem) -> Result<(), Error>
    pub async fn pop(&self) -> Result<WorkItem, Error>
}
```

Manages asynchronous work queues.

### scan.rs

```rust
pub async fn scan_graph(
    pattern: &str,
    session: &Session
) -> Result<Vec<Path>, Error>
```

Scans graph for matching patterns.
