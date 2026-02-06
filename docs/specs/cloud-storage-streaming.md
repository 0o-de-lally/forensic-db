# Cloud Storage Streaming ETL

**Status:** Proposed
**Author:** forensic-db contributors
**Created:** 2026-02-06
**Target Version:** 0.2.0

## Problem Statement

### Current Limitations

The 0L blockchain backup archives are extremely large and growing:
- **v5 archives:** ~10 GB compressed
- **v6 archives:** ~50 GB compressed
- **v7 archives:** ~100 GB compressed
- **Future versions:** Expected to exceed 200+ GB

Currently, `forensic-db` requires users to:
1. Clone the entire Git LFS repository (or download archives)
2. Store archives locally on disk
3. Manually unzip compressed archives
4. Run the ETL process

**Pain Points:**

1. **Disk Space Requirements:** Users need 2-3x the archive size:
   - Original compressed archives
   - Uncompressed data for processing
   - Working space for ETL operations
   - Example: v7 requires 200-300 GB local storage

2. **Download Time:** Cloning 100+ GB repositories takes hours or days depending on bandwidth

3. **Redundant Storage:** Multiple users analyzing the same chain duplicate storage across machines

4. **Selective Analysis Blocked:** Users must download entire archives even if they only need specific epochs or transaction ranges

5. **Bandwidth Costs:** Transferring hundreds of GB from GitHub LFS or mirrors incurs egress costs

### Impact

- **Analysts:** Blocked from starting forensic work due to storage/bandwidth constraints
- **Research Teams:** Unable to spin up temporary analysis environments in cloud VMs (expensive to provision 500+ GB disks)
- **CI/CD:** Cannot run automated tests against realistic datasets
- **Cost:** Unnecessary storage and bandwidth expenses for both archive hosts and users

## Proposed Solution

Enable `forensic-db` to **stream archive files directly from S3-compatible object storage** (AWS S3, Cloudflare R2, MinIO, etc.) without requiring local storage of the full archives.

### Key Benefits

1. **Zero Local Storage:** Process archives in-memory or with minimal temp buffering
2. **Selective Processing:** Download only the specific epoch ranges needed
3. **Fast Startup:** Begin ETL within seconds instead of hours
4. **Cost Reduction:** Leverage S3 range requests to minimize data transfer
5. **Scalability:** Enable horizontal scaling across multiple workers
6. **Cloud-Native:** First-class support for cloud deployments (Docker, Kubernetes)

## Requirements

### Functional Requirements

#### FR1: S3-Compatible Storage Support
- Support AWS S3, Cloudflare R2, DigitalOcean Spaces, Wasabi, MinIO, and any S3-compatible API
- Authenticate using standard credential mechanisms (access key/secret, IAM roles, environment variables)
- Support multiple regions and custom endpoints

#### FR2: Streaming Ingestion
- Stream archive files directly from object storage to ETL pipeline
- Process compressed (gzip/tar) streams without full extraction
- Maintain existing batching and parallel processing capabilities

#### FR3: Selective Range Processing
- Allow users to specify epoch ranges (e.g., "epochs 100-200")
- List available archives in bucket before downloading
- Skip or filter specific archive types (transaction vs account_state)

#### FR4: Backward Compatibility
- Existing file-based ingestion must continue to work
- No breaking changes to CLI interface
- Existing workflows remain unchanged

#### FR5: Progress & Observability
- Report download progress (bytes transferred, % complete)
- Log which archives are being processed
- Handle network interruptions gracefully with retries

### Non-Functional Requirements

#### NFR1: Performance
- Streaming throughput must not be slower than disk-based processing
- Target: Process 1000 transactions/sec minimum (matching current performance)
- Memory usage should remain bounded (<2 GB per worker thread)

#### NFR2: Reliability
- Automatic retry on transient network errors (exponential backoff)
- Resume capability for interrupted streams
- Validate archive integrity (checksums, manifests)

#### NFR3: Security
- Support encrypted storage (S3 server-side encryption)
- Never log credentials or expose in error messages
- Support IAM roles, STS tokens, and credential rotation

#### NFR4: Cost Efficiency
- Minimize data transfer with HTTP range requests where possible
- Cache manifest files locally to avoid repeated small requests
- Support requester-pays buckets for shared archives

