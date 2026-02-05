use crate::util::de_address_from_any_string;
use anyhow::{Context, Result};
use diem_types::account_address::AccountAddress;
use log::info;
use neo4rs::Graph;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Exchange onboard json files are formatted like so:
// NOTE: that the address string is flexible:
//  can be in upper or lowercase and with 0x prepended or not.
// [
//   {
//     "user_id": 189,
//     "onramp_address": "01F3B9C815FEB654718DE5D53CD665699A2B80951B696939E2D9EC27D0126BAD"
//   },
//   ...
// ]

/// Metadata for exchange on-ramp mapping.
///
/// Maps an on-chain address to an exchange user ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeOnRamp {
    #[serde(deserialize_with = "de_address_from_any_string")]
    onramp_address: Option<AccountAddress>,
    // TODO: this should be string, since exchanges/bridges will have different identifiers
    user_id: u64,
}

impl ExchangeOnRamp {
    /// Parses a JSON file containing exchange on-ramp mappings.
    pub fn parse_json_file(path: &Path) -> Result<Vec<Self>> {
        let s = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&s)?)
    }

    /// Converts the struct into a Cypher-compatible object string.
    pub fn to_cypher_object_template(&self) -> String {
        format!(
            r#"{{user_id: {}, address: "{}" }}"#,
            self.user_id,
            self.onramp_address.as_ref().unwrap().to_hex_literal(),
        )
    }

    /// Creates a Cypher map string from a list of `ExchangeOnRamp` objects.
    pub fn to_cypher_map(list: &[Self]) -> String {
        let mut list_literal = "".to_owned();
        for el in list {
            // skip empty records
            if el.onramp_address.is_none() {
                continue;
            };
            let s = el.to_cypher_object_template();
            list_literal.push_str(&s);
            list_literal.push(',');
        }
        list_literal.pop(); // need to drop last comma ","
        format!("[{}]", list_literal)
    }

    /// Generates a Cypher query to batch link addresses to swap accounts.
    pub fn cypher_batch_link_owner(list_str: &str) -> String {
        format!(
            r#"
  WITH {list_str} AS owner_data
  UNWIND owner_data AS each_owner

  MATCH (id:SwapAccount {{swap_id: each_owner.user_id}})
  MATCH (addr:Account {{address: each_owner.address}})
  MERGE (addr)-[rel:OnRamp]->(id)

  WITH rel
  RETURN
      COUNT(rel) AS owners_merged
"#
        )
    }
}

/// Batches and inserts exchange on-ramp data into the Neo4j database.
pub async fn impl_batch_tx_insert(pool: &Graph, batch_txs: &[ExchangeOnRamp]) -> Result<u64> {
    let mut unique_owners = vec![];
    batch_txs.iter().for_each(|t| {
        if !unique_owners.contains(&t.user_id) {
            unique_owners.push(t.user_id);
        }
    });

    info!("unique owner links in batch: {}", unique_owners.len());

    let list_str = ExchangeOnRamp::to_cypher_map(batch_txs);

    // first insert the users
    // cypher queries makes it annoying to do a single insert of users and
    // txs
    let cypher_string = ExchangeOnRamp::cypher_batch_link_owner(&list_str);

    // Execute the query
    let cypher_query = neo4rs::query(&cypher_string);
    let mut res = pool
        .execute(cypher_query)
        .await
        .context("execute query error")?;

    let row = res.next().await?.context("no row returned")?;

    let owners_merged: u64 = row.get("owners_merged").context("no owners_merged field")?;

    info!("owners linked to addresses: {}", owners_merged);

    Ok(owners_merged)
}
