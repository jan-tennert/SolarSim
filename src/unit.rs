pub fn format_seconds(seconds: f64) -> String {
    if seconds < 1.0 {
        return format!("{:.2} s", seconds);
    } else if seconds < 60.0 {
        return format!("{:.2} s", seconds);
    } else if seconds < 3600.0 {
        let minutes = seconds / 60.0;
        return format!("{:.2} min", minutes);
    } else if seconds < 86400.0 {
        let hours = (seconds as i32) / 3600;
        let minutes = (seconds % 3600.0) as i32 / 60;
        return format!("{} hours {} minutes", hours, minutes);
    } else if seconds < 2592000.0 {
        let days = (seconds as i32) / 86400;
        let remaining_hours = (seconds % 86400.0) as i32 / 3600;
        return format!("{} days {} hours", days, remaining_hours);
    } else if seconds < 31536000.0 {
        let months = (seconds as i32) / 2592000;
        let remaining_days = (seconds % 2592000.0) as i32 / 386400;
        return format!("{} months {} days", months, remaining_days);
    } else {
        let years = (seconds as i32) / 31536000;
        let remaining_months = (seconds % 31536000.0) as i32 / 2592000;
        return format!("{} years {} months", years, remaining_months);
    }
}

pub fn format_length(distance: f32) -> String {
    let kilometers = distance / 1000.0;
    if kilometers < 1_000_000.0 {
        return format!("{:.2} km", kilometers);
    } else if kilometers < 1_000_000_000.0 {
        let millions = kilometers / 1_000_000.0;
        return format!("{:.2} million km", millions);
    } else {
        let billions = kilometers / 1_000_000_000.0;
        return format!("{:.2} billion km", billions);
    }
}