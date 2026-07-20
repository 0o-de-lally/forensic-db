//! S3-compatible cloud storage backend (e.g., Cloudflare R2).

use super::backend::{ArchiveDescriptor, MaterializedArchive, StorageBackend};
use crate::scan::BundleContent;
use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use diem_temppath::TempPath;
use log::{debug, info};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

/// S3-compatible storage backend for cloud archives.
///
/// This backend supports S3-compatible services like Cloudflare R2.
/// Archives are downloaded on-demand to temporary directories and
/// automatically cleaned up via RAII.
pub struct S3Storage {
    client: Client,
    bucket: String,
    prefix: String,
}

impl S3Storage {
    /// Create a new S3 storage backend.
    ///
    /// # Arguments
    /// * `endpoint` - S3 endpoint URL (e.g., for Cloudflare R2)
    /// * `access_key` - AWS access key ID
    /// * `secret_key` - AWS secret access key
    /// * `bucket` - S3 bucket name
    /// * `prefix` - Optional prefix for filtering objects (e.g., "v6/transaction/")
    pub async fn new(
        endpoint: String,
        access_key: String,
        secret_key: String,
        bucket: String,
        prefix: String,
    ) -> Result<Self> {
        // Configure for S3-compatible services (e.g., Cloudflare R2)
        let credentials = Credentials::new(
            access_key,
            secret_key,
            None,  // session token
            None,  // expiry
            "static",  // provider name
        );

        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(endpoint)
            .region("auto")  // R2 uses "auto" region
            .credentials_provider(credentials)
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket,
            prefix,
        })
    }
}

impl StorageBackend for S3Storage {
    fn list_archives(
        &self,
        content_filter: Option<BundleContent>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ArchiveDescriptor>>> + Send + '_>> {
        Box::pin(async move {
        let manifest_suffix = content_filter.unwrap_or(BundleContent::Unknown).filename();

        info!("Listing S3 objects in bucket: {}, prefix: {}", self.bucket, self.prefix);

        let mut descriptors = Vec::new();
        let mut continuation_token: Option<String> = None;

        // Paginate through all objects
        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(&self.prefix);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await
                .context("Failed to list S3 objects")?;

            if let Some(contents) = response.contents() {
                for object in contents {
                    if let Some(key) = object.key() {
                        // Look for manifest files
                        if key.contains(".manifest") {
                            // Check if it matches the filter
                            let content_type = if key.contains("transaction.manifest") {
                                BundleContent::Transaction
                            } else if key.contains("state.manifest") {
                                BundleContent::StateSnapshot
                            } else if key.contains("epoch_ending.manifest") {
                                BundleContent::EpochEnding
                            } else {
                                BundleContent::Unknown
                            };

                            // Skip if doesn't match filter
                            if let Some(ref filter) = content_filter {
                                if content_type != *filter && *filter != BundleContent::Unknown {
                                    continue;
                                }
                            }

                            // Extract archive_id from the key
                            // Key format: "prefix/archive_id/file.manifest"
                            let archive_id = key
                                .trim_end_matches(&manifest_suffix)
                                .trim_end_matches(".gz")
                                .trim_end_matches('/')
                                .split('/')
                                .last()
                                .unwrap_or(key)
                                .to_string();

                            // Get the directory prefix (everything before the filename)
                            let remote_path = key
                                .rsplit_once('/')
                                .map(|(dir, _)| dir.to_string())
                                .unwrap_or_else(|| key.to_string());

                            let compressed = key.ends_with(".gz");

                            debug!("Found archive: {} at {}", archive_id, remote_path);

                            descriptors.push(ArchiveDescriptor {
                                archive_id,
                                remote_path,
                                content_type,
                                compressed,
                            });
                        }
                    }
                }
            }

            // Check if there are more pages
            if response.is_truncated().unwrap_or(false) {
                continuation_token = response.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        info!("Found {} archives in S3", descriptors.len());
        Ok(descriptors)
        })
    }

    fn materialize_archive(
        &self,
        descriptor: &ArchiveDescriptor,
    ) -> Pin<Box<dyn Future<Output = Result<MaterializedArchive>> + Send + '_>> {
        Box::pin(async move {
        info!("Downloading archive: {} from S3", descriptor.archive_id);

        // Create temp directory for this archive
        let temp_dir = TempPath::new();
        temp_dir.create_as_dir()
            .context("Failed to create temp directory")?;

        let archive_path = temp_dir.path().join(&descriptor.archive_id);
        std::fs::create_dir_all(&archive_path)
            .context("Failed to create archive subdirectory")?;

        // Download all files in this archive directory
        // List all objects with the archive prefix
        let list_prefix = format!("{}/", descriptor.remote_path);

        let mut continuation_token: Option<String> = None;
        let mut file_count = 0;

        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(&list_prefix);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await
                .context("Failed to list archive files in S3")?;

            if let Some(contents) = response.contents() {
                for object in contents {
                    if let Some(key) = object.key() {
                        // Download this file
                        let filename = key
                            .strip_prefix(&list_prefix)
                            .unwrap_or(key);

                        // Skip if it's a directory marker
                        if filename.is_empty() {
                            continue;
                        }

                        let local_file_path = archive_path.join(filename);

                        debug!("Downloading: {} -> {}", key, local_file_path.display());

                        let get_response = self.client
                            .get_object()
                            .bucket(&self.bucket)
                            .key(key)
                            .send()
                            .await
                            .context(format!("Failed to download {}", key))?;

                        // Read the body and write to file
                        let data = get_response.body.collect().await
                            .context("Failed to read object body")?
                            .into_bytes();

                        std::fs::write(&local_file_path, data)
                            .context(format!("Failed to write file {}", local_file_path.display()))?;

                        file_count += 1;
                    }
                }
            }

            if response.is_truncated().unwrap_or(false) {
                continuation_token = response.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        info!("Downloaded {} files for archive: {}", file_count, descriptor.archive_id);

        Ok(MaterializedArchive {
            local_path: archive_path,
            _temp_handle: Some(temp_dir),
        })
        })
    }
}