## Specifications

### CLI Interface

#### New Global Options

```bash
libra-forensic-db [OPTIONS] <COMMAND>

Cloud Storage Options:
  --storage-backend <TYPE>      Storage backend: 'local' (default) or 's3'
  --s3-endpoint <URL>           S3-compatible endpoint (e.g., https://account.r2.cloudflarestorage.com)
  --s3-region <REGION>          S3 region (default: us-east-1)
  --s3-bucket <BUCKET>          S3 bucket name
  --s3-prefix <PREFIX>          Object key prefix/path within bucket (optional)
  --s3-access-key <KEY>         S3 access key (or use AWS_ACCESS_KEY_ID env var)
  --s3-secret-key <SECRET>      S3 secret key (or use AWS_SECRET_ACCESS_KEY env var)
```

#### Environment Variables

```bash
# S3 credentials (follows AWS SDK conventions)
export AWS_ACCESS_KEY_ID='your-access-key'
export AWS_SECRET_ACCESS_KEY='your-secret-key'
export AWS_REGION='us-east-1'

# Custom endpoint for S3-compatible providers
export AWS_ENDPOINT_URL='https://account.r2.cloudflarestorage.com'

# Forensic-db specific
export LIBRA_STORAGE_BACKEND='s3'
export LIBRA_S3_BUCKET='libra-archives'
export LIBRA_S3_PREFIX='epoch-archive-mainnet/v7'
```

#### Example Usage

**Cloudflare R2:**
```bash
# Using environment variables
export AWS_ACCESS_KEY_ID='your-r2-access-key'
export AWS_SECRET_ACCESS_KEY='your-r2-secret-key'
export AWS_ENDPOINT_URL='https://account.r2.cloudflarestorage.com'

libra-forensic-db ingest-all \
  --storage-backend s3 \
  --s3-bucket libra-archives \
  --s3-prefix v7/transaction \
  --archive-content transaction

# Using command-line flags
libra-forensic-db ingest-all \
  --storage-backend s3 \
  --s3-endpoint https://account.r2.cloudflarestorage.com \
  --s3-bucket libra-archives \
  --s3-prefix v7/transaction \
  --s3-access-key $R2_ACCESS_KEY \
  --s3-secret-key $R2_SECRET_KEY \
  --archive-content transaction
```

**AWS S3:**
```bash
# Use IAM instance role (no credentials needed)
libra-forensic-db ingest-all \
  --storage-backend s3 \
  --s3-bucket ol-mainnet-archives \
  --s3-region us-west-2 \
  --archive-content transaction
```

**Local files (existing behavior):**
```bash
libra-forensic-db ingest-all \
  --storage-backend local \
  --start-path ./epoch-archives \
  --archive-content transaction

# Or simply (local is default):
libra-forensic-db ingest-all \
  --start-path ./epoch-archives
```

### Configuration File Support

For complex setups, support TOML configuration:

```toml
# ~/.config/forensic-db/config.toml

[storage]
backend = "s3"

[storage.s3]
endpoint = "https://account.r2.cloudflarestorage.com"
bucket = "libra-archives"
prefix = "v7"
region = "auto"

# Optional: override specific credentials
# access_key = "..."
# secret_key = "..."

[database]
uri = "neo4j://localhost:7687"
username = "neo4j"
password = "secure-password"

[processing]
threads = 8
batch_size = 2000
```

## Technical Design

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│  (warehouse_cli.rs - parse storage backend config)          │
└────────────────────────┬────────────────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         │                               │
         ▼                               ▼
┌──────────────────┐            ┌─────────────────┐
│  LocalStorage    │            │   S3Storage     │
│  (existing)      │            │   (new)         │
└────────┬─────────┘            └────────┬────────┘
         │                               │
         │   Implements StorageBackend trait
         │                               │
         └───────────────┬───────────────┘
                         │
                         ▼
         ┌───────────────────────────────┐
         │    Archive Scanner/Loader     │
         │  (scan.rs, load.rs - minimal  │
         │   changes, uses trait)        │
         └───────────────┬───────────────┘
                         │
                         ▼
         ┌───────────────────────────────┐
         │      Extract & Transform      │
         │  (extract_*.rs - no changes)  │
         └───────────────┬───────────────┘
                         │
                         ▼
         ┌───────────────────────────────┐
         │         Neo4j Load            │
         │  (load_*.rs - no changes)     │
         └───────────────────────────────┘
