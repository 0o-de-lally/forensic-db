# API Reference

Public modules and key functions in the `libra-forensic-db` crate. For full generated documentation, run `cargo doc --open`.

## Crate Structure

All modules are publicly exported from `src/lib.rs`:

```
libra_forensic_db
├── scan                  # Archive discovery and manifest parsing
├── load                  # High-level ingestion orchestration
├── extract_transactions  # Transaction extraction from archives
├── extract_snapshot      # Account state snapshot extraction
├── extract_exchange_orders # Exchange order JSON parsing
├── load_tx_cypher        # Transaction Cypher generation and batch loading
├── load_account_state    # Account state batch loading
├── load_exchange_orders  # Exchange order batch loading
├── enrich_exchange_onboarding # Exchange onramp mapping
├── enrich_whitepages     # Account ownership mapping
├── json_rescue_v5_extract # Legacy V5 JSON extraction
├── json_rescue_v5_load   # Legacy V5 loading
├── neo4j_init            # Database connection and index setup
├── queue                 # Processing queue management
├── schema_transaction    # Transaction data structures
├── schema_account_state  # Account state data structures
├── schema_exchange_orders # Exchange order data structures
├── analytics/            # Analytics submodules
├── warehouse_cli         # CLI definitions
└── util                  # Common helpers
```

## Core Modules

### `scan` - Archive Discovery

Scans directories for archive bundles and parses manifest files.

```rust
/// A map of directory paths to their corresponding manifest information.
pub struct ArchiveMap(pub BTreeMap<PathBuf, ManifestInfo>);

/// Metadata about a Libra blockchain archive.
pub struct ManifestInfo {
    pub archive_dir: PathBuf,
    pub archive_id: String,
    pub version: FrameworkVersion,
    pub contents: BundleContent,
    pub processed: bool,
}

/// Supported framework versions.
pub enum FrameworkVersion { Unknown, V5, V6, V7 }

/// Types of data bundles found in archives.
pub enum BundleContent { Unknown, StateSnapshot, Transaction, EpochEnding }

/// Scan a directory tree for archive bundles, optionally filtering by content type.
pub fn scan_dir_archive(
    start_path: &Path,
    content_filter: Option<BundleContent>,
) -> Result<ArchiveMap>
```

### `load` - Ingestion Orchestration

Top-level functions that coordinate extraction, transformation, and loading.

```rust
/// Process all archives from an ArchiveMap sequentially.
/// Uses the queue system to track progress and skip already-processed archives.
pub async fn ingest_all(
    archive_map: &ArchiveMap,
    pool: &Graph,
    force_queue: bool,     // if true, clears and rebuilds the queue
    batch_size: usize,     // records per Neo4j batch (default: 250)
) -> Result<()>

/// Load a single archive based on its manifest type (transaction or snapshot).
pub async fn try_load_one_archive(
    man: &ManifestInfo,
    pool: &Graph,
    batch_size: usize,
) -> Result<BatchTxReturn>
```

### `neo4j_init` - Database Setup

Database connection management and index initialization.

```rust
/// Environment variable names for database credentials.
pub static URI_ENV: &str = "LIBRA_GRAPH_DB_URI";
pub static USER_ENV: &str = "LIBRA_GRAPH_DB_USER";
pub static PASS_ENV: &str = "LIBRA_GRAPH_DB_PASS";

/// Read credentials from environment variables.
pub fn get_credentials_from_env() -> Result<(String, String, String)>

/// Connect to a local Neo4j instance.
pub async fn get_neo4j_localhost_pool(port: u16) -> Result<Graph>

/// Connect to a remote Neo4j instance.
pub async fn get_neo4j_remote_pool(uri: &str, user: &str, pass: &str) -> Result<Graph>

/// Create constraints and indexes if they don't exist.
/// Called automatically before ingestion.
pub async fn maybe_create_indexes(graph: &Graph) -> Result<()>
```

## Extraction Modules

### `extract_transactions`

