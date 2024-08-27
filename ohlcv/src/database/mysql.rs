//! MySQL/MariaDB database implementation.

use serde::Deserialize;
use sqlx::{mysql::MySqlPoolOptions, MySql};
use tracing::{info, instrument};

use crate::{Coin, Error};

use super::{Credentials, Database};

/// The type of database.
pub type Db = MySql;
/// The type of the database pool.
pub type DbPool = sqlx::Pool<MySql>;
/// The type of the database options.
pub type DbOptions = MySqlPoolOptions;

/// The default port for a MySQL/MariaDB database.
pub const DEFAULT_PORT: u16 = 3306;
/// The default username for the root user.
pub const DEFAULT_ROOT: &str = "root";

/// The configuration for a MySQL/MariaDB database.
///
/// This struct is used to configure the connection to a MySQL/MariaDB database.
/// The fields are deserialized from a configuration file using the `serde`
/// crate. The struct implements the `Database` trait to allow interaction with
/// the database.
///
/// The configuration includes the following fields:
///
/// - `host`: The hostname of the database server.
/// - `port`: The port of the database server. If not set, the default port
///   `3306` is used.
/// - `database`: The name of the database.
/// - `username`: The username to connect to the database.
/// - `password`: The password to connect to the database. If not set, the
///   password must be defined as an environment variable. See the
///   [`Credentials`] struct for more information.
/// - `root_username`: The username of the root user. If not set, the default
///   username `root` is used.
///
/// The database must be created and managed beforehand. The tables are created
/// and dropped by the `root` user using the `init_schema` and `drop_schema`
/// methods.
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub(super) host: String,
    pub(super) port: Option<u16>,
    pub(super) database: String,
    pub(super) username: String,
    pub(super) password: Option<String>,
    pub(super) root_username: Option<String>,
    #[serde(skip)]
    pub(super) pool: Option<DbPool>,
}

impl DbConfig {
    #[instrument(skip(self, creds))]
    async fn connect(&self, creds: &Credentials) -> Result<DbPool, Error> {
        if let Some(password) = creds.password() {
            let username = creds.username();
            let url = format!(
                "mysql://{username}:{password}@{host}:{port}/{database}",
                host = self.host,
                port = self.port.unwrap_or(DEFAULT_PORT),
                database = self.database
            );

            DbOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .map_err(|err| Error::SqlConnect(self.username.clone(), Box::new(err)))
        } else {
            Err(Error::MissingPassword(creds.username().to_owned()))
        }
    }

    #[instrument(skip(self))]
    async fn db(&mut self) -> Result<&DbPool, Error> {
        if self.pool.is_none() {
            let creds = Credentials::try_from(&*self)?;
            self.pool = Some(self.connect(&creds).await?);
        }

        // This is safe because the `db` field is set above.
        Ok(self.pool.as_ref().unwrap())
    }
}

impl Database for DbConfig {
    #[inline]
    fn root_username(&self) -> Option<&str> {
        self.root_username.as_deref().or(Some(DEFAULT_ROOT))
    }

    #[inline]
    fn requires_credentials(&self) -> bool {
        true
    }

    #[instrument(skip(self, creds, coins))]
    async fn init_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: &[Coin],
    ) -> Result<(), Error> {
        let root = self.root_username().unwrap();
        let creds = creds.unwrap_or_else(|| Credentials::new(root));
        let db = self.connect(&creds).await?;

        info!("Initializing schema for MySQL database");
        for coin in coins {
            info!("Creating table for {coin:#}");
            let table = coin.table_name();
            let query = format!(
                "CREATE TABLE IF NOT EXISTS {table} (
                    time_stamp TIMESTAMP NOT NULL,
                    time_frame ENUM('5m', '15m', '1h', '4h', '1d') NOT NULL,
                    sources SMALLINT UNSIGNED NOT NULL,
                    open DECIMAL(20, 10) NOT NULL,
                    high DECIMAL(20, 10) NOT NULL,
                    low DECIMAL(20, 10) NOT NULL,
                    close DECIMAL(20, 10) NOT NULL,
                    volume DECIMAL(20, 10) NOT NULL,
                    PRIMARY KEY (time_stamp, time_frame)
                );"
            );

            sqlx::query(&query)
                .execute(&db)
                .await
                .map_err(|err| Error::SqlCreateTable(table, Box::new(err)))?;
        }
        Ok(())
    }

    #[instrument(skip(self, creds, coins))]
    async fn drop_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: Option<&[Coin]>,
    ) -> Result<(), Error> {
        let root = self.root_username().unwrap();
        let creds = creds.unwrap_or_else(|| Credentials::new(root));
        let db = self.connect(&creds).await?;

        info!("Dropping schema for MySQL database");
        if let Some(coins) = coins {
            for coin in coins {
                info!("Dropping table for {coin:#}");
                let table = coin.table_name();
                let query = format!("DROP TABLE IF EXISTS {table};");

                sqlx::query(&query)
                    .execute(&db)
                    .await
                    .map_err(|err| Error::SqlDropTable(table, Box::new(err)))?;
            }
        } else {
            let query = "SHOW TABLES;";
            let tables = sqlx::query_as::<Db, (String,)>(query)
                .fetch_all(&db)
                .await
                .map_err(|err| Error::SqlSelect(Box::new(err)))?;

            for table in tables {
                let table = table.0;
                info!("Dropping table `{table}`");

                if table.starts_with(Coin::table_prefix()) {
                    let query = format!("DROP TABLE IF EXISTS {table};");

                    sqlx::query(&query)
                        .execute(&db)
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
        self.host == other.host
            && self.port == other.port
            && self.database == other.database
            && self.username == other.username
            && self.root_username == other.root_username
    }
}