```

### Core Abstractions

#### Storage Backend Trait

```rust
// src/storage/mod.rs

use async_trait::async_trait;
use anyhow::Result;
use std::path::PathBuf;

/// Represents a source of archive data (local filesystem, S3, etc.)
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// List all archive bundles at the given path/prefix
    async fn list_archives(&self, path: &str, content_type: Option<BundleContent>)
        -> Result<Vec<ArchiveInfo>>;

    /// Open a readable stream for a specific archive file
    async fn open_archive(&self, archive: &ArchiveInfo)
        -> Result<Box<dyn AsyncRead + Unpin + Send>>;

    /// Check if an archive exists
    async fn exists(&self, path: &str) -> Result<bool>;

    /// Get metadata for an archive (size, modified time, etc.)
    async fn metadata(&self, path: &str) -> Result<ArchiveMetadata>;

    /// Backend identifier for logging
    fn backend_type(&self) -> &'static str;
}

/// Metadata about an archive bundle
#[derive(Debug, Clone)]
pub struct ArchiveInfo {
    pub path: String,           // Full path or S3 key
    pub content_type: BundleContent,
    pub epoch_range: (u64, u64),
    pub size_bytes: Option<u64>,
    pub last_modified: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct ArchiveMetadata {
    pub size_bytes: u64,
    pub last_modified: SystemTime,
    pub content_type: Option<String>,
}
```

#### Local Storage Implementation

```rust
// src/storage/local.rs

pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn list_archives(&self, path: &str, content_type: Option<BundleContent>)
        -> Result<Vec<ArchiveInfo>> {
        // Existing logic from scan_dir_archive()
        // Walk filesystem, parse manifest files
        todo!("Port existing scan logic")
    }

    async fn open_archive(&self, archive: &ArchiveInfo)
        -> Result<Box<dyn AsyncRead + Unpin + Send>> {
        let path = self.base_path.join(&archive.path);
        let file = tokio::fs::File::open(path).await?;
        Ok(Box::new(file))
    }

    // ... other methods
}
```

#### S3 Storage Implementation

```rust
// src/storage/s3.rs

use aws_sdk_s3::{Client, Config, Region};
use aws_types::credentials::SharedCredentialsProvider;

pub struct S3Storage {
    client: Client,
    bucket: String,
    prefix: String,
}

impl S3Storage {
    pub async fn new(config: S3Config) -> Result<Self> {
        let sdk_config = aws_config::load_from_env().await;

        let mut s3_config_builder = Config::builder()
            .region(Region::new(config.region.clone()));

        // Custom endpoint for R2, MinIO, etc.
        if let Some(endpoint) = config.endpoint {
            s3_config_builder = s3_config_builder.endpoint_url(endpoint);
        }

        // Explicit credentials if provided
        if let (Some(key), Some(secret)) = (config.access_key, config.secret_key) {
            let credentials = aws_types::Credentials::new(
                key, secret, None, None, "forensic-db"
            );
            s3_config_builder = s3_config_builder
                .credentials_provider(SharedCredentialsProvider::new(credentials));
        }

        let s3_config = s3_config_builder.build();
        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: config.bucket,
            prefix: config.prefix.unwrap_or_default(),
        })
    }
}

#[async_trait]
impl StorageBackend for S3Storage {
    async fn list_archives(&self, path: &str, content_type: Option<BundleContent>)
        -> Result<Vec<ArchiveInfo>> {
        let prefix = format!("{}/{}", self.prefix, path).trim_start_matches('/').to_string();

        let mut archives = Vec::new();
        let mut continuation_token = None;

        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(&prefix);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await?;

            for object in response.contents.unwrap_or_default() {
                let key = object.key.unwrap();

                // Parse manifest.json files to identify archives
                if key.ends_with("manifest.json") {
                    let manifest = self.fetch_manifest(&key).await?;
                    archives.push(self.parse_archive_info(key, manifest)?);
                }
            }

            if !response.is_truncated.unwrap_or(false) {
                break;
            }
            continuation_token = response.next_continuation_token;
        }

