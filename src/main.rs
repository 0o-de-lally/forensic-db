//! # Libra Forensic DB CLI
//!
//! Entry point for the `libra-forensic-db` binary.
//! Provides a command-line interface for ingesting Libra blockchain archives
//! into a Neo4j graph database.
use clap::Parser;
use libra_forensic_db::{log_setup, warehouse_cli::WarehouseCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_setup();
    WarehouseCli::parse().run().await?;
    Ok(())
}
