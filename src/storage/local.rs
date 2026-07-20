//! Local filesystem storage backend.

use super::backend::{ArchiveDescriptor, MaterializedArchive, StorageBackend};
use crate::scan::BundleContent;
use anyhow::{Context, Result};
use glob::glob;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

/// Storage backend for local filesystem access.
///
/// This is a zero-overhead wrapper that uses the existing glob-based
/// archive discovery logic. No downloads or copies are performed.
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    /// Create a new local storage backend.
    ///
    /// # Arguments
    /// * `base_path` - Root directory to search for archives
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

impl StorageBackend for LocalStorage {
    fn list_archives(
        &self,
        content_filter: Option<BundleContent>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ArchiveDescriptor>>> + Send + '_>> {
        Box::pin(async move {
            let path = self.base_path.canonicalize()?;
            let filename = content_filter.unwrap_or(BundleContent::Unknown).filename();
            let pattern = format!(
                "{}/**/{}*", // also matches .gz
                path.to_str().context("cannot parse starting dir")?,
                filename,
            );

            let mut descriptors = Vec::new();

            for manifest_path in glob(&pattern)?.flatten() {
                let archive_dir = manifest_path
                    .parent()
                    .context("can't find manifest dir")?;

                let archive_id = archive_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .context("invalid archive directory name")?
                    .to_owned();

                let content_type = BundleContent::new_from_man_file(&manifest_path);

                // Check if files are compressed by looking for .gz extension
                let compressed = manifest_path
                    .to_str()
                    .map(|s| s.ends_with(".gz"))
                    .unwrap_or(false);

                descriptors.push(ArchiveDescriptor {
                    archive_id,
                    remote_path: archive_dir.to_string_lossy().to_string(),
                    content_type,
                    compressed,
                });
            }

            Ok(descriptors)
        })
    }

    fn materialize_archive(
        &self,
        descriptor: &ArchiveDescriptor,
    ) -> Pin<Box<dyn Future<Output = Result<MaterializedArchive>> + Send + '_>> {
        Box::pin(async move {
            // No download needed for local files, just return the existing path
            Ok(MaterializedArchive {
                local_path: PathBuf::from(&descriptor.remote_path),
                _temp_handle: None,
            })
        })
    }
}