        // Filter by content type if specified
        if let Some(ct) = content_type {
            archives.retain(|a| a.content_type == ct);
        }

        Ok(archives)
    }

    async fn open_archive(&self, archive: &ArchiveInfo)
        -> Result<Box<dyn AsyncRead + Unpin + Send>> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&archive.path)
            .send()
            .await?;

        let stream = response.body.into_async_read();
        Ok(Box::new(stream))
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        match self.client
            .head_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("404") || e.to_string().contains("NotFound") {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    async fn metadata(&self, path: &str) -> Result<ArchiveMetadata> {
        let response = self.client
            .head_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await?;

        Ok(ArchiveMetadata {
            size_bytes: response.content_length.unwrap_or(0) as u64,
            last_modified: response.last_modified
                .and_then(|dt| SystemTime::try_from(dt).ok())
                .unwrap_or(SystemTime::now()),
            content_type: response.content_type,
        })
    }

    fn backend_type(&self) -> &'static str {
        "s3"
    }
}

// Helper methods
impl S3Storage {
    async fn fetch_manifest(&self, key: &str) -> Result<ManifestInfo> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let bytes = response.body.collect().await?.into_bytes();
        let manifest: ManifestInfo = serde_json::from_slice(&bytes)?;
        Ok(manifest)
    }

    fn parse_archive_info(&self, key: String, manifest: ManifestInfo)
        -> Result<ArchiveInfo> {
        // Parse epoch range from key or manifest
        // Determine content type (transaction vs account_state)
        todo!("Parse manifest into ArchiveInfo")
    }
}
```

### Integration Points

#### CLI Updates

```rust
// src/warehouse_cli.rs

#[derive(Parser)]
pub struct WarehouseCli {
    // Existing database options...

    // Storage backend configuration
    #[clap(long, default_value = "local")]
    storage_backend: StorageBackend,

    #[clap(long)]
    s3_endpoint: Option<String>,

    #[clap(long)]
    s3_region: Option<String>,

    #[clap(long)]
    s3_bucket: Option<String>,

    #[clap(long)]
    s3_prefix: Option<String>,

    #[clap(long)]
    s3_access_key: Option<String>,

    #[clap(long)]
    s3_secret_key: Option<String>,

    // Existing subcommand...
}

impl WarehouseCli {
    pub async fn run(self) -> Result<()> {
        let storage: Box<dyn storage::StorageBackend> = match self.storage_backend {
            StorageBackend::Local => {
                Box::new(storage::LocalStorage::new(/* ... */))
            }
            StorageBackend::S3 => {
                let config = storage::S3Config {
                    endpoint: self.s3_endpoint,
                    region: self.s3_region.unwrap_or_else(|| "us-east-1".to_string()),
                    bucket: self.s3_bucket.ok_or_else(|| anyhow!("--s3-bucket required"))?,
                    prefix: self.s3_prefix,
                    access_key: self.s3_access_key.or_else(|| std::env::var("AWS_ACCESS_KEY_ID").ok()),
                    secret_key: self.s3_secret_key.or_else(|| std::env::var("AWS_SECRET_ACCESS_KEY").ok()),
                };
                Box::new(storage::S3Storage::new(config).await?)
            }
        };

        // Pass storage backend to ingestion logic
        match self.command {
            Sub::IngestAll { .. } => {
                ingest_all(storage, /* ... */).await?;
            }
            // ... other commands
        }

        Ok(())
    }
}
```

#### Ingestion Updates

```rust
// src/load.rs

pub async fn ingest_all(
    storage: Box<dyn StorageBackend>,
    start_path: &str,
    content_type: Option<BundleContent>,
    batch_size: usize,
    graph: Graph,
) -> Result<()> {
    // List all archives from storage backend
    let archives = storage.list_archives(start_path, content_type).await?;

    info!("Found {} archives to process", archives.len());

    for archive in archives {
        // Open stream from storage
        let mut reader = storage.open_archive(&archive).await?;

        // Process stream (existing logic)
        process_archive_stream(&mut reader, batch_size, &graph).await?;
    }

    Ok(())
}

