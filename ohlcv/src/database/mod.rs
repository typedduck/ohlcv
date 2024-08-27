//! Module for database access and schema initialization.
//!
//! The module provides a trait [`Database`] that defines the interface for
//! interacting with a database. The trait is implemented for the [`DbType`]
//! type.
//!
//! The module also provides implementations for the following databases:
//!
//! - SQLite
//! - PostgreSQL
//! - MySQL/MariaDB
//!
//! The database can be accessed using the [`DbType`] type. The tables defining
//! the candles can be initialized and dropped using the `init_schema` and
//! `drop_schema` methods. All data definition is done by the `root` user. The
//! normal user only has access to the data. Exception to this is SQLite, where
//! no user management is needed.

use std::{fmt, future::Future};

use serde::de::DeserializeOwned;

use crate::{Coin, Error};

/// Trait for interacting with a database.
pub trait Database: DeserializeOwned + fmt::Debug {
    /// Get the username of the root user.
    ///
    /// If the database does not have a root user or does not require one, this
    /// method should return `None`. But if the methods
    /// [`has_root()`](Database::has_root) or
    /// [`requires_credentials()`](Database::requires_credentials) return
    /// `true`, this method should return the username of the root user.
    #[must_use]
    fn root_username(&self) -> Option<&str>;

    /// Check if the database has a root user.
    ///
    /// If the method [`requires_credentials()`](Database::requires_credentials)
    /// returns `true`, this method should return `true`.
    #[inline]
    #[must_use]
    fn has_root(&self) -> bool {
        self.root_username().is_some()
    }

    /// Return whether the database requires credentials.
    ///
    /// If the database requires credentials, the method
    /// [`root_username()`](Database::root_username) should return the username
    /// of the root user and the method [`has_root()`](Database::has_root)
    /// should return `true`.
    #[must_use]
    fn requires_credentials(&self) -> bool;

    /// Initialize the database schema.
    ///
    /// The credentials are optional and may be used to connect to the database
    /// as a alternative user. The coins are used to create the tables for the
    /// specified coins.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema could not be initialized.
    fn init_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: &[Coin],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Drop the database schema.
    ///
    /// The credentials are optional and may be used to connect to the database
    /// as a alternative user. The coins are used to drop the tables for the
    /// specified coins.
    ///
    /// If the coins are not specified, all tables are dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema could not be dropped.
    fn drop_schema(
        &mut self,
        creds: Option<Credentials>,
        coins: Option<&[Coin]>,
    ) -> impl Future<Output = Result<(), Error>>;
}

mod credentials;
pub use credentials::Credentials;

mod dbtype;
pub use dbtype::DbType;

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub mod mysql;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub mod sqlite;
