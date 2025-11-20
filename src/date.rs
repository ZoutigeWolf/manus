use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use chrono_tz::Europe::Amsterdam;
use chrono_tz::Tz;

pub fn parse_date(days: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() + Duration::days(days as i64)
}

pub fn parse_time(minutes: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(minutes / 60, minutes % 60, 0).unwrap()
}

pub fn parse_datetime(days: u32, minutes: u32) -> DateTime<Tz> {
    Amsterdam
        .from_local_datetime(&parse_date(days).and_time(parse_time(minutes)))
        .unwrap()
}

pub fn to_string(dt: DateTime<impl TimeZone>) -> String {
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);

    format!("{}", utc.format("%Y%m%dT%H%M%SZ"))
}
