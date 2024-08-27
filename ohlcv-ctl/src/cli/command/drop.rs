use std::path::PathBuf;

use ohlcv::Database;
use tracing::instrument;

use crate::{
    config::{CoinConfig, Config},
    Error,
};

use super::root_credentials;

/// Drop tables from the database.
///
/// # Arguments
///
/// * `all` - Whether to drop all tables. If false, only tables for the
///   configured coins will be dropped.
/// * `config` - Optional path to the configuration file. If not provided, the
///   default configuration file will be used. This file is expected to be in
///   TOML format. The default file is `ohlcv.toml` and is expected to be in the
///   current working directory or in `/etc/ohlcv`.
///
/// # Errors
///
/// Returns an error if the tables cannot be dropped or if the configuration
/// file cannot be loaded.
#[instrument]
pub async fn drop(all: bool, config: Option<&PathBuf>) -> Result<(), Error> {
    let mut config = Config::load(config)?;
    let creds = root_credentials(&config.database)?;

    if all {
        config.database.drop_schema(creds, None).await?;
    } else {
        let coins = config
            .coins
            .iter()
            .map(CoinConfig::as_coin)
            .collect::<Vec<_>>();

        config
            .database
            .drop_schema(creds, Some(coins.as_slice()))
            .await?;
    }
    Ok(())
}
