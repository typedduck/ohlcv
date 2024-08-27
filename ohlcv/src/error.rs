use std::{error::Error as StdError, fmt};

use time::OffsetDateTime;

use crate::Timeframe;

/// Error type.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum Error {
    /// SQLx common error.
    SqlCommon(Box<sqlx::Error>),
    /// Failed to connect to the database.
    SqlConnect(String, Box<sqlx::Error>),
    /// Failed to create table.
    SqlCreateTable(String, Box<sqlx::Error>),
    /// Failed to drop table.
    SqlDropTable(String, Box<sqlx::Error>),
    /// Failed to drop type.
    SqlDropType(String, Box<sqlx::Error>),
    // Failed to select rows.
    SqlSelect(Box<sqlx::Error>),
    /// Iterator of candles to merge is empty.
    MergeEmpty,
    /// Timeframes of candles to merge are not equal.
    MergeTimeframe(usize, Timeframe, Timeframe),
    /// Timestamps of candles to merge are not equal.
    MergeTimestamp(usize, OffsetDateTime, OffsetDateTime),
    /// Password is missing for the user.
    MissingPassword(String),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::SqlCommon(err)
            | Self::SqlConnect(_, err)
            | Self::SqlCreateTable(_, err)
            | Self::SqlDropTable(_, err)
            | Self::SqlDropType(_, err)
            | Self::SqlSelect(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SqlConnect(a, err_a), Self::SqlConnect(b, err_b))
            | (Self::SqlCreateTable(a, err_a), Self::SqlCreateTable(b, err_b))
            | (Self::SqlDropTable(a, err_a), Self::SqlDropTable(b, err_b))
            | (Self::SqlDropType(a, err_a), Self::SqlDropType(b, err_b)) => {
                a == b && err_a.to_string() == err_b.to_string()
            }
            (Self::SqlCommon(err_a), Self::SqlCommon(err_b))
            | (Self::SqlSelect(err_a), Self::SqlSelect(err_b)) => {
                err_a.to_string() == err_b.to_string()
            }
            (Self::MergeEmpty, Self::MergeEmpty) => true,
            (Self::MergeTimeframe(a, t1_a, t2_a), Self::MergeTimeframe(b, t1_b, t2_b)) => {
                a == b && t1_a == t1_b && t2_a == t2_b
            }
            (Self::MergeTimestamp(a, t1_a, t2_a), Self::MergeTimestamp(b, t1_b, t2_b)) => {
                a == b && t1_a == t1_b && t2_a == t2_b
            }
            (Self::MissingPassword(a), Self::MissingPassword(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SqlCommon(err) => err.fmt(f),
            Self::SqlConnect(user, err) => {
                write!(f, "failed to connect user `{user}` to the database: {err}")
            }
            Self::SqlCreateTable(table, err) => {
                write!(f, "failed to create table `{table}`: {err}")
            }
            Self::SqlDropTable(table, err) => {
                write!(f, "failed to drop table `{table}`: {err}")
            }
            Self::SqlDropType(typename, err) => {
                write!(f, "failed to drop type `{typename}`: {err}")
            }
            Self::SqlSelect(err) => {
                write!(f, "failed to select rows: {err}")
            }
            Self::MergeEmpty => {
                write!(f, "failed to merge candles: iterator is empty")
            }
            Self::MergeTimeframe(index, a, b) => {
                write!(
                    f,
                    "timeframes of candles at index {index} do not match: {a} and {b}"
                )
            }
            Self::MergeTimestamp(index, a, b) => {
                write!(
                    f,
                    "timestamps of candles at index {index} do not match: {a} and {b}"
                )
            }
            Self::MissingPassword(username) => {
                write!(f, "missing password for user: {username}")
            }
        }
    }
}
