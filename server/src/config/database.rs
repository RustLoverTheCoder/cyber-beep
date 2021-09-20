use std::time::Duration;

use sea_orm::{DbConn, SqlxPostgresConnector, DbErr};
use serde::{Deserialize, Serialize};
use sqlx_core::postgres::PgPoolOptions;
use sea_orm::sea_query::TableCreateStatement;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_conn: Option<u32>,
    pub max_conn: u32,
    pub conn_timeout: u64,
    pub idle_timeout: Option<u64>,
}

pub async fn initialize(config: &DatabaseConfig) -> anyhow::Result<DbConn> {
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
    let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    // todo auto create table

    Ok(connection)
}