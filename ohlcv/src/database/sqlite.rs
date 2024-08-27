//! SQLite database implementation.

use serde::Deserialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tracing::{info, instrument};

use crate::{Coin, Error};

use super::{Credentials, Database};

/// The type of database.
pub type Db = Sqlite;
/// The type of the database pool.
pub type DbPool = sqlx::Pool<Sqlite>;
/// The type of the database options.
pub type DbOptions = SqlitePoolOptions;

/// The configuration for a SQLite database.
///
/// This struct is used to configure the connection to a SQLite database. The
/// fields are deserialized from a configuration file using the `serde` crate.
/// The struct implements the `Database` trait to allow interaction with the
/// database.
///
/// The configuration includes the following fields:
///
/// - `database`: The name of the database.
///
/// On initialization, the database is created if it does not exist. This
/// differs from the other database types, where the database must be created
/// and managed beforehand.
#[derive(Debug, Default, Deserialize)]
pub struct DbConfig {
    database: String,
    #[serde(skip)]
    pool: Option<DbPool>,
}

impl DbConfig {
    #[instrument(skip(self))]
    async fn db(&mut self) -> Result<&DbPool, Error> {
        let exists = Db::database_exists(&self.database)
            .await
            .map_err(|err| Error::SqlConnect("default user".to_owned(), Box::new(err)))?;

        if !exists {
            Db::create_database(&self.database)
                .await
                .map_err(|err| Error::SqlConnect("default user".to_owned(), Box::new(err)))?;
        }
        if self.pool.is_none() {
            let url = format!("sqlite://{}", self.database);
            let pool = DbOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .map_err(|err| Error::SqlConnect("default user".to_owned(), Box::new(err)))?;
            self.pool = Some(pool);
        }

        // This is safe because the `pool` field is set above.
        Ok(self.pool.as_ref().unwrap())
    }
}

impl Database for DbConfig {
    #[inline]
    fn root_username(&self) -> Option<&'static str> {
        None
    }

    #[inline]
    fn requires_credentials(&self) -> bool {
        false
    }

    #[instrument(skip(self, _creds, coins))]
    async fn init_schema(
        &mut self,
        _creds: Option<Credentials>,
        coins: &[Coin],
    ) -> Result<(), Error> {
        let db = self.db().await?;

        info!("Initializing schema for SQLite database");
        for coin in coins {
            info!("Creating table for {coin:#}");
            let table = coin.table_name();
            let query = format!(
                "CREATE TABLE IF NOT EXISTS {table} (
                    time_stamp TIMESTAMP NOT NULL,
                    time_frame TEXT NOT NULL,
                    sources INTEGER NOT NULL,
                    open REAL NOT NULL,
                    high REAL NOT NULL,
                    low REAL NOT NULL,
                    close REAL NOT NULL,
                    volume REAL NOT NULL,
                    PRIMARY KEY (time_stamp, time_frame)
                );"
            );

            sqlx::query(&query)
                .execute(db)
                .await
                .map_err(|err| Error::SqlCreateTable(table, Box::new(err)))?;
        }
        Ok(())
    }

    #[instrument(skip(self, _creds, coins))]
    async fn drop_schema(
        &mut self,
        _creds: Option<Credentials>,
        coins: Option<&[Coin]>,
    ) -> Result<(), Error> {
        let db = self.db().await?;

        info!("Dropping schema for SQLite database");
        if let Some(coins) = coins {
            for coin in coins {
                info!("Dropping table for {coin:#}");
                let table = coin.table_name();
                let query = format!("DROP TABLE IF EXISTS {table};");

                sqlx::query(&query)
                    .execute(db)
                    .await
                    .map_err(|err| Error::SqlDropTable(table, Box::new(err)))?;
            }
        } else {
            let query = "SELECT name FROM sqlite_master WHERE type = 'table';";
            let tables = sqlx::query_as::<Db, (String,)>(query)
                .fetch_all(db)
                .await
                .map_err(|err| Error::SqlSelect(Box::new(err)))?;

            for table in tables {
                let table = table.0;
                info!("Dropping table `{table}`");

                if table.starts_with(Coin::table_prefix()) {
                    let query = format!("DROP TABLE IF EXISTS {table};");

                    sqlx::query(&query)
                        .execute(db)
                        .await
                        .map_err(|err| Error::SqlDropTable(table, Box::new(err)))?;
                }
            }
        }
        Ok(())
    }
}

impl PartialEq for DbConfig {
    fn eq(&self, other: &Self) -> bool {
        self.database == other.database
    }
}
