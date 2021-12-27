use std::{cmp::min, convert::TryFrom, fmt, str::FromStr};

use chrono::{DateTime, Datelike as _, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidDate {
    #[error("invlaid year")]
    InvalidYear,
    #[error("invalid month")]
    InvalidMonth,
}

#[derive(Copy, Clone, Debug)]
pub struct LaxDate {
    year: Year,
    month: Option<u8>,
    day: Option<u8>,
}

impl LaxDate {
    pub fn year(self) -> Year {
        self.year
    }

    pub fn month(self) -> Option<Month> {
        self.month
            .map(|m| Month(self.year.0 * 12 + u16::from(m) - 1))
    }
}

impl FromStr for LaxDate {
    type Err = InvalidDate;

    fn from_str(s: &str) -> Result<LaxDate, InvalidDate> {
        let mut parts = s.splitn(3, '.');
        let year_part = parts.next().expect("non-empty split");
        Ok(LaxDate {
            year: Year::try_from(
                year_part
                    .parse::<u16>()
                    .map_err(|_| InvalidDate::InvalidYear)?,
            )?,
            month: parts
                .next()
                .and_then(|m| m.parse().ok())
                .filter(|m| 1 <= *m && *m <= 12),
            day: parts.next().and_then(|d| d.parse().ok()),
        })
    }
}

impl fmt::Display for LaxDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}.", self.year.0)?;
        match self.month {
            Some(month) => write!(f, "{:02}.", month)?,
            None => f.write_str("??.")?,
        }
        match self.day {
            Some(day) => write!(f, "{:02}", day),
            None => f.write_str("??"),
        }
    }
}

const MAX_YEAR: u16 = 3000; // MAX_YEAR * 12 + 12 < 2^16

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Year(u16);

impl Year {
    pub fn max_value() -> Year {
        Year(MAX_YEAR)
    }

    pub fn min_masters() -> Year {
        Year(1952)
    }

    pub fn max_masters() -> Year {
        Year(2021)
    }

    pub fn add_years_saturating(self, years: u16) -> Year {
        min(Year(self.0.saturating_add(years)), Year::max_value())
    }
}

impl From<Year> for u16 {
    fn from(Year(year): Year) -> u16 {
        year
    }
}

impl TryFrom<u16> for Year {
    type Error = InvalidDate;

    fn try_from(year: u16) -> Result<Year, InvalidDate> {
        if year <= Year::max_value().0 {
            Ok(Year(year))
        } else {
            Err(InvalidDate::InvalidYear)
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Month(u16);

impl Month {
    pub fn max_value() -> Month {
        Month(MAX_YEAR * 12 + 11)
    }

    pub fn from_time_saturating(time: DateTime<Utc>) -> Month {
        let year = match time.year_ce() {
            (true, ce) => min(u32::from(MAX_YEAR), ce) as u16,
            (false, _) => 0,
        };

        Month(year * 12 + time.month0() as u16)
    }

    pub fn add_months_saturating(self, months: u16) -> Month {
        min(Month(self.0.saturating_add(months)), Month::max_value())
    }

    pub fn year(self) -> Year {
        Year(self.0 / 12)
    }
}

impl From<Month> for u16 {
    fn from(Month(month): Month) -> u16 {
        month
    }
}

impl TryFrom<u16> for Month {
    type Error = InvalidDate;

    fn try_from(month: u16) -> Result<Month, InvalidDate> {
        if month <= Month::max_value().0 {
            Ok(Month(month))
        } else {
            Err(InvalidDate::InvalidMonth)
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.0 / 12, self.0 % 12 + 1)
    }
}

impl FromStr for Month {
    type Err = InvalidDate;

    fn from_str(s: &str) -> Result<Month, InvalidDate> {
        match s.split_once(|ch| ch == '-' || ch == '/') {
            Some((year_part, month_part)) => {
                let year: u16 = year_part.parse().map_err(|_| InvalidDate::InvalidMonth)?;
                let month_plus_one: u16 =
                    month_part.parse().map_err(|_| InvalidDate::InvalidMonth)?;

                if year <= MAX_YEAR && 1 <= month_plus_one && month_plus_one <= 12 {
                    Ok(Month(year * 12 + month_plus_one - 1))
                } else {
                    Err(InvalidDate::InvalidMonth)
                }
            }
            None => Err(InvalidDate::InvalidMonth),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Arbitrary for Month {
        fn arbitrary(g: &mut Gen) -> Month {
            Month(u16::arbitrary(g) % (u16::from(Month::max_value()) + 1))
        }
    }
}
