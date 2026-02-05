use anyhow::{bail, Context, Result};
use log::info;
use neo4rs::Graph;

use crate::scan::ArchiveMap;

/// Updates or creates a task in the Neo4j queue.
pub async fn update_task(
    pool: &Graph,
    archive_id: &str,
    completed: bool,
    batch: usize,
) -> Result<String> {
    let cypher_string = format!(
        r#"MERGE (a:Queue {{ archive_id: "{}", batch: {} }})
        SET a.completed = {}
        RETURN a.archive_id AS archive_id"#,
        archive_id,
        batch,
        completed.to_string().to_lowercase(),
    );

    let cypher_query = neo4rs::query(&cypher_string);

    let mut res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;

    let row = res.next().await?.context("no row returned")?;
    let task_id: String = row.get("archive_id").context("no created_accounts field")?;
    Ok(task_id)
}

/// Retrieves all archive IDs that have incomplete tasks in the queue.
pub async fn get_queued(pool: &Graph) -> Result<Vec<String>> {
    let cypher_string = r#"
      MATCH (a:Queue)
      WHERE a.completed = false
      RETURN DISTINCT a.archive_id
    "#;

    let cypher_query = neo4rs::query(cypher_string);

    let mut res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;

    let mut archive_ids: Vec<String> = vec![];

    while let Some(row) = res.next().await? {
        // Extract `archive_id` as a String
        if let Ok(archive_name) = row.get::<String>("a.archive_id") {
            archive_ids.push(archive_name);
        }
    }

    Ok(archive_ids)
}

/// Checks if a specific batch of an archive is marked as complete.
///
/// Returns `Ok(Some(true))` if complete, `Ok(Some(false))` if incomplete,
/// and `Err` if the task is not found.
pub async fn is_batch_complete(
    pool: &Graph,
    archive_id: &str,
    batch: usize,
) -> Result<Option<bool>> {
    let cypher_string = format!(
        r#"
        MATCH (a:Queue {{ archive_id: "{}", batch: {} }})
        RETURN DISTINCT a.completed;
      "#,
        archive_id, batch
    );

    let cypher_query = neo4rs::query(&cypher_string);

    let mut res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;

    if let Some(row) = res.next().await? {
        // Extract `archive_id` as a String
        Ok(row.get::<bool>("a.completed").ok())
    } else {
        bail!("not found")
    }
}

/// Checks if all batches for a given archive ID are completed.
pub async fn are_all_completed(pool: &Graph, archive_id: &str) -> Result<bool> {
    let cypher_string = format!(
        r#"
        MATCH (a:Queue {{archive_id: '{}' }})
        WITH COLLECT(a.completed) AS completedStatuses, COUNT(a) AS totalTasks
        RETURN CASE
          WHEN totalTasks = 0 THEN false
          ELSE ALL(status IN completedStatuses WHERE status = true)
        END AS allCompleted;
      "#,
        archive_id,
    );

    let cypher_query = neo4rs::query(&cypher_string);

    let mut res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;

    if let Some(row) = res.next().await? {
        // Extract `archive_id` as a String
        Ok(row.get::<bool>("allCompleted")?)
    } else {
        bail!("not found")
    }
}

/// Removes all tasks from the queue in the database.
pub async fn clear_queue(pool: &Graph) -> Result<()> {
    let cypher_string = r#"
        MATCH (a:Queue)
        DELETE a
      "#
    .to_string();

    let cypher_query = neo4rs::query(&cypher_string);

    let mut _res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;
    Ok(())
}

/// Populates the queue from an `ArchiveMap`, marking the first batch (0) as pending.
pub async fn push_queue_from_archive_map(map: &ArchiveMap, pool: &Graph) -> Result<()> {
    for (_, a) in map.0.iter() {
        // set at least one batch of each archive_id to false, so it gets picked up in the queue
        update_task(pool, &a.archive_id, false, 0).await?;
        info!("enqueued archive {}, batch #0", &a.archive_id);
    }
    Ok(())
}