```rust
/// Extract all transactions and events from an archive path.
/// Reads `transaction.manifest` and processes each chunk.
/// Returns (user_transactions, events).
pub async fn extract_current_transactions(
    archive_path: &Path,
    framework_version: &FrameworkVersion,
) -> Result<(Vec<WarehouseTxMaster>, Vec<WarehouseEvent>)>
```

### `extract_snapshot`

```rust
/// Extract account states from a V5 framework snapshot.
pub async fn extract_v5_snapshot(
    archive_path: &Path,
) -> Result<Vec<WarehouseAccState>>

/// Extract account states from a V6/V7 framework snapshot.
pub async fn extract_current_snapshot(
    archive_path: &Path,
) -> Result<Vec<WarehouseAccState>>
```

### `extract_exchange_orders`

```rust
/// Parse exchange order JSON files.
pub fn deserialize_orders(json_data: &str) -> Result<Vec<ExchangeOrder>>
```

### `json_rescue_v5_extract`

```rust
/// Extract V5 JSON rescue data from a directory.
pub fn extract_v5_json_rescue(
    archive_path: &Path,
) -> Result<Vec<WarehouseTxMaster>>

/// Decompress a .tgz file to a temporary directory.
pub fn decompress_to_temppath(tgz_file: &Path) -> Result<TempPath>

/// List all JSON files in a directory.
pub fn list_all_json_files(search_dir: &Path) -> Result<Vec<PathBuf>>

/// List all .tgz archives in a directory.
pub fn list_all_tgz_archives(search_dir: &Path) -> Result<Vec<PathBuf>>
```

## Loading Modules

### `load_tx_cypher`

```rust
/// Process a batch of transactions, loading them into Neo4j.
/// Manages queue status (pending -> complete) per batch.
pub async fn tx_batch(
    txs: &[WarehouseTxMaster],
    events: &[WarehouseEvent],
    pool: &Graph,
    batch_size: usize,
    archive_id: &str,
) -> Result<BatchTxReturn>

/// Execute a batch insert of transactions into Neo4j using UNWIND.
pub async fn impl_batch_tx_insert(
    pool: &Graph,
    batch_txs: &[WarehouseTxMaster],
) -> Result<(u64, u64)>
```

### `load_account_state`

```rust
/// Process account state snapshots in batches.
pub async fn snapshot_batch(
    snaps: &[WarehouseAccState],
    pool: &Graph,
    batch_size: usize,
    archive_id: &str,
) -> Result<()>

/// Execute a batch insert of account states into Neo4j.
pub async fn impl_batch_snapshot_insert(
    pool: &Graph,
    batch: &[WarehouseAccState],
) -> Result<u64>
```

### `load_exchange_orders`

```rust
/// Process exchange orders in batches from a JSON file.
pub async fn load_from_json(
    json_path: &Path,
    pool: &Graph,
    batch_size: usize,
) -> Result<(u64, u64)>  // (merged, ignored)

/// Execute a batch insert of exchange orders.
pub async fn impl_batch_tx_insert(
    pool: &Graph,
    batch_txs: &[ExchangeOrder],
) -> Result<(u64, u64)>
```

## Enrichment Modules

### `enrich_whitepages`

```rust
/// Account ownership metadata.
pub struct Whitepages {
    address: Option<AccountAddress>,
    owner: Option<String>,
    address_note: Option<String>,
}

impl Whitepages {
    /// Parse a JSON file containing whitepages data.
    pub fn parse_json_file(path: &Path) -> Result<Vec<Self>>
}

/// Batch insert whitepages data into Neo4j.
pub async fn impl_batch_tx_insert(
    pool: &Graph,
    wp: &[Whitepages],
) -> Result<u64>
```

### `enrich_exchange_onboarding`

