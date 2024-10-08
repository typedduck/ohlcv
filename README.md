# OHLCV

OHLCV is a library and a command line tool for downloading historical OHLCV
(Open, High, Low, Close, Volume) data from various cryptocurrency exchanges. The
data is stored in a database and can be easily queried using SQL.

Price values are stored as [`Decimal`](https://crates.io/crates/rust_decimal)
values. This ensures that the data is processed accurately and without rounding
errors.

The library is written in Rust and uses the [SQLx](https://crates.io/crates/sqlx)
crate for database access.

## Status

This project is in the early stages of development and is not yet ready for
production use. This is the initial upload defining the project structure.

![Build Status](https://img.shields.io/github/actions/workflow/status/typedduck/ohlcv/rust.yml)

**OHLCV**:

[![Crates.io](https://img.shields.io/crates/v/ohlcv)](https://crates.io/crates/ohlcv)
[![Crates.io](https://img.shields.io/crates/d/ohlcv)](https://crates.io/crates/ohlcv)

**OHLCV-ctl**:

[![Crates.io](https://img.shields.io/crates/v/ohlcv-ctl)](https://crates.io/crates/ohlcv-ctl)
[![Crates.io](https://img.shields.io/crates/d/ohlcv-ctl)](https://crates.io/crates/ohlcv-ctl)

## Data model

The data model mainly consists of the following types:

- `Candle`: Represents a candlestick in a trading pair.
- `Coin`: Represents a cryptocurrency and the quote currency.
- `Currency`: Represents a currency.
- `Timeframe`: Represents a timeframe of a candlestick.

The data model is designed to be simple and easy to use. For every trading pair
(`Candle`) consisting of a base currency and a quote currency, there is a
corresponding table in the database. The table name is constructed from the base
currency, the quote currency, and a prefix. The prefix is the same for all
tables and is used to group the tables together.

In the table there the candles are aggregated for all timeframes. The columns of
the table are the following:

- `time_stamp`: The start time of the candle.
- `time_frame`: The timeframe of the candle.
- `sources`: The number of sources the candle was downloaded from.
- `open`: The opening price of the candle.
- `high`: The highest price of the candle.
- `low`: The lowest price of the candle.
- `close`: The closing price of the candle.
- `volume`: The volume of the candle.

The primary key of the table is the combination of `time_stamp` and
`time_frame`.

## Database access

The library supports the following databases:

- SQLite
- PostgreSQL
- MySQL/MariaDB

The database can be accessed using the `DbType` type. The tables defining the
candles can be initialized and dropped using the `init_schema` and `drop_schema`
methods. All data definition is done by the `root` user. The normal user only
has access to the data. Exception to this is SQLite, where no user management is
needed.

See the implementation of the database configuration for more details.

The `Database` trait provides methods to interact with the database. The trait
is implemented for the [`DbType`] type.

## Download historical OHLCV data

**This feature is not yet implemented.**

The library can download historical OHLCV data from various cryptocurrency
exchanges. The data is stored in a database and can be queried using SQL. The
data will be downloaded in a 5-minute interval of the previous day, resulting in
288 candles per day. The candles will be aggregated in the database to form
larger candles, such as 15-minute, 1-hour, 4-hour, and 1-day candles.

It is possible to download data for multiple trading pairs and multiple
exchanges at the same time. In order to collect the data, the library will use
the exchange's public REST API. The data will be downloaded in parallel to speed
up the process. Care will be taken to avoid rate limiting and to handle errors
gracefully.

To get a consistent time-series of the data, the command line tool must be run
at least once a day. The tool will download the data for the previous day and
aggregate it in the database. If the tool is run more than once a day, it will
only download the missing trading pairs. All times are in UTC only.

Supported exchanges include:

- Binance
- KuCoin

More exchanges will be added in the future.

The databases supported include:

- SQLite
- PostgreSQL
- MySQL
- MariaDB

The downloaded data can be exported to a CSV file.

There will be methods implemented to handle gaps in the data. Gaps will be
classified as:

- Short gaps: A gap of one or two 5-minute candles.
- Moderate gaps: A gap of three to five 5-minute candles.

The library provides methods to fill the gaps in the data. The gaps will be
filled by interpolating the missing candles. Short gaps will be filled by
linear interpolation, while moderate gaps will be filled by cubic spline
interpolation. Special care is taken, if the gap is at the beginning or end of
the data.

The download will fail for a trading pair for an exchange if:

- there is a gap of more than five candles,
- next gap is less than 5 candles away,
- more than 5% of the data is missing.

If dowloading a trading pair from more than one exchange a gap may not be
across two or more exchanges at the same time. In this case the candles of
the first exchange will be kept and the candles of the other exchanges will be
downloaded again. The order of the exchanges is determined by the order of the
exchanges in the configuration file for the trading pair.

There will be three attempts to download the data with increasing time between
attempts for a trading pair for an exchange.

## Helper scripts

In the `scripts` directory there are helper scripts that can be used to
set up the supported databases. All scripts, except for the SQLite database,
install the required dependencies as docker containers.

## License

This project is licensed under the MIT License or the Apache License 2.0, at
your option. For details, see the `LICENSE-MIT` and `LICENSE-APACHE` files for
more information.

## Support

If you like this project and want to support it, you can do so by:

- Giving it a star on GitHub.
- Sharing it with your friends.
- Contributing to the project by opening an issue or a pull request.
- Donating to the project by using the following links:
  - Bitcoin (Taproot): `bc1pqdck3v3r7sa4mgl0dztfzufa4xg66g8cpcgwvjax9rtx6mlxafdqcgw3g2`
  - Bitcoin (Segwit): `bc1qet2ypmsxtx6mc03329ft5a736fy906flm4c42a9d3e7mvu872tcs8myzs6`
  - [Patreon](https://www.patreon.com/typedduck)

Patreon supporters will be listed in the `SUPPORTERS.md` file.
