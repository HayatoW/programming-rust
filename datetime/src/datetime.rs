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

