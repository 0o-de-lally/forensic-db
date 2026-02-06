# CLI Reference

Complete command-line interface documentation for `libra-forensic-db`.

## Global Options

These options apply to all subcommands:

```bash
libra-forensic-db [GLOBAL_OPTIONS] <SUBCOMMAND> [SUBCOMMAND_OPTIONS]
```

### Database Connection

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--db-uri` | `-r` | String | URI of graph database (e.g., `neo4j+s://localhost:7687`) |
| `--db-username` | `-u` | String | Database username |
| `--db-password` | `-p` | String | Database password |

**Environment Variables:**
- `LIBRA_GRAPH_DB_URI` - Default database URI
- `LIBRA_GRAPH_DB_USER` - Default username
- `LIBRA_GRAPH_DB_PASS` - Default password

### Performance Options

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--threads` | `-t` | Number | Maximum parallel tasks (default: CPU count) |
| `--clear-queue` | `-q` | Flag | Force clear processing queue |

### Logging

Set via `RUST_LOG` environment variable:
- `error` - Only errors
- `warn` - Warnings and errors
- `info` - Informational messages (recommended)
- `debug` - Detailed debug output
- `trace` - Very verbose output

```bash
export RUST_LOG=info
```

## Commands

### `ingest-all`

Scan a directory tree and ingest all archive bundles found.

**Usage:**
```bash
libra-forensic-db ingest-all \
  --start-path <PATH> \
  [--archive-content <TYPE>] \
  [--batch-size <N>]
```

**Options:**

| Option | Short | Type | Required | Description |
|--------|-------|------|----------|-------------|
| `--start-path` | `-d` | Path | Yes | Root directory to scan for archives |
| `--archive-content` | `-c` | Enum | No | Type of archives to process: `transaction` or `account_state` |
| `--batch-size` | `-b` | Number | No | Number of records per batch (default: 1000) |

**Examples:**

```bash
# Ingest all transaction archives
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content transaction

# Ingest with larger batches for performance
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content transaction \
  --batch-size 5000

# Ingest account state snapshots
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content account_state
```

**Behavior:**
- Recursively scans `start-path` for archive directories
- Processes archives in parallel (controlled by `--threads`)
- Skips already-processed archives (idempotent)
- Creates a processing queue for fault tolerance

---

### `ingest-one`

Process and load a single archive bundle.

**Usage:**
```bash
libra-forensic-db ingest-one \
  --archive-dir <PATH> \
  [--batch-size <N>]
```

**Options:**

| Option | Short | Type | Required | Description |
|--------|-------|------|----------|-------------|
| `--archive-dir` | `-d` | Path | Yes | Path to specific archive directory |
| `--batch-size` | `-b` | Number | No | Number of records per batch (default: 1000) |

**Examples:**

```bash
# Process a single epoch
libra-forensic-db ingest-one \
  --archive-dir ./epoch-archives/transaction/0100-0199

# Process with custom batch size
libra-forensic-db ingest-one \
  --archive-dir ./epoch-archives/transaction/0100-0199 \
  --batch-size 2000
```

**Use Cases:**
- Testing/debugging a specific archive
- Re-processing a failed archive
- Selective data loading

---

### `check`

Verify archive integrity without loading to database.

**Usage:**
```bash
libra-forensic-db check --archive-dir <PATH>
```

**Options:**

| Option | Short | Type | Required | Description |
|--------|-------|------|----------|-------------|
| `--archive-dir` | `-d` | Path | Yes | Path to archive directory to verify |

**Examples:**

```bash
# Verify archive can be decoded
libra-forensic-db check --archive-dir ./epoch-archives/transaction/0000-0099
```

**Validation:**
- Checks manifest file exists and is valid
- Verifies archive files can be opened
- Attempts to decode BCS data
- Reports any errors without modifying database

---

### `local-docker-db`

Start a local Neo4j instance using Docker.

**Usage:**
```bash
libra-forensic-db local-docker-db \
  [--data-dir <PATH>] \
  [--docker-image <IMAGE>]
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--data-dir` | Path | `./neo4j_data` | Local directory to persist data |
| `--docker-image` | String | `neo4j:5.12.0` | Docker image tag to use |

**Examples:**

```bash
# Start with defaults
libra-forensic-db local-docker-db

# Specify custom data directory
libra-forensic-db local-docker-db --data-dir /mnt/fast-ssd/neo4j

# Use specific Neo4j version
libra-forensic-db local-docker-db --docker-image neo4j:5.25.1-community
```

**Behavior:**
- Pulls Docker image if not present
- Starts container on ports 7474 (HTTP) and 7687 (Bolt)
- Uses `NEO4J_AUTH=none` (no authentication)
- Persists data to `--data-dir`
- Restarts automatically on reboot

**Access:**
- Neo4j Browser: http://localhost:7474
- Bolt protocol: neo4j://localhost:7687

---

### `enrich-exchange`

Load off-chain exchange order data into the graph.

**Usage:**
```bash
libra-forensic-db enrich-exchange \
  --exchange-json <FILE> \
  [--batch-size <N>]
