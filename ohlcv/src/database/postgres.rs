//! PostgreSQL database implementation.

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Postgres};
use tracing::{info, instrument};

use crate::{Coin, Error};

use super::{Credentials, Database};

/// The type of database.
pub type Db = Postgres;
/// The type of the database pool.
pub type DbPool = sqlx::Pool<Postgres>;
/// The type of the database options.
pub type DbOptions = PgPoolOptions;

/// The default port for a PostgreSQL database.
pub const DEFAULT_PORT: u16 = 5432;
/// The default username for the root user.
pub const DEFAULT_ROOT: &str = "postgres";

/// The configuration for a PostgreSQL database.
///
/// This struct is used to configure the connection to a PostgreSQL database.
/// The fields are deserialized from a configuration file using the `serde`
/// crate. The struct implements the `Database` trait to allow interaction with
/// the database.
///
/// The configuration includes the following fields:
///
/// - `host`: The hostname of the database server.
/// - `port`: The port of the database server. If not set, the default port
///   `5432` is used.
/// - `database`: The name of the database.
/// - `schema`: The schema of the database. If not set, the default schema
///   `public` is used.
/// - `username`: The username to connect to the database.
/// - `password`: The password to connect to the database. If not set, the
///   password must be defined as an environment variable. See the
///   [`Credentials`] struct for more information.
/// - `root_username`: The username of the root user. If not set, the default
///   username `postgres` is used.
///
/// The database must be created and managed beforehand. The tables are created
/// and dropped by the `root` user using the `init_schema` and `drop_schema`
/// methods.
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub(super) host: String,
    pub(super) port: Option<u16>,
    pub(super) database: String,
    pub(super) schema: Option<String>,
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
                "postgresql://{username}:{password}@{host}:{port}/{database}",
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

    #[inline]
    #[must_use]
    fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}

impl Database for DbConfig {
    fn root_username(&self) -> Option<&str> {
        self.root_username.as_deref().or(Some(DEFAULT_ROOT))
    }

    fn requires_credentials(&self) -> bool {
        true
    }

    #[instrument(skip(self, creds, coins))]
    async fn init_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: &[crate::Coin],
    ) -> Result<(), Error> {
        let root = self.root_username().unwrap();
        let creds = creds.unwrap_or_else(|| Credentials::new(root));
        let db = self.connect(&creds).await?;

        info!("Initializing schema for Postgres database");
        for coin in coins {
            info!("Creating table for {coin:#}");
            let table = coin.table_name();
            sqlx::query(&format!(
                "CREATE TABLE IF NOT EXISTS {schema}.{table} (
                    time_stamp TIMESTAMP WITH TIME ZONE NOT NULL,
                    time_frame VARCHAR(3) NOT NULL,
                    sources SMALLINT NOT NULL CHECK (sources > 0),
                    open DECIMAL(20, 10) NOT NULL,
                    high DECIMAL(20, 10) NOT NULL,
                    low DECIMAL(20, 10) NOT NULL,
                    close DECIMAL(20, 10) NOT NULL,
                    volume DECIMAL(20, 10) NOT NULL,
                    PRIMARY KEY (time_stamp, time_frame)
                )",
                schema = self.schema()
            ))
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
        coins: Option<&[crate::Coin]>,
    ) -> Result<(), Error> {
        let root = self.root_username().unwrap();
        let creds = creds.unwrap_or_else(|| Credentials::new(root));
        let db = self.connect(&creds).await?;

        info!("Dropping schema for Postgres database");
        if let Some(coins) = coins {
            for coin in coins {
                info!("Dropping table for {coin:#}");
                let table = coin.table_name();
                let query = format!(
                    "DROP TABLE IF EXISTS {schema}.{table}",
                    schema = self.schema()
                );

                sqlx::query(&query)
                    .execute(&db)
                    .await
                    .map_err(|err| Error::SqlDropTable(table, Box::new(err)))?;
            }
        } else {
            let query = format!(
                "SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname = '{}'",
                self.schema()
            );
            let tables = sqlx::query_as::<Db, (String,)>(&query)
                .fetch_all(&db)
                .await
                .map_err(|err| Error::SqlSelect(Box::new(err)))?;

            for table in tables {
                let table = table.0;
                info!("Dropping table `{schema}.{table}`", schema = self.schema());

                if table.starts_with(Coin::table_prefix()) {
                    let query = format!(
                        "DROP TABLE IF EXISTS {schema}.{table}",
                        schema = self.schema()
                    );

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
            && self.schema == other.schema
            && self.username == other.username
            && self.root_username == other.root_username
    }
}
