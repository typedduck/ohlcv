use std::fmt;

use serde::{Deserialize, Serialize};

use crate::Currency;

/// Represents a cryptocurrency and its quote currency.
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Coin {
    symbol: Box<str>,
    name: Box<str>,
    currency: Currency,
}

impl Coin {
    /// Create a new [`Coin`].
    #[must_use]
    pub fn new(symbol: impl Into<String>, name: impl Into<String>, currency: Currency) -> Self {
        Self {
            symbol: symbol.into().to_uppercase().into_boxed_str(),
            name: name.into().into_boxed_str(),
            currency,
        }
    }

    /// The symbol of the coin.
    ///
    /// The symbol is used to identify the coin in the database and is part of
    /// the table name.
    #[must_use]
    #[inline]
    pub const fn symbol(&self) -> &str {
        &self.symbol
    }

    /// The human-readable name of the coin.
    #[must_use]
    #[inline]
    pub const fn name(&self) -> &str {
        &self.name
    }

    /// The quote currency of the coin.
    #[must_use]
    #[inline]
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// The prefix of the table name.
    #[must_use]
    #[inline]
    pub const fn table_prefix() -> &'static str {
        "candles"
    }

    /// The table name of the coin.
    ///
    /// The table name is used to identify the coin in the database. It is
    /// constructed from the table prefix, the symbol and the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use ohlcv::Coin;
    /// use ohlcv::Currency;
    ///
    /// let coin = Coin::new("BTC", "Bitcoin", Currency::USD);
    /// assert_eq!(coin.table_name(), "candles_btc_usd");
    /// ```
    #[must_use]
    pub fn table_name(&self) -> String {
        format!(
            "{}_{}_{}",
            Self::table_prefix(),
            self.symbol.to_lowercase(),
            self.currency.to_string().to_lowercase()
        )
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{} ({})", self.name, self.symbol)
        } else {
            write!(f, "{}", self.symbol)
        }
    }
}

impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
