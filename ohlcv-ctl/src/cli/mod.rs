use clap::ArgMatches;

pub mod command;

/// Command line interface for the collector.
///
/// Returns the matches from the command line arguments.
#[allow(clippy::cognitive_complexity)]
#[must_use]
pub fn clargs() -> ArgMatches {
    use std::path::PathBuf;

    use clap::{arg, command, value_parser, ArgAction, Command};

    let command = command!()
        .subcommand(
            Command::new("init")
                .about("Initialize the database tables")
                .arg(
                    arg!(config: -c --config <FILE> "optional path to the configuration file")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("drop")
                .about("Remove the database tables")
                .arg(arg!(all: -a --all "remove tables for all coins").action(ArgAction::SetTrue))
                .arg(
                    arg!(config: -c --config <FILE> "optional path to the configuration file")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("fetch")
                .about("Fetch data from the origin")
                .arg(
                    arg!(config: -c --config <FILE> "optional path to the configuration file")
                        .value_parser(value_parser!(PathBuf)),
                ),
        );

    command.get_matches()
}
