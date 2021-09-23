pub use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

use crate::config::ServerConfig;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TracingConfig {
    /// Structured logging global level
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub level: Level,
    /// Structured logging filter:
    pub filter: Option<String>,
}

pub fn initialize(config: &ServerConfig) -> anyhow::Result<()> {
    let tracing = &config.tracing;
    LogTracer::init()?;

    let filter = match &tracing.filter {
        None => EnvFilter::from_default_env(),
        Some(directives) => EnvFilter::try_new(directives)?,
    }.add_directive(tracing.level.into());

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(ChronoLocal::rfc3339())
        .finish();
    Ok(tracing::subscriber::set_global_default(subscriber)?)
}