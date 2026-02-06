use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use neo4rs::Graph;
use serde_json::json;
use std::path::PathBuf;

use crate::{
    analytics::{self, offline_matching::Matching},
    enrich_exchange_onboarding::{self, ExchangeOnRamp},
    enrich_whitepages::{self, Whitepages},
    json_rescue_v5_load,
    load::{ingest_all, try_load_one_archive},
    load_exchange_orders,
    neo4j_init::{self, get_credentials_from_env},
    scan::{scan_dir_archive, BundleContent, ManifestInfo},
    unzip_temp, util,
};

/// CLI for the Libra Forensic Database.
///
/// Supports various subcommands for ingesting archives, enriching data, and running analytics.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
pub struct WarehouseCli {
    #[clap(long, short('r'))]
    /// URI of graphDB e.g. neo4j+s://localhost:port
    db_uri: Option<String>,

    #[clap(long, short('u'))]
    /// username of db
    db_username: Option<String>,

    #[clap(long, short('p'))]
    /// db password
    db_password: Option<String>,

    #[clap(long, short('q'))]
    /// force clear queue
    clear_queue: bool,

    #[clap(long, short('t'))]
    /// max tasks to run in parallel
    threads: Option<usize>,

    #[clap(subcommand)]
    command: Sub,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Sub {
    /// scans sub directories for archive bundles
    IngestAll {
        #[clap(long, short('d'))]
        /// path to start crawling from
        start_path: PathBuf,
        #[clap(long, short('c'))]
        /// type of content to load
        archive_content: Option<BundleContent>,
        #[clap(long, short('b'))]
        /// size of each batch to load
        batch_size: Option<usize>,
    },
    /// process and load a single archive
    IngestOne {
        #[clap(long, short('d'))]
        /// location of archive
        archive_dir: PathBuf,

        #[clap(long, short('b'))]
        /// size of each batch to load
        batch_size: Option<usize>,
    },
    /// check archive is valid and can be decoded
    Check {
        #[clap(long, short('d'))]
        archive_dir: PathBuf,
    },
    /// add supporting data in addition to chain records
    EnrichExchange {
        #[clap(long)]
        /// file with swap records
        exchange_json: PathBuf,
        #[clap(long)]
        /// size of each batch to load
        batch_size: Option<usize>,
    },
    /// link an onboarding address to an exchange ID
    EnrichExchangeOnramp {
        #[clap(long)]
        /// file with onboarding accounts
        onboarding_json: PathBuf,
    },
    /// map owners of accounts from json file
    EnrichWhitepages {
        #[clap(long)]
        /// file with owner map
        owner_json: PathBuf,
    },
    VersionFiveTx {
        #[clap(long)]
        /// starting path for v5 .tgz files
        archive_dir: PathBuf,
    },
    /// Start a local Neo4j instance using Docker
    LocalDockerDb {
        #[clap(long, default_value = "./neo4j_data")]
        /// Local directory to store data
        data_dir: PathBuf,

        #[clap(long, default_value = "neo4j:5.12.0")]
        /// Docker image tag to use
        docker_image: String,
    },
    #[clap(subcommand)]
    Analytics(AnalyticsSub),
}

#[derive(Subcommand)]

pub enum AnalyticsSub {
    ExchangeRMS {
        #[clap(long)]
        /// commits the analytics to the db
        persist: bool,
    },

