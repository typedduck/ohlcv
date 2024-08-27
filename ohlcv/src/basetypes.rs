use std::{fmt, ops::RangeBounds, str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The type of currency.
///
/// Currency is used as the quote currency for price and volume and can be
/// specified in the coin section of the configuration file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
    /// US-Dollar
    USD,
    /// Euro
    EUR,
    /// British Pound
    GBP,
    /// Japanese Yen
    JPY,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::USD => write!(f, "USD"),
            Self::EUR => write!(f, "EUR"),
            Self::GBP => write!(f, "GBP"),
            Self::JPY => write!(f, "JPY"),
        }
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Self::USD),
            "EUR" => Ok(Self::EUR),
            "GBP" => Ok(Self::GBP),
            "JPY" => Ok(Self::JPY),
            _ => Err(s.to_string()),
        }
    }
}

/// The type of timeframe.
///
/// Timeframes are used to group the data into intervals of time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    #[serde(alias = "5m")]
    FiveMinutes,
    #[serde(alias = "15m")]
    Quarters,
    #[serde(alias = "1h")]
    OneHour,
    #[serde(alias = "4h")]
    FourHours,
    #[serde(alias = "1d")]
    OneDay,
}

const DURATION_5M: Duration = Duration::from_secs(5 * 60);
const DURATION_15M: Duration = Duration::from_secs(15 * 60);
const DURATION_1H: Duration = Duration::from_secs(60 * 60);
const DURATION_4H: Duration = Duration::from_secs(4 * 60 * 60);
const DURATION_1D: Duration = Duration::from_secs(24 * 60 * 60);

impl Timeframe {
    /// Get the duration of the timeframe.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        match self {
            Self::FiveMinutes => DURATION_5M,
            Self::Quarters => DURATION_15M,
            Self::OneHour => DURATION_1H,
            Self::FourHours => DURATION_4H,
            Self::OneDay => DURATION_1D,
        }
    }

    /// Round the given time down to the nearest timeframe.
    #[must_use]
    #[allow(clippy::missing_panics_doc, clippy::cast_possible_wrap)]
    pub fn round_down(&self, time: OffsetDateTime) -> OffsetDateTime {
        let duration = self.duration().as_secs() as i64;
        let seconds = time.unix_timestamp();
        let seconds = seconds - seconds.rem_euclid(duration);

        // This always succeeds, as the seconds are valid.
        OffsetDateTime::from_unix_timestamp(seconds).unwrap()
    }

    /// Round the given time up to the nearest timeframe.
    #[must_use]
    #[allow(clippy::missing_panics_doc, clippy::cast_possible_wrap)]
    pub fn round_up(&self, time: OffsetDateTime) -> OffsetDateTime {
        let duration = self.duration().as_secs() as i64;
        let seconds = time.unix_timestamp();
        let seconds = seconds + duration - seconds.rem_euclid(duration);

        // This always succeeds, as the seconds are valid.
        OffsetDateTime::from_unix_timestamp(seconds).unwrap()
    }

    /// Return the start and end time of range.
    ///
    /// The start time is rounded down to the nearest timeframe if the bound is
    /// included and rounded up if the bound is excluded. The end time is
    /// rounded up to the nearest timeframe if the bound is included and rounded
    /// down if the bound is excluded. If unbound the start time is the start of
    /// the Unix epoch and the end time is the end of the current excluded
    /// timeframe.
    #[must_use]
    pub fn range<R>(&self, range: R) -> (OffsetDateTime, OffsetDateTime)
    where
        R: RangeBounds<OffsetDateTime>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(start) => self.round_down(*start),
            std::ops::Bound::Excluded(start) => self.round_up(*start),
            std::ops::Bound::Unbounded => OffsetDateTime::UNIX_EPOCH,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(end) => self.round_up(*end),
            std::ops::Bound::Excluded(end) => self.round_down(*end),
            std::ops::Bound::Unbounded => self.round_down(OffsetDateTime::now_utc()),
        };

        (start, end)
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FiveMinutes => write!(f, "5m"),
            Self::Quarters => write!(f, "15m"),
            Self::OneHour => write!(f, "1h"),
            Self::FourHours => write!(f, "4h"),
            Self::OneDay => write!(f, "1d"),
        }
    }
}

impl Ord for Timeframe {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.duration().cmp(&other.duration())
    }
}

impl PartialOrd for Timeframe {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.duration().cmp(&other.duration()))
    }
}

impl FromStr for Timeframe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "5m" => Ok(Self::FiveMinutes),
            "15m" => Ok(Self::Quarters),
            "1h" => Ok(Self::OneHour),
            "4h" => Ok(Self::FourHours),
            "1d" => Ok(Self::OneDay),
            _ => Err(s.to_string()),
        }
    }
}

impl TryFrom<Duration> for Timeframe {
    type Error = String;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        match duration {
            DURATION_5M => Ok(Self::FiveMinutes),
            DURATION_15M => Ok(Self::Quarters),
            DURATION_1H => Ok(Self::OneHour),
            DURATION_4H => Ok(Self::FourHours),
            DURATION_1D => Ok(Self::OneDay),
            _ => Err(duration.as_secs().to_string()),
        }
    }
}

impl Default for Timeframe {
    fn default() -> Self {
        Self::FiveMinutes
    }
}
