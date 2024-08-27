use std::path::PathBuf;

use tracing::instrument;

use crate::{config::Config, Error};

/// Fetch data from the origin.
///
/// # Arguments
///
/// * `config` - Optional path to the configuration file. If not provided, the
///   default configuration file will be used. This file is expected to be in
///   TOML format. The default file is `ohlcv.toml` and is expected to be in
///   the current working directory or in `/etc/ohlcv`.
///
/// # Errors
///
/// Returns an error if the data cannot be fetched or if the configuration file
/// cannot be loaded.
#[instrument]
pub async fn fetch(config: Option<&PathBuf>) -> Result<(), Error> {
    let _config = Config::load(config)?;

    todo!()
}
