#![allow(clippy::doc_markdown, clippy::multiple_crate_versions)]

use ohlcv_ctl::{clargs, command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite")))]
compile_error!("At least one of the features 'mysql', 'postgres', or 'sqlite' must be enabled.");

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let matches = clargs();
    let command = matches.subcommand();

    if let Err(err) = command::execute(command).await {
        eprintln!("Error: {err}");
    }
}
