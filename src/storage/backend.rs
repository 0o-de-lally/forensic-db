//! Core storage backend trait and types.

use crate::scan::BundleContent;
use anyhow::Result;
use diem_temppath::TempPath;
use std::path::PathBuf;

use std::pin::Pin;
use std::future::Future;

/// Trait for storage backends that provide access to blockchain archives.
///
/// Implementations handle listing archives and materializing them to local paths
/// for processing. S3-based backends download on-demand, while local backends
/// return existing paths.
pub trait StorageBackend: Send + Sync {
    /// List all archives matching the content type filter.
    ///
    /// # Arguments
    /// * `content_filter` - Optional filter for specific content types (Transaction, StateSnapshot, etc.)
    ///
    /// # Returns
    /// Vector of archive descriptors that can be materialized for processing.
    fn list_archives(
        &self,
        content_filter: Option<BundleContent>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ArchiveDescriptor>>> + Send + '_>>;

    /// Get a local path to the archive.
    ///
    /// For S3 backends, this downloads the archive to a temporary directory.
    /// For local backends, this is a no-op that returns the existing path.
    ///
    /// # Arguments
    /// * `descriptor` - Archive metadata from `list_archives()`
    ///
    /// # Returns
    /// A `MaterializedArchive` with a local path and optional cleanup handle.
    fn materialize_archive(
        &self,
        descriptor: &ArchiveDescriptor,
    ) -> Pin<Box<dyn Future<Output = Result<MaterializedArchive>> + Send + '_>>;
}

/// Metadata about a discovered archive.
#[derive(Clone, Debug)]
pub struct ArchiveDescriptor {
    /// Unique identifier for the archive (directory name).
    pub archive_id: String,
    /// Remote path (S3 key) or local filesystem path.
    pub remote_path: String,
    /// Type of content in this archive.
    pub content_type: BundleContent,
    /// Whether the archive files are gzip-compressed.
    pub compressed: bool,
}

/// A materialized archive with a local filesystem path.
///
/// For S3-sourced archives, the `_temp_handle` ensures automatic cleanup
/// when the struct is dropped (RAII pattern).
pub struct MaterializedArchive {
    /// Local filesystem path to the archive directory.
    pub local_path: PathBuf,
    /// Optional temporary path handle for automatic cleanup.
    /// When this drops, the temp directory is deleted.
    pub _temp_handle: Option<TempPath>,
}