async fn process_archive_stream(
    reader: &mut (dyn AsyncRead + Unpin + Send),
    batch_size: usize,
    graph: &Graph,
) -> Result<()> {
    // If compressed, wrap in decompressor
    let decoder = async_compression::tokio::bufread::GzipDecoder::new(
        tokio::io::BufReader::new(reader)
    );

    // If tar archive, extract entries
    let mut archive = tokio_tar::Archive::new(decoder);
    let mut entries = archive.entries()?;

    while let Some(entry) = entries.next().await {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();

        // Process entry based on type
        if path.ends_with(".json") {
            let mut contents = String::new();
            entry.read_to_string(&mut contents).await?;

            // Parse and load into graph (existing logic)
            process_json_entry(&contents, batch_size, graph).await?;
        }
    }

    Ok(())
}
```

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# S3 support
aws-config = "1.1"
aws-sdk-s3 = "1.15"
aws-types = "1.1"

# Async compression
async-compression = { version = "0.4", features = ["tokio", "gzip"] }
tokio-tar = "0.3"

# Streaming utilities
tokio-util = { version = "0.7", features = ["io"] }
bytes = "1.5"
```

## Implementation Plan

### Phase 1: Foundation (Week 1-2)
- [ ] Define `StorageBackend` trait
- [ ] Implement `LocalStorage` (refactor existing code)
- [ ] Update CLI to accept `--storage-backend` flag
- [ ] Add unit tests for trait and local implementation
- [ ] Update documentation

### Phase 2: S3 Integration (Week 3-4)
- [ ] Implement `S3Storage` with basic operations
- [ ] Add credential handling (env vars, CLI flags, IAM)
- [ ] Implement archive listing and streaming
- [ ] Add retry logic and error handling
- [ ] Integration tests with MinIO (local S3-compatible server)

### Phase 3: Optimization (Week 5-6)
- [ ] Add manifest caching to reduce API calls
- [ ] Implement progress reporting for downloads
- [ ] Add memory-bounded buffering
- [ ] Performance benchmarks (S3 vs local)
- [ ] Handle large files with multipart streams

### Phase 4: Production Readiness (Week 7-8)
- [ ] Comprehensive error messages
- [ ] User documentation and examples
- [ ] CLI reference updates
- [ ] Add configuration file support
- [ ] Real-world testing with R2 and AWS S3

## Alternatives Considered

### Alternative 1: Pre-Download Script
**Approach:** Provide a separate tool to download archives from S3 to local disk first.

**Pros:**
- No changes to core ETL code
- Simple to implement

**Cons:**
- Doesn't solve the disk space problem
- Still requires full downloads
- Adds another tool to maintain

**Verdict:** ❌ Rejected - doesn't address root problem

### Alternative 2: FUSE Filesystem Mount
**Approach:** Use `s3fs` or similar to mount S3 as a filesystem, use existing code.

**Pros:**
- Zero code changes
- Works with any S3-compatible storage

**Cons:**
- Requires root/FUSE permissions (not available in Docker)
- Poor performance (not optimized for sequential reads)
- Hidden complexity and failure modes
- Platform-specific (Linux/macOS only)

**Verdict:** ❌ Rejected - too many operational issues

### Alternative 3: HTTP Range Requests Only
**Approach:** Use simple HTTP GET requests with byte ranges instead of full S3 SDK.

**Pros:**
- Lighter dependency footprint
- Works with any HTTP server

**Cons:**
- Must reimplement auth, retries, pagination
- No IAM role support
- Doesn't work with private buckets easily

**Verdict:** ❌ Rejected - reinventing the wheel

### Alternative 4: Lazy Caching Proxy
**Approach:** Run a local proxy that fetches and caches archive chunks on demand.

**Pros:**
- Transparent to application
- Can share cache across multiple runs

**Cons:**
- Complex architecture (another service)
- Still needs local disk for cache
- Adds latency

**Verdict:** ❌ Rejected - over-engineered

## Success Criteria

### Functional Success
- [ ] Can ingest v7 transaction archives from Cloudflare R2 without local storage
- [ ] Can authenticate with AWS S3 using IAM instance roles
- [ ] Can selectively process epoch ranges (e.g., only epochs 100-199)
- [ ] Existing local file ingestion continues to work unchanged
- [ ] All existing tests pass

