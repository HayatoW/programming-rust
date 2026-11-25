/// 曜日
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<chrono::Weekday> for DayOfWeek {
    fn from(value: chrono::Weekday) -> Self {
        match value {
            chrono::Weekday::Sun => Self::Sunday,
            chrono::Weekday::Mon => Self::Monday,
            chrono::Weekday::Tue => Self::Tuesday,
            chrono::Weekday::Wed => Self::Wednesday,
            chrono::Weekday::Thu => Self::Thursday,
            chrono::Weekday::Fri => Self::Friday,
            chrono::Weekday::Sat => Self::Saturday
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("has macros")]
    HasMacros,
    #[error("out of range")]
    OutOfRange,
    #[error("parse")]
    Parse(#[source] chrono::ParseError)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub fn from_unix_timestamp(unix_timestamp: i64) -> Result<Self, Error> {
        chrono::DateTime::from_timestamp(unix_timestamp, 0)
            .ok_or(Error::OutOfRange)
            .and_then(Self::from_chrono_date_time)
    }

    pub fn max_unix_timestamp() -> i64 {
        253_402_300_799_i64
    }

    pub fn min_unix_timestamp() -> i64 {
        -62_135_596_800_i64
    }
}

impl DateTime {
    fn from_chrono_date_time(
        chrono_date_time: chrono::DateTime<chrono::Utc>
    ) -> Result<Self, Error> {
        let unix_timestamp = chrono_date_time.timestamp();
        if !(Self::min_unix_timestamp()..=Self::max_unix_timestamp()).contains(&unix_timestamp) {
            return Err(Error::OutOfRange);
        }
        if chrono_date_time != chrono::SubsecRound::trunc_subsecs(chrono_date_time, 3) {
            return Err(Error::HasMacros);
        }
        Ok(Self(chrono_date_time))
    }
}
