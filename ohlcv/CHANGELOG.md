# Change Log

## [0.0.2](https://github.com/typedduck/ohlcv/tree/ohlcv-v0.0.2) - 2024-08-27

- Issue with `std::env::remove_var` and `std::env::set_var` not thread-safe.
  Changed the tests of `Credentials` to ensure that they are executed
  sequentially.

## [0.0.1](https://github.com/typedduck/ohlcv/tree/ohlcv-v0.0.1) - 2024-08-27

- Initial release
- Add base types, candle, and coin.
- Add database interface and schema initialization.
- Database support for SQLite, PostgreSQL, and MySQL/MariaDB.
- Minimal exchange implementation.