    TradesMatching {
        #[clap(long)]
        /// start day (exclusive) of trades YYYY-MM-DD
        start_day: String,

        #[clap(long)]
        /// end day (exclusive) of trades YYYY-MM-DD
        end_day: String,

        #[clap(long)]
        /// slow search producing likely candidates at each day
        /// requires top n # for length of initial list to scan
        replay_balances: Option<u64>,

        #[clap(long)]
        /// get perfect deposit matches on dump cases, requires tolerance value of 1.0 or more
        match_simple_dumps: Option<f64>,

        #[clap(long)]
        /// clear cache for local matches
        clear_cache: bool,
    },
}

impl WarehouseCli {
    /// Runs the CLI application based on the parsed subcommand.
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Sub::IngestAll {
                start_path,
                archive_content,
                batch_size,
            } => {
                let map = scan_dir_archive(start_path, archive_content.to_owned())?;

                let pool = try_db_connection_pool(self).await?;
                neo4j_init::maybe_create_indexes(&pool).await?;

                ingest_all(&map, &pool, self.clear_queue, batch_size.unwrap_or(250)).await?;
            }
            Sub::IngestOne {
                archive_dir,
                batch_size,
            } => {
                info!("checking if we need to decompress");
                let (archive_dir, temp) = unzip_temp::maybe_handle_gz(archive_dir)?;
                let mut man = ManifestInfo::new(&archive_dir);
                man.set_info()?;
                let pool = try_db_connection_pool(self).await?;
                neo4j_init::maybe_create_indexes(&pool).await?;

                try_load_one_archive(&man, &pool, batch_size.unwrap_or(250)).await?;
                drop(temp);
            }
            Sub::Check { archive_dir } => {
                let am = scan_dir_archive(archive_dir, None)?;
                if am.0.is_empty() {
                    error!("cannot find .manifest file under {}", archive_dir.display());
                }
                for (p, man) in am.0 {
                    info!("manifest found at {} \n {:?}", p.display(), man);
                }
            }
            Sub::EnrichExchange {
                exchange_json: swap_record_json,
                batch_size,
            } => {
                let pool = try_db_connection_pool(self).await?;
                neo4j_init::maybe_create_indexes(&pool).await?;

                let (merged, ignored) = load_exchange_orders::load_from_json(
                    swap_record_json,
                    &pool,
                    batch_size.unwrap_or(250),
                )
                .await?;
                info!(
                    "SUCCESS: exchange transactions merged: {}, ignored: {}",
                    merged, ignored
                );
            }
            Sub::EnrichExchangeOnramp { onboarding_json } => {
                info!("exchange onramp");
                let pool = try_db_connection_pool(self).await?;

                let wp = ExchangeOnRamp::parse_json_file(onboarding_json)?;
                let owners_merged =
                    enrich_exchange_onboarding::impl_batch_tx_insert(&pool, &wp).await?;

                println!("SUCCESS: {} exchange onramp accounts linked", owners_merged);
            }
            Sub::EnrichWhitepages {
                owner_json: json_file,
            } => {
                info!("whitepages");
                let pool = try_db_connection_pool(self).await?;

                let wp = Whitepages::parse_json_file(json_file)?;
                let owners_merged = enrich_whitepages::impl_batch_tx_insert(&pool, &wp).await?;

                println!("SUCCESS: {} owner accounts linked", owners_merged);
            }
            Sub::VersionFiveTx { archive_dir } => {
                let pool = try_db_connection_pool(self).await?;

                json_rescue_v5_load::rip_concurrent_limited(
                    archive_dir,
                    &pool,
                    self.threads.to_owned(),
                )
                .await?;
            }
            Sub::LocalDockerDb {
                data_dir,
                docker_image,
            } => {
                let current_dir = std::env::current_dir()?;
                let abs_data_dir = if data_dir.is_absolute() {
                    data_dir.clone()
                } else {
                    current_dir.join(data_dir)
                };

                // Create data and logs directories if they don't exist
                std::fs::create_dir_all(&abs_data_dir)?;
                let data_mount = abs_data_dir.join("data");
                let logs_mount = abs_data_dir.join("logs");
                std::fs::create_dir_all(&data_mount)?;
                std::fs::create_dir_all(&logs_mount)?;

                info!(
                    "Starting Neo4j using docker image: {}, data dir: {}",
                    docker_image,
                    abs_data_dir.display()
                );

                // Use a default password 'neo4j' if not specified in env
                // The CLI struct has db_password, but that's for connecting *to* the DB.
                // We should probably use the same password if provided, or default to 'neo4j'.
                // However, the standard Neo4j image expects NEO4J_AUTH=user/password
                let user = self.db_username.as_deref().unwrap_or("neo4j");
                let pass = self.db_password.as_deref().unwrap_or("neo4j");
                let auth_env = format!("{}/{}", user, pass);

                let status = std::process::Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg("forensic-neo4j")
                    .arg("--rm") // Remove container on exit
                    .arg("-p")
                    .arg("7474:7474")
                    .arg("-p")
                    .arg("7687:7687")
                    .arg("-v")
                    .arg(format!("{}:/data", data_mount.display()))
                    .arg("-v")
                    .arg(format!("{}:/logs", logs_mount.display()))
                    .arg("--env")
                    .arg(format!("NEO4J_AUTH={}", auth_env))
                    // Increase heap memory for better performance
                    .arg("--env")
                    .arg("NEO4J_dbms_memory_heap_initial__size=1G")
                    .arg("--env")
                    .arg("NEO4J_dbms_memory_heap_max__size=2G")
                    .arg(docker_image)
                    .status()?;

                if !status.success() {
                    bail!("Docker command failed with status: {}", status);
                }
            }
            Sub::Analytics(analytics_sub) => match analytics_sub {
                AnalyticsSub::ExchangeRMS { persist } => {
                    if *persist {
                        warn!("ExchangeRMS committing analytics to database!")
                    };
                    let pool = try_db_connection_pool(self).await?;
                    let results = analytics::exchange_stats::query_rms_analytics_concurrent(
                        &pool, None, None, *persist,
                    )
                    .await?;
                    println!("{:#}", json!(&results).to_string());
                }
                AnalyticsSub::TradesMatching {
                    replay_balances,
                    match_simple_dumps,
                    clear_cache,
                    start_day,
                    end_day,
                } => {
                    let dir: PathBuf = PathBuf::from(".");

                    if *clear_cache {
                        Matching::clear_cache(&dir)?;
                    }

                    if replay_balances.is_none() && match_simple_dumps.is_none() {
                        bail!("nothing to do. Must enter --replay-balance or --match-simple-dumps")
                    }
                    let pool = try_db_connection_pool(self).await?;

                    let mut m = Matching::read_cache_from_file(&dir).unwrap_or_default();
                    if let Some(top_n) = replay_balances {
                        let _ = m
                            .depth_search_by_top_n_accounts(
                                &pool,
                                util::parse_date(start_day),
                                util::parse_date(end_day),
                                *top_n,
                                Some(dir.clone()),
                            )
                            .await;
                    }

                    if let Some(tolerance) = match_simple_dumps {
                        m.search_dumps(
                            &pool,
                            util::parse_date(start_day),
                            util::parse_date(end_day),
                            *tolerance,
                        )
                        .await?;
                    }

                    m.write_cache_to_file(&dir)?;
                    m.write_definite_to_file(&dir)?;

                    println!("{:#}", json!(&m.definite));
                }
            },
        };
        Ok(())
    }
}

/// Attempts to establish a connection pool to Neo4j using credentials from env or CLI args.
pub async fn try_db_connection_pool(cli: &WarehouseCli) -> Result<Graph> {
    let db = match get_credentials_from_env() {
        Ok((uri, user, password)) => Graph::new(uri, user, password).await?,
        Err(_) => {
            let uri = cli
                .db_uri
                .as_ref()
                .expect("Must pass --db-uri or set URI_ENV");
            let user = cli
                .db_username
                .as_ref()
                .expect("Must pass --db-user or set USER_ENV");
            let password = cli
                .db_password
                .as_ref()
                .expect("Must pass --db-password or set PASS_ENV");
            Graph::new(uri, user, password).await?
        }
    };
    Ok(db)
}
