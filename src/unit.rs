pub fn format_seconds(seconds: f64) -> String {
    if seconds < 1.0 {
        return format!("{:.2} s", seconds);
    } else if seconds < 60.0 {
        return format!("{:.2} s", seconds);
    } else if seconds < 3600.0 {
        let minutes = seconds / 60.0;
        return format!("{:.2} min", minutes);
    } else if seconds < 86400.0 {
        let hours = seconds / 3600.0;
        return format!("{:.2} hours", hours);
    } else if seconds < 2592000.0 {
        let days = seconds / 86400.0;
        return format!("{:.2} days", days);
    } else if seconds < 31536000.0 {
        let months = seconds / 2592000.0;
        return format!("{:.2} months", months);
    } else {
        let years = seconds / 31536000.0;
        return format!("{:.2} years", years);
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