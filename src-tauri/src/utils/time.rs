// Time utilities

/// Format seconds to HH:MM:SS string
pub fn format_timestamp(secs: f64) -> String {
    let total_secs = secs as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    
    if hours > 0 {
        format!("{:02}h{:02}m{:02}s", hours, minutes, seconds)
    } else {
        format!("{:02}m{:02}s", minutes, seconds)
    }
}

/// Format seconds to display string (e.g., "1:23:45")
pub fn format_duration(secs: f64) -> String {
    let total_secs = secs as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}

/// Parse timestamp string to seconds
pub fn parse_timestamp(timestamp: &str) -> Option<f64> {
    let parts: Vec<&str> = timestamp.split(':').collect();
    
    match parts.len() {
        2 => {
            // MM:SS
            let minutes: f64 = parts[0].parse().ok()?;
            let seconds: f64 = parts[1].parse().ok()?;
            Some(minutes * 60.0 + seconds)
        }
        3 => {
            // HH:MM:SS
            let hours: f64 = parts[0].parse().ok()?;
            let minutes: f64 = parts[1].parse().ok()?;
            let seconds: f64 = parts[2].parse().ok()?;
            Some(hours * 3600.0 + minutes * 60.0 + seconds)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(65.0), "01m05s");
        assert_eq!(format_timestamp(3665.0), "01h01m05s");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(65.0), "1:05");
        assert_eq!(format_duration(3665.0), "1:01:05");
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp("1:05"), Some(65.0));
        assert_eq!(parse_timestamp("1:01:05"), Some(3665.0));
    }
}
