use serde::Deserialize;

use crate::{Coin, Error};

#[cfg(feature = "mysql")]
use super::mysql::DbConfig as MySqlConfig;

#[cfg(feature = "postgres")]
use super::postgres::DbConfig as PostgresConfig;

#[cfg(feature = "sqlite")]
use super::sqlite::DbConfig as SqliteConfig;

use super::{Credentials, Database};

/// The type of the database.
///
/// This is a convenience enum to allow the use of different database types in a
/// configuration file. The enum is serialized and deserialized using the
/// `serde` crate. It implements the `Database` trait and forwards the calls to
/// the appropriate database type.
///
/// The serialization is tagged with the `type` field. This may have the following
/// values:
///
/// - `mysql` or `mariadb`: The configuration for a MySQL/MariaDB database.
/// - `postgres`: The configuration for a PostgreSQL database.
/// - `sqlite`: The configuration for a SQLite database.
///
/// See the documentation of the individual database types for more details.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum DbType {
    #[cfg(feature = "mysql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
    #[serde(alias = "mysql", alias = "mariadb")]
    /// The configuration for a MySQL/MariaDB database.
    MySql(MySqlConfig),
    #[cfg(feature = "postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
    #[serde(alias = "postgres")]
    /// The configuration for a PostgreSQL database.
    Postgres(PostgresConfig),
    #[cfg(feature = "sqlite")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
    #[serde(alias = "sqlite")]
    /// The configuration for a SQLite database.
    Sqlite(SqliteConfig),
}

impl Database for DbType {
    fn root_username(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "mysql")]
            Self::MySql(config) => config.root_username(),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(config) => config.root_username(),
            #[cfg(feature = "postgres")]
            Self::Postgres(config) => config.root_username(),
        }
    }

    fn requires_credentials(&self) -> bool {
        match self {
            #[cfg(feature = "mysql")]
            Self::MySql(config) => config.requires_credentials(),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(config) => config.requires_credentials(),
            #[cfg(feature = "postgres")]
            Self::Postgres(config) => config.requires_credentials(),
        }
    }

    async fn init_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: &[Coin],
    ) -> Result<(), Error> {
        match self {
            #[cfg(feature = "mysql")]
            Self::MySql(config) => config.init_schema(creds, coins).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(config) => config.init_schema(creds, coins).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(config) => config.init_schema(creds, coins).await,
        }
    }

    async fn drop_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: Option<&[Coin]>,
    ) -> Result<(), Error> {
        match self {
            #[cfg(feature = "mysql")]
            Self::MySql(config) => config.drop_schema(creds, coins).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(config) => config.drop_schema(creds, coins).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(config) => config.drop_schema(creds, coins).await,
        }
    }
}
