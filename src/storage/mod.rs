//! Storage abstraction for blockchain archives.
//!
//! This module provides a trait-based approach for accessing archives from different storage backends:
//! - `LocalStorage`: Direct filesystem access
//! - `S3Storage`: S3-compatible cloud storage (e.g., Cloudflare R2)
//!
//! The abstraction allows lazy downloading and temporary materialization for S3 archives
//! while maintaining zero-copy performance for local files.

pub mod backend;
pub mod local;
pub mod s3;

pub use backend::{ArchiveDescriptor, MaterializedArchive, StorageBackend};
pub use local::LocalStorage;
pub use s3::S3Storage;