```rust
/// Exchange on-ramp mapping (on-chain address to exchange user ID).
pub struct ExchangeOnRamp {
    onramp_address: Option<AccountAddress>,
    user_id: u64,
}

impl ExchangeOnRamp {
    /// Parse a JSON file containing on-ramp mappings.
    pub fn parse_json_file(path: &Path) -> Result<Vec<Self>>
}

/// Batch insert on-ramp data into Neo4j.
pub async fn impl_batch_tx_insert(
    pool: &Graph,
    wp: &[ExchangeOnRamp],
) -> Result<u64>
```

## Queue Module

### `queue`

Manages a processing queue stored in Neo4j for resumable loading.

```rust
/// Update the status of a task in the queue.
pub async fn update_task(
    pool: &Graph,
    archive_id: &str,
    batch_id: &str,
    status: &str,
) -> Result<()>

/// Get all queued (incomplete) archive IDs.
pub async fn get_queued(pool: &Graph) -> Result<Vec<String>>

/// Check if all batches for an archive are complete.
pub async fn are_all_completed(pool: &Graph, archive_id: &str) -> Result<bool>

/// Clear all queue entries.
pub async fn clear_queue(pool: &Graph) -> Result<()>

/// Populate the queue from an ArchiveMap.
pub async fn push_queue_from_archive_map(
    map: &ArchiveMap,
    pool: &Graph,
) -> Result<()>
```

## Analytics Modules

### `analytics::exchange_stats`

```rust
/// Run RMS analytics across all exchange users concurrently.
pub async fn query_rms_analytics_concurrent(
    pool: &Graph,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    persist: bool,
) -> Result<Vec<serde_json::Value>>
```

### `analytics::offline_matching`

```rust
/// Matching engine for correlating on-chain deposits with exchange trades.
pub struct Matching {
    pub definite: Vec<serde_json::Value>,
    // ... internal fields
}

impl Matching {
    pub fn new() -> Self
    pub async fn depth_search_by_top_n_accounts(&mut self, ...) -> Result<()>
    pub async fn search_dumps(&mut self, ...) -> Result<()>
    pub fn write_cache_to_file(&self, dir: &Path) -> Result<()>
    pub fn read_cache_from_file(dir: &Path) -> Result<Self>
    pub fn clear_cache(dir: &Path) -> Result<()>
    pub fn write_definite_to_file(&self, dir: &Path) -> Result<()>
}
```

## Data Structures

### `schema_transaction`

```rust
/// Core transaction record for the warehouse.
pub struct WarehouseTxMaster {
    // Contains: sender, recipient, tx_hash, function, amount,
    // block_datetime, block_timestamp, epoch, round, etc.
}

/// Transaction event data.
pub struct WarehouseEvent { /* event metadata */ }

/// Relationship labels for the graph.
pub enum RelationLabel { Tx, Swap, Owns, OnRamp, /* ... */ }
```

### `schema_account_state`

```rust
/// Account state snapshot record.
pub struct WarehouseAccState {
    // Contains: address, balance, slow_wallet fields,
    // epoch, version, timestamp, etc.
}
```

### `schema_exchange_orders`

```rust
/// Off-chain exchange order record.
pub struct ExchangeOrder {
    // Contains: user, accepter, orderType, amount,
    // price, created_at, filled_at, etc.
}
```

## Utility Modules

### `unzip_temp`

```rust
/// Handle gzipped archives: decompress if needed, return path and temp handle.
pub fn maybe_handle_gz(archive_path: &Path) -> Result<(PathBuf, Option<TempPath>)>

/// Decompress a tar archive.
pub fn decompress_tar_archive(src_path: &Path, dst_dir: &Path) -> Result<()>

/// Decompress all .gz files in a directory.
pub fn decompress_all_gz(parent_dir: &Path, dst_dir: &Path) -> Result<()>
```

### `warehouse_cli`

```rust
/// CLI entry point. Parses args and dispatches to subcommands.
pub struct WarehouseCli { /* clap-derived fields */ }

impl WarehouseCli {
    pub async fn run(&self) -> Result<()>
}

/// Establish a Neo4j connection pool from CLI args or env vars.
pub async fn try_db_connection_pool(cli: &WarehouseCli) -> Result<Graph>
```
