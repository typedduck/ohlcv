use std::path::PathBuf;

use ohlcv::Database;
use tracing::instrument;

use crate::{
    config::{CoinConfig, Config},
    Error,
};

use super::root_credentials;

/// Initialize the database
///
/// # Arguments
///
/// * `config` - Optional path to the configuration file. If not provided, the
///   default configuration file will be used. This file is expected to be in
///   TOML format. The default file is `ohlcv.toml` and is expected to be in the
///   current working directory or in `/etc/ohlcv`.
///
/// # Errors
///
/// Returns an error if the database cannot be initialized or if the
/// configuration file cannot be loaded.
#[instrument]
pub async fn init(config: Option<&PathBuf>) -> Result<(), Error> {
    let mut config = Config::load(config)?;
    let creds = root_credentials(&config.database)?;
    let coins = config
        .coins
        .iter()
        .map(CoinConfig::as_coin)
        .collect::<Vec<_>>();

    config
        .database
        .init_schema(creds, coins.as_slice())
        .await
        .map_err(Error::Ohlcv)
}
