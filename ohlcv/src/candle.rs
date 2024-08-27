use std::{fmt, num::NonZero};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{Error, Timeframe};

/// Represents a candlestick in a trading pair.
///
/// A candlestick is a type of price chart that displays the high, low, open,
/// and closing prices of a cryptocurrency over a specific period of time. If
/// the price of the cryptocurrency increased over the period, the candlestick
/// is green. If the price decreased, the candlestick is red.
#[derive(Clone, Copy, Debug, Eq, Deserialize, Serialize)]
pub struct Candle {
    /// Start time of the candle in UTC
    pub timestamp: OffsetDateTime,
    /// Timeframe of the candle
    pub timeframe: Timeframe,
    /// Number of sources (exchanges) that contributed to the candle
    pub sources: NonZero<usize>,
    /// Open price of the candle in quote currency
    pub open: Decimal,
    /// High price of the candle in quote currency
    pub high: Decimal,
    /// Low price of the candle in quote currency
    pub low: Decimal,
    /// Close price of the candle in quote currency
    pub close: Decimal,
    /// Volume of the candle in quote currency
    pub volume: Decimal,
}

impl Candle {
    /// Merges many candles with the same timestamp and timeframe into a single
    /// candle.
    ///
    /// The price components (open, high, low, close) of the new candle are
    /// calculated by averaging the prices of the input candles weighted by
    /// their volumes (volume-weighted average, VWAP). The volume of the new
    /// candle is the sum of the volumes of the input candles.
    ///
    /// # Errors
    ///
    /// Returns an error if the input candles have different timestamps or
    /// timeframes or if the input iterator is empty.
    #[allow(clippy::missing_panics_doc)]
    pub fn merge<'a, I>(candles: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let mut timestamp = Option::<OffsetDateTime>::None;
        let mut timeframe = Option::<Timeframe>::None;
        let mut sources = 0;
        let mut open = Decimal::ZERO;
        let mut high = Decimal::ZERO;
        let mut low = Decimal::MAX;
        let mut close = Decimal::ZERO;
        let mut volume = Decimal::ZERO;

        for (index, candle) in candles.into_iter().enumerate() {
            if let Some(timestamp) = timestamp {
                if timestamp != candle.timestamp {
                    return Err(Error::MergeTimestamp(index, timestamp, candle.timestamp));
                }
            } else {
                timestamp = Some(candle.timestamp);
            }

            if let Some(timeframe) = timeframe {
                if timeframe != candle.timeframe {
                    return Err(Error::MergeTimeframe(index, timeframe, candle.timeframe));
                }
            } else {
                timeframe = Some(candle.timeframe);
            }

            sources += candle.sources.get();
            volume += candle.volume;
            open += candle.open * candle.volume;
            high += candle.high * candle.volume;
            low += candle.low * candle.volume;
            close += candle.close * candle.volume;
        }

        let open = open / volume;
        let high = high / volume;
        let low = low / volume;
        let close = close / volume;

        match (timestamp, timeframe) {
            (Some(timestamp), Some(timeframe)) => Ok(Self {
                timestamp,
                timeframe,
                // This is safe because the input iterator is not empty and the
                // sources are always greater than zero.
                sources: NonZero::new(sources).unwrap(),
                open,
                high,
                low,
                close,
                volume,
            }),
            _ => Err(Error::MergeEmpty),
        }
    }

    /// Returns the color of the candlestick.
    #[must_use]
    pub fn color(&self) -> Color {
        if self.close > self.open {
            Color::Green
        } else {
            Color::Red
        }
    }

    /// Returns the body of the candlestick.
    #[must_use]
    pub fn body(&self) -> Decimal {
        self.close - self.open
    }

    /// Returns the high wick of the candlestick.
    #[must_use]
    pub fn high_wick(&self) -> Decimal {
        self.high - self.close.max(self.open)
    }

    /// Returns the low wick of the candlestick.
    #[must_use]
    pub fn low_wick(&self) -> Decimal {
        self.open.min(self.close) - self.low
    }

    /// Returns the range of the candlestick.
    #[must_use]
    pub fn range(&self) -> Decimal {
        self.high - self.low
    }

    /// Returns the upper shadow of the candlestick.
    #[must_use]
    pub fn upper_shadow(&self) -> Decimal {
        self.high - self.close.max(self.open)
    }

    /// Returns the lower shadow of the candlestick.
    #[must_use]
    pub fn lower_shadow(&self) -> Decimal {
        self.open.min(self.close) - self.low
    }
}

impl PartialEq for Candle {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.timeframe == other.timeframe
    }
}

impl PartialOrd for Candle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.timestamp.cmp(&other.timestamp) {
            std::cmp::Ordering::Equal => self.timeframe.partial_cmp(&other.timeframe),
            ordering => Some(ordering),
        }
    }
}

impl Default for Candle {
    fn default() -> Self {
        Self {
            timestamp: OffsetDateTime::UNIX_EPOCH,
            timeframe: Timeframe::default(),
            sources: NonZero::new(1).unwrap(),
            open: Decimal::ZERO,
            high: Decimal::ZERO,
            low: Decimal::ZERO,
            close: Decimal::ZERO,
            volume: Decimal::ZERO,
        }
    }
}

/// Represents the color of a candlestick.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    /// The candlestick is green. This means that the price of the candlestick
    /// is higher than the opening price.
    Green,
    /// The candlestick is red. This means that the price of the candlestick
    /// is lower than the opening price.
    Red,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Green => write!(f, "green"),
            Self::Red => write!(f, "red"),
        }
    }
}
