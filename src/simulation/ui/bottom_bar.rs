use chrono::{DateTime, Duration, Utc};

pub fn get_date_from_seconds(start: i64, seconds: f32) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(start)
        .unwrap()
        .checked_add_signed(Duration::seconds((seconds as f64).round() as i64))
     //   .checked_add_days(Days::new((((millis * 100.0).round()) / 100.0) as u64))
        .unwrap()
}