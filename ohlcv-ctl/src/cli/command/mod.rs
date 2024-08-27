//! Command line interface for the collector.

mod drop;
use std::fmt;

pub use drop::drop;

mod fetch;
pub use fetch::fetch;

mod init;
pub use init::init;

use clap::ArgMatches;
use inquire::{Password, PasswordDisplayMode};
use ohlcv::{
    database::{Credentials, DbType},
    Database,
};
use tracing::instrument;

use crate::Error;

/// Execute the command specified by the command line arguments.
///
/// # Errors
///
/// Returns an error if the command is not recognized or if an error occurs
/// while executing the command.
#[instrument(skip(command))]
pub async fn execute(command: Option<(&str, &ArgMatches)>) -> Result<(), Error> {
    match command {
        Some(("drop", args)) => {
            let config = args.get_one::<std::path::PathBuf>("config");
            let all = args.get_flag("all");

            drop(all, config).await
        }
        Some(("init", args)) => {
            let config = args.get_one::<std::path::PathBuf>("config");

            init(config).await
        }
        Some(("fetch", args)) => {
            let config = args.get_one::<std::path::PathBuf>("config");

            fetch(config).await
        }
        Some((command, _)) => Err(Error::CommandName(command.into())),
        None => fetch(None).await,
    }
}

#[instrument]
fn ask_password(username: impl AsRef<str> + fmt::Debug) -> Result<String, Error> {
    let username = username.as_ref();

    Password::new(&format!(
        "Enter password for the database user `{username}`:"
    ))
    .with_display_toggle_enabled()
    .with_display_mode(PasswordDisplayMode::Hidden)
    .without_confirmation()
    .with_help_message("Output is hidden.")
    .prompt()
    .map_err(|err| Error::AskPassword(username.into(), Box::new(err)))
}

fn root_credentials(db: &DbType) -> Result<Option<Credentials>, Error> {
    if let Some(username) = db.root_username() {
        let creds = Credentials::new(username);

        if creds.has_password() {
            return Ok(Some(creds));
        }

        let password = ask_password(username)?;
        return Ok(Some(creds.with_password(password)));
    }
    Ok(None)
}
