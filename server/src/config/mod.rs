use std::net::{IpAddr, Ipv4Addr};

use anyhow::Context;
use figment::{Error, Figment, Metadata, Profile, Provider};
use figment::providers::{Env, Format, Serialized, Toml};
use figment::value::{Dict, Map};
use serde::{Deserialize, Serialize};

use crate::config::database::Database;
use crate::config::tracing::{Level, Tracing};

mod tracing;
mod database;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    pub profile: Profile,
    pub address: IpAddr,
    pub port: u16,
    pub tracing: Tracing,
    pub postgres: Database
}

impl Config {
    const DEBUG_PROFILE: Profile = Profile::const_new("debug");

    #[cfg(not(debug_assertions))]
    const RELEASE_PROFILE: Profile = Profile::const_new("release");

    #[cfg(debug_assertions)]
    const DEFAULT_PROFILE: Profile = Self::DEBUG_PROFILE;

    #[cfg(not(debug_assertions))]
    const DEFAULT_PROFILE: Profile = Self::RELEASE_PROFILE;
}

impl Config {
    /// Extract configuration file
    pub fn extract() -> anyhow::Result<Config> {
        dotenv::dotenv().ok();

        let figment = Figment::from(Config::default())
            .merge(Toml::file(Env::var_or("SERVER_CONFIG", "server.toml")).nested())
            .merge(Env::prefixed("SERVER_").ignore(&["PROFILE"]).global())
            .select(Profile::from_env_or("SERVER_PROFILE", Self::DEFAULT_PROFILE));

        let mut config = figment.extract::<Self>().context("Failed to extract configuration file")?;
        config.profile = figment.profile().clone();
        Ok(config)
    }

    pub fn init_tracing(&self) -> anyhow::Result<()> {
        tracing::initialize(&self)
    }

    fn debug_default() -> Config {
        Config {
            profile: Self::DEBUG_PROFILE,
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            port: 5000,
            tracing: Tracing {
                level: Level::INFO,
                filter: None,
            },
            postgres: Database {
                url: "postgres://localhost/postgres".to_string(),
                min_conn: None,
                max_conn: num_cpus::get() * 4,
                conn_timeout: 5,
                idle_timeout: None
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn release_default() -> Config {
        Config {
            profile: Self::RELEASE_PROFILE,
            ..Config::debug_default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(debug_assertions)] { Config::debug_default() }
        #[cfg(not(debug_assertions))] { Config::release_default() }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Cyber-Beep-Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        // This can be used to extend
        Serialized::defaults(self).data()
    }

    fn profile(&self) -> Option<Profile> {
        Some(self.profile.clone())
    }
}


