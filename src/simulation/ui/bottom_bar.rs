use chrono::{Days, NaiveDateTime};

pub fn get_date_from_millis(start: i64, millis: f32) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_millis(start)
        .unwrap()
        .checked_add_days(Days::new((((millis * 100.0).round()) / 100.0) as u64))
        .unwrap()
}