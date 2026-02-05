use std::fmt::Display;

/// Response metadata for a batch transaction insertion.
///
/// Tracks the number of unique, created, modified, and unchanged accounts,
/// as well as the total number of transactions created in the batch.
#[derive(Debug, Clone)]
pub struct BatchTxReturn {
    pub unique_accounts: u64,
    pub created_accounts: u64,
    pub modified_accounts: u64,
    pub unchanged_accounts: u64,
    pub created_tx: u64,
}

impl Display for BatchTxReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total Transactions - unique accounts: {}, created accounts: {}, modified accounts: {}, unchanged accounts: {}, transactions created: {}",
          self.unique_accounts,
          self.created_accounts,
          self.modified_accounts,
          self.unchanged_accounts,
          self.created_tx
        )
    }
}

impl Default for BatchTxReturn {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchTxReturn {
    /// Creates a new, empty `BatchTxReturn`.
    pub fn new() -> Self {
        Self {
            unique_accounts: 0,
            created_accounts: 0,
            modified_accounts: 0,
            unchanged_accounts: 0,
            created_tx: 0,
        }
    }

    /// Increments the current counts with values from another `BatchTxReturn`.
    pub fn increment(&mut self, new: &BatchTxReturn) {
        self.unique_accounts += new.unique_accounts;
        self.created_accounts += new.created_accounts;
        self.modified_accounts += new.modified_accounts;
        self.unchanged_accounts += new.unchanged_accounts;
        self.created_tx += new.created_tx;
    }
}
