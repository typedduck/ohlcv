#![allow(clippy::doc_markdown, clippy::multiple_crate_versions)]
//! # ohlcv-ctl
//!
//! ## Status
//!
//! ![Build Status](https://img.shields.io/github/actions/workflow/status/typedduck/ohlcv/rust.yml)
//! [![Crates.io](https://img.shields.io/crates/v/ohlcv-ctl)](https://crates.io/crates/ohlcv-ctl)
//! [![Crates.io](https://img.shields.io/crates/d/ohlcv-ctl)](https://crates.io/crates/ohlcv-ctl)
//!     
//! - [x] Initialize the database schema, command `init`.
//! - [x] Drop the database schema, command `drop`.
//! - [ ] Download historical OHLCV data, command `fetch`.
//! - [ ] Export the data to a CSV or JSON file, command `export`.
//! - [ ] Import the data from a CSV or JSON file, command `import`.
//!
//! ## Overview
//!
//! `ohlcv-ctl` is a command line tool to interact with the ohlcv library. It
//! provides the following functionality:
//!
//! - Download historical OHLCV data from various cryptocurrency exchanges.
//! - Export the data to a CSV file.
//! - Initialize the database schema.
//! - Drop the database schema.
//!
//! The `fetch` command is used to download historical OHLCV data from various
//! cryptocurrency exchanges. The data is downloaded in a 5-minute interval of
//! the previous day, resulting in 288 candles per day. The candles are
//! aggregated in the database to form larger candles, such as 15-minute,
//! 1-hour, 4-hour, and 1-day candles.
//!
//! The data can be downloaded for multiple trading pairs and multiple exchanges
//! at the same time. The data is downloaded in parallel to speed up the
//! process. Care is taken to avoid rate limiting and to handle errors
//! gracefully.
//!
//! To get a consistent time-series of the data, the command line tool must be
//! run at least once a day. The tool will download the data for the previous
//! day and aggregate it in the database. If the tool is run more than once a
//! day, it will only download the missing trading pairs. All times are in UTC
//! only.
//!
//! The `init` command is used to initialize the database schema. The schema
//! includes tables for the candles of the trading pairs.
//!
//! The `drop` command is used to drop the database schema. This will remove the
//! tables and data from the database of the defined trading pairs. If the
//! `--all` option is used, all tables for all coins will be removed.
//!
//! ## Configuration
//!
//! The command line interface uses a configuration file to specify the database
//! and exchange settings. The configuration file is in TOML format and has the
//! following structure:
//!
//! ```toml
//! # If user_agent is not set, the default user agent `ohlcv-ctl/<version>`
//! # will be used.
//! user_agent = "<optional user-agent>"
//!
//! [database]
//! type = "mysql"
//! address = "localhost"
//! database = "ohlcv"
//! username = "<ohlcv user>"
//! password = "<secret password>"
//!
//! [[coins]]
//! name = "Bitcoin"
//! symbol = "BTC"
//! currency = "USD"
//! exchanges = { "Binance" = "BTCUSDC" }
//! ```
//!
//! See the implementation of the database configuration for more details about
//! the fields in the `OHLCV` crate.

mod cli;
pub use cli::{clargs, command};

pub mod config;

mod error;
pub use error::Error;