```

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--exchange-json` | Path | Yes | JSON file with exchange orders |
| `--batch-size` | Number | No | Records per batch (default: 1000) |

**JSON Schema:**

```json
[
  {
    "user": 1,
    "orderType": "Sell",
    "amount": "40000.000",
    "price": "0.00460",
    "created_at": "2024-05-12T15:25:14.991Z",
    "filled_at": "2024-05-14T15:04:13.000Z",
    "accepter": 3768
  }
]
```

**Fields:**
- `user` (Number) - User ID of order creator
- `orderType` (String) - "Buy" or "Sell"
- `amount` (String) - Order amount (decimal)
- `price` (String) - Order price (decimal)
- `created_at` (ISO8601) - Order creation timestamp
- `filled_at` (ISO8601) - Order fill timestamp
- `accepter` (Number) - User ID of order taker

**Examples:**

```bash
# Load exchange orders
libra-forensic-db enrich-exchange \
  --exchange-json ./exchange-orders.json

# Load with larger batches
libra-forensic-db enrich-exchange \
  --exchange-json ./orders.json \
  --batch-size 2000
```

**Graph Schema:**
- Creates `SwapAccount` nodes for users
- Creates `Swap` relationships between users
- Links to on-chain `Account` nodes where possible

---

### `enrich-exchange-onramp`

Map exchange onboarding addresses to exchange user IDs.

**Usage:**
```bash
libra-forensic-db enrich-exchange-onramp \
  --onboarding-json <FILE>
```

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--onboarding-json` | Path | Yes | JSON file with onramp mappings |

**JSON Schema:**

```json
[
  {
    "user_id": 1,
    "onboard_address": "0xABC123..."
  }
]
```

**Examples:**

```bash
libra-forensic-db enrich-exchange-onramp \
  --onboarding-json ./onboarding.json
```

**Use Case:**
Links initial deposit addresses to exchange accounts, enabling flow-of-funds analysis from on-chain deposits to off-chain trades.

---

### `enrich-whitepages`

Map blockchain addresses to known owners/entities.

**Usage:**
```bash
libra-forensic-db enrich-whitepages \
  --owner-json <FILE>
```

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--owner-json` | Path | Yes | JSON file with ownership mappings |

**JSON Schema:**

```json
[
  {
    "address": "0xABC123...",
    "owner": "Alice",
    "entity_type": "Individual",
    "notes": "Core contributor"
  }
]
```

**Examples:**

```bash
libra-forensic-db enrich-whitepages \
  --owner-json ./known-owners.json
```

**Graph Schema:**
- Creates `Owner` nodes for each entity
- Creates `Owns` relationships to `Account` nodes
- Adds metadata properties (entity_type, notes)

---

### `version-five-tx`

Load legacy v5 transaction data from `.tgz` archives.

**Usage:**
```bash
libra-forensic-db version-five-tx \
  --archive-dir <PATH>
```

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--archive-dir` | Path | Yes | Root directory containing v5 `.tgz` files |

**Examples:**

```bash
libra-forensic-db version-five-tx \
  --archive-dir ./v5-archives
```

**Note:** This command is for historical v5 data only. Use `ingest-all` for v6/v7.

---

### `analytics exchange-rms`

Calculate exchange risk management statistics.

**Usage:**
```bash
libra-forensic-db analytics exchange-rms \
  [--persist]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--persist` | Flag | Write results to database (default: print only) |

**Examples:**

```bash
# Calculate and display stats
libra-forensic-db analytics exchange-rms

# Calculate and persist to DB
libra-forensic-db analytics exchange-rms --persist
```

**Calculated Metrics:**
- Total exchange volume
- Risk exposure by user
- Deposit/withdrawal patterns
- Account balance distributions

---

### `analytics trades-matching`

Match on-chain deposits to off-chain exchange trades.

**Usage:**
```bash
libra-forensic-db analytics trades-matching \
  --start-day <YYYY-MM-DD> \
  --end-day <YYYY-MM-DD> \
  [--replay-balances <N>] \
  [--match-simple-dumps <TOLERANCE>] \
  [--clear-cache]
```

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--start-day` | Date | Yes | Start date (exclusive) in YYYY-MM-DD format |
| `--end-day` | Date | Yes | End date (exclusive) in YYYY-MM-DD format |
| `--replay-balances` | Number | No | Top N accounts for slow balance replay |
| `--match-simple-dumps` | Float | No | Tolerance for simple dump matching (≥1.0) |
| `--clear-cache` | Flag | No | Clear local matching cache before running |

