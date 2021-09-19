use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Database {
    pub url: String,
    pub min_conn: Option<u32>,
    pub max_conn: usize,
    pub conn_timeout: u64,
    pub idle_timeout: Option<u64>,
}