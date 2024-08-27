//! Configuration for ohlcv-ctl.

use std::{collections::HashMap, fmt, path::Path};

use ohlcv::{database::DbType, Coin, Currency, Exchange};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::Error;

/// Name of the default configuration file.
pub const CONFIG_FILE: &str = concat!(env!("CARGO_PKG_NAME"), ".toml",);

/// Default paths to search for the configuration file if not specified by the
/// user either through a command-line argument or environment variable. The
/// paths are appended with [`CONFIG_FILE`] to form the full path to the
/// configuration file. Paths are searched in order, and the first file found is
/// used.
pub const CONFIG_PATHS: [&str; 2] = [".", "/etc/ohlcv"];

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Map of exchange names to the coin's symbol on that exchange.
pub type ExchangeMap = HashMap<Exchange, String>;

/// Configuration for a coin.
#[derive(Debug, Deserialize)]
#[allow(clippy::module_name_repetitions, dead_code)]
pub struct CoinConfig {
    symbol: String,
    name: String,
    currency: Currency,
    /// Map of exchange names to the coin's symbol on that exchange.
    pub exchanges: ExchangeMap,
}

impl CoinConfig {
    /// Convert the configuration into a [`Coin`] instance.
    #[must_use]
    pub fn as_coin(&self) -> ohlcv::Coin {
        Coin::new(self.symbol.clone(), self.name.clone(), self.currency)
    }
}

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct Config {
    user_agent: Option<Box<str>>,
    /// Database connection information.
    pub database: DbType,
    /// List of coins to fetch.
    pub coins: Vec<CoinConfig>,
}

impl Config {
    /// Load the configuration from the specified file.
    ///
    /// # Errors
    ///
    /// This function returns an error if the file cannot be read or if the
    /// configuration is not valid TOML defined by the [`Config`] struct.
    #[instrument]
    pub fn load(path: Option<impl AsRef<Path> + fmt::Debug>) -> Result<Self, Error> {
        let path = path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(|| {
                CONFIG_PATHS
                    .iter()
                    .map(|p| Path::new(p).join(CONFIG_FILE))
                    .find(|p| p.exists())
            })
            .ok_or_else(|| Error::ConfigFile)?;
        info!("Loading configuration from {:?}", path);
        let source = std::fs::read_to_string(path)?;

        toml::from_str(&source).map_err(Error::ConfigFormat)
    }

    /// Get the user agent string to use for HTTP requests.
    #[must_use]
    #[inline]
    #[instrument(skip(self))]
    pub fn user_agent(&self) -> &str {
        self.user_agent.as_deref().unwrap_or(USER_AGENT)
    }
}