### Performance Success
- [ ] S3 streaming achieves ≥80% of local disk throughput
- [ ] Memory usage remains <2 GB per worker thread
- [ ] Can process 1000+ transactions/second from S3

### Operational Success
- [ ] Clear error messages for auth failures, network issues, etc.
- [ ] Progress reporting shows download and processing status
- [ ] Automatic retry recovers from transient failures
- [ ] Works in Docker without requiring additional configuration

### Documentation Success
- [ ] Getting Started guide updated with S3 setup instructions
- [ ] CLI Reference documents all new flags
- [ ] Example configurations for R2, AWS S3, MinIO
- [ ] Troubleshooting section covers common S3 issues

## Security Considerations

### Credentials Management
- **Never log credentials** in debug output or error messages
- Support AWS credential chain: env vars → config file → IAM role → instance metadata
- Document secure credential practices (IAM roles preferred over access keys)
- Support temporary credentials (STS tokens)

### Network Security
- Always use HTTPS for S3 endpoints (except localhost MinIO for testing)
- Validate SSL certificates by default
- Support custom CA certificates for private S3 servers

### Data Integrity
- Verify manifest checksums before processing
- Detect and report corrupted streams
- Log which archives were successfully processed for audit trail

## Cost Analysis

### Storage Costs (Cloudflare R2 Example)
- **Storage:** $0.015/GB/month → ~$1.50/month for 100 GB archive
- **Class A Operations** (list, write): $4.50 per million → ~$0.01 for typical ingestion
- **Class B Operations** (read): $0.36 per million → ~$0.01 for typical ingestion
- **Egress:** **$0** (R2 has no egress fees)

**Total estimated cost per full ingestion:** ~$0.02

### Bandwidth Savings
- **Before:** Users download 100 GB each → 1000 users = 100 TB egress
- **After:** Users stream only needed epochs → e.g., 10 GB each = 10 TB egress
- **Savings:** 90 TB egress avoided

For GitHub LFS, this represents **$9,000/month savings** at $0.10/GB egress.

## Open Questions

1. **Caching Strategy:** Should we cache manifest files locally? For how long?
   - **Recommendation:** Cache manifests for 24 hours, clear with `--clear-cache` flag

2. **Compression Handling:** Should we support streaming brotli/zstd in addition to gzip?
   - **Recommendation:** Start with gzip only, add others based on demand

3. **Authentication:** Should we support S3 pre-signed URLs for unauthenticated access?
   - **Recommendation:** Yes, useful for public read-only archives

4. **Partial Downloads:** Should we support resuming interrupted downloads?
   - **Recommendation:** Yes, but phase 2 - store checkpoint metadata

5. **Multi-Region:** Should we automatically select nearest S3 region?
   - **Recommendation:** No, let users specify region explicitly for now

## Future Enhancements

### Short-term (v0.3.0)
- Support for Azure Blob Storage backend
- Support for Google Cloud Storage backend
- Pre-signed URL support for temporary access

### Medium-term (v0.4.0)
- Parallel multi-part downloads for large archives
- Smart caching with LRU eviction
- Archive index to skip scanning (pre-built manifest DB)

### Long-term (v0.5.0+)
- Distributed processing across multiple workers
- CDC (change data capture) for incremental updates
- BitTorrent backend for peer-to-peer archive distribution

## References

- [AWS S3 SDK for Rust](https://docs.rs/aws-sdk-s3/)
- [Cloudflare R2 Documentation](https://developers.cloudflare.com/r2/)
- [S3 API Specification](https://docs.aws.amazon.com/AmazonS3/latest/API/)
- [tokio-tar crate](https://docs.rs/tokio-tar/)
- [async-compression crate](https://docs.rs/async-compression/)

## Feedback & Discussion

Please provide feedback via GitHub issues or discussions:
- **GitHub Issue:** `#<issue-number>` (to be created)
- **Discussion Thread:** (to be created)

---

**Next Steps:**
1. Review and gather feedback from maintainers and users
2. Create tracking issue and break into implementation tasks
3. Begin Phase 1 implementation
