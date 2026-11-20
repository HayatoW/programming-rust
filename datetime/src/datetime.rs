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
    pub fn from_unix_timestamp(unix_timestamp: i64) {
        chrono::DateTime::from_timestamp(unix_timestamp, 0).ok_or(Error::OutOfRange).and_then(Self::from
    }
}

impl DateTime {
    fn from_chrono_date_time(
        chrono_date_time: chrono::DateTime<chrono::Utc>
    ) -> Result<Self, Error> {
        let unix_timestamp = chrono_date_time.timestamp();

    }
}
