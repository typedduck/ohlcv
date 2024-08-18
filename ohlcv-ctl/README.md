# OHLCV Command Line Interface

![Build Status](https://img.shields.io/github/actions/workflow/status/typedduck/ohlcv/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/kamo-macros)](https://crates.io/crates/ohlcv-ctl)
[![Crates.io](https://img.shields.io/crates/d/kamo-macros)](https://crates.io/crates/ohlcv-ctl)

The OHLCV command line interface is a tool for downloading historical OHLCV
(Open, High, Low, Close, Volume) data from various cryptocurrency exchanges.

The functionality provided by the command line interface is implemented in the
[OHLCV library](https://crates.io/crates/ohlcv).

## Status

This project is in the early stages of development and is not yet ready for
production use. This is the initial upload defining the project structure.

## Command Line Interface

The command line interface provides the following functionality:

- Download historical OHLCV data from various cryptocurrency exchanges.
- Export the data to a CSV file.
- Initialize the database schema.
- Drop the database schema.

```text
Usage: ohlcv-ctl [COMMAND]

Commands:
  init   Initialize the database tables
  drop   Remove the database tables
  fetch  Fetch data from the origin
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The `fetch` command is used to download historical OHLCV data from various
cryptocurrency exchanges. The data is downloaded in a 5-minute interval of the
previous day, resulting in 288 candles per day. The candles are aggregated in
the database to form larger candles, such as 15-minute, 1-hour, 4-hour, and
1-day candles.

The data can be downloaded for multiple trading pairs and multiple exchanges at
the same time. The data is downloaded in parallel to speed up the process. Care
is taken to avoid rate limiting and to handle errors gracefully.

To get a consistent time-series of the data, the command line tool must be run
at least once a day. The tool will download the data for the previous day and
aggregate it in the database. If the tool is run more than once a day, it will
only download the missing trading pairs. All times are in UTC only.

```text
Usage: ohlcv-ctl fetch [OPTIONS]

Options:
  -c, --config <FILE>  optional path to the configuration file
  -h, --help           Print help
```

The `init` command is used to initialize the database schema. The schema
includes tables for the candles of the trading pairs.

```text
Usage: ohlcv-ctl init [OPTIONS]

Options:
  -c, --config <FILE>  optional path to the configuration file
  -h, --help           Print help
```

The `drop` command is used to drop the database schema. This will remove the
tables and data from the database of the defined trading pairs. If the `--all`
option is used, all tables for all coins will be removed.

```text
Usage: ohlcv-ctl drop [OPTIONS]

Options:
  -a, --all            remove tables for all coins
  -c, --config <FILE>  optional path to the configuration file
  -h, --help           Print help
```

## Configuration

The command line interface uses a configuration file to specify the database and
exchange settings. The configuration file is in TOML format and has the
following structure:

```toml
# If user_agent is not set, the default user agent `ohlcv-ctl/<version>` will be
# used.
user_agent = "<optional user-agent>"

[database]
address = "localhost"
database = "ohlcv"
username = "<ohlcv user>"
password = "<secret password>"

[[coins]]
name = "Bitcoin"
symbol = "BTC"
currency = "USD"
exchanges = { "Binance" = "BTCUSDC" }
```
