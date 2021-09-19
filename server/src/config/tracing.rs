use anyhow::Context;
use serde::{Deserialize, Serialize};
pub use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

use crate::config::Config;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tracing {
    /// Structured logging global level
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub level: Level,
    /// Structured logging filter:
    pub filter: Option<String>,
}

pub fn initialize(config: &Config) -> anyhow::Result<()> {
    let tracing = &config.tracing;
    let mut filter = EnvFilter::from_default_env().add_directive(tracing.level.into());
    if let Some(directive) = &tracing.filter {
        let directive = directive.parse()
            .context(format!("[{}]::tracing_filter Invalid filter instruction", config.profile))?;
        filter = filter.add_directive(directive);
    };
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(ChronoLocal::rfc3339())
        .finish();
    Ok(tracing::subscriber::set_global_default(subscriber)?)
}