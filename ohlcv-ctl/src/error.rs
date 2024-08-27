use std::{error::Error as StdError, fmt};

/// Error type for the CLI.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum Error {
    /// Failed to ask password.
    AskPassword(String, Box<inquire::error::InquireError>),
    /// Unknown command name.
    CommandName(String),
    /// Configuration file is missing.
    ConfigFile,
    /// Failed to parse configuration file.
    ConfigFormat(toml::de::Error),
    /// Failed to read or write to a file.
    Io(std::io::Error),
    /// Error returned by the OHLCV crate.
    Ohlcv(ohlcv::Error),
}

impl StdError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::AskPassword(_, err) => Some(err.as_ref()),
            Self::CommandName(_) | Self::ConfigFile => None,
            Self::ConfigFormat(err) => Some(err),
            Self::Io(err) => Some(err),
            Self::Ohlcv(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AskPassword(name, err) => {
                write!(f, "Failed to ask password for '{name}': {err}")
            }
            Self::CommandName(name) => write!(f, "Unknown command name: '{name}'"),
            Self::ConfigFile => write!(f, "Configuration file is missing"),
            Self::ConfigFormat(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::Ohlcv(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ohlcv::Error> for Error {
    #[inline]
    fn from(err: ohlcv::Error) -> Self {
        Self::Ohlcv(err)
    }
}

impl From<toml::de::Error> for Error {
    #[inline]
    fn from(err: toml::de::Error) -> Self {
        Self::ConfigFormat(err)
    }
}
