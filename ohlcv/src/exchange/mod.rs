use serde::{Deserialize, Serialize};

/// The type of exchange.
///
/// This is a convenience enum to allow the use of different exchange types in a
/// configuration file. The enum is serialized and deserialized using the
/// `serde` crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    /// The Binance exchange.
    Binance,
    /// The KuCoin exchange.
    KuCoin,
}