**Examples:**

```bash
# Match trades in date range
libra-forensic-db analytics trades-matching \
  --start-day 2024-01-01 \
  --end-day 2024-12-31

# Find exact deposit matches (simple dumps)
libra-forensic-db analytics trades-matching \
  --start-day 2024-06-01 \
  --end-day 2024-06-30 \
  --match-simple-dumps 1.0

# Replay balances for top 100 accounts
libra-forensic-db analytics trades-matching \
  --start-day 2024-01-01 \
  --end-day 2024-12-31 \
  --replay-balances 100

# Clear cache and re-run
libra-forensic-db analytics trades-matching \
  --start-day 2024-01-01 \
  --end-day 2024-12-31 \
  --clear-cache
```

**Algorithms:**

1. **Simple Dumps** (`--match-simple-dumps`):
   - Finds on-chain deposits that match exchange trades within tolerance
   - Example: Deposit of 10,000 coins → Trade of 10,000 coins within 24h
   - Tolerance accounts for fees (e.g., 1.01 = 1% tolerance)

2. **Balance Replay** (`--replay-balances`):
   - Reconstructs account balances day-by-day
   - Identifies likely funding sources for trades
   - Slower but more comprehensive

**Output:**
- Creates `MatchedDeposit` relationships
- Adds `confidence` score property
- Stores matching metadata

---

## Common Workflows

### Initial Setup and Full Ingestion

```bash
# 1. Start database
libra-forensic-db local-docker-db --data-dir ./neo4j_data

# 2. Ingest all transaction data
export RUST_LOG=info
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content transaction

# 3. Ingest account states
libra-forensic-db ingest-all \
  --start-path ./epoch-archives \
  --archive-content account_state

# 4. Add exchange data
libra-forensic-db enrich-exchange --exchange-json ./orders.json
libra-forensic-db enrich-exchange-onramp --onboarding-json ./onramp.json

# 5. Add ownership data
libra-forensic-db enrich-whitepages --owner-json ./owners.json

# 6. Run analytics
libra-forensic-db analytics exchange-rms --persist
libra-forensic-db analytics trades-matching \
  --start-day 2024-01-01 \
  --end-day 2024-12-31
```

### Selective Ingestion

```bash
# Process only specific epochs
libra-forensic-db ingest-one --archive-dir ./epoch-archives/transaction/0100-0199
libra-forensic-db ingest-one --archive-dir ./epoch-archives/transaction/0200-0299

# Verify before loading
libra-forensic-db check --archive-dir ./epoch-archives/transaction/0300-0399
```

### Re-processing Failed Archives

```bash
# Clear queue and retry
libra-forensic-db --clear-queue ingest-all \
  --start-path ./epoch-archives \
  --archive-content transaction
```

### Development/Testing

```bash
# Start test database
libra-forensic-db local-docker-db --data-dir ./test_neo4j

# Load small dataset
libra-forensic-db ingest-one \
  --archive-dir ./epoch-archives/transaction/0000-0099 \
  --batch-size 100

# Check results with single-threaded execution for debugging
libra-forensic-db --threads 1 ingest-one \
  --archive-dir ./epoch-archives/transaction/0000-0099
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Connection error (database) |
| 3 | Invalid arguments |
| 4 | Archive error (missing/corrupt) |

## Performance Tips

### Optimize Ingestion Speed

1. **Increase batch size** for high-memory systems:
   ```bash
   --batch-size 5000
   ```

2. **Control parallelism** based on bottleneck:
   - CPU-bound: Use all cores (`--threads $(nproc)`)
   - I/O-bound: Reduce threads to avoid contention (`--threads 4`)
   - Memory-bound: Reduce batch size and threads

3. **Use local SSD** for Neo4j data directory:
   ```bash
   libra-forensic-db local-docker-db --data-dir /mnt/nvme/neo4j
   ```

4. **Increase Neo4j memory** (see Getting Started guide)

### Monitor Progress

```bash
# Enable detailed logging
export RUST_LOG=debug
libra-forensic-db ingest-all --start-path ./epoch-archives

# Watch Neo4j logs (Docker)
docker logs -f <container-id>
```

## Troubleshooting

### Command Not Found

```bash
# Ensure binary is in PATH
echo $PATH | grep -q "$HOME/.cargo/bin" || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Permission Denied (Docker)

```bash
# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker
```

### Database Connection Timeout

```bash
# Test connection with explicit credentials
libra-forensic-db \
  --db-uri neo4j://localhost:7687 \
  --db-username neo4j \
  --db-password test \
  analytics exchange-rms
```

## See Also

- [Getting Started](../product/getting-started.md) - Setup guide
- [User Guide](../product/user-guide.md) - Usage examples
- [Architecture](architecture.md) - System design
- [Sample CQL](sample-cql.md) - Query examples
