use jiff::{SignedDuration, Timestamp};

pub fn get_date_from_seconds(start: i64, seconds: f32) -> Timestamp {
    Timestamp::from_millisecond(start)
        .unwrap()
        .checked_add(SignedDuration::from_secs((seconds as f64).round() as i64))
        .unwrap()
}