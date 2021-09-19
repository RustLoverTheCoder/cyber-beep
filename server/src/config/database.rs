use std::time::Duration;

use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use serde::{Deserialize, Serialize};
use sqlx_core::postgres::PgPoolOptions;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_conn: Option<u32>,
    pub max_conn: u32,
    pub conn_timeout: u64,
    pub idle_timeout: Option<u64>,
}

pub async fn initialize(config: &DatabaseConfig) -> anyhow::Result<DatabaseConnection> {
    let mut options = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(config.conn_timeout))
        .max_connections(config.max_conn);
    if let Some(min) = config.idle_timeout {
        options = options.idle_timeout(Some(Duration::from_secs(min)));
    }
    if let Some(min) = config.min_conn {
        options = options.min_connections(min);
    }
    let pool = options.connect(&config.url).await?;
    Ok(SqlxPostgresConnector::from_sqlx_postgres_pool(pool))
}