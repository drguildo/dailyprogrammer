use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    for format in [
        "%s.%l",
        "%s:%m:%h %M/%d/%y",
        "The minute is %m! The hour is %h.",
    ] {
        println!("{}", localize(format, now));
    }
}

fn localize(format: &str, now: SystemTime) -> String {
    let elapsed = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let days_since_epoch = (elapsed.as_secs() / 86_400) as i64;
    let seconds_today = elapsed.as_secs() % 86_400;
    let milliseconds = elapsed.subsec_millis();

    let (year, month, day) = civil_from_days(days_since_epoch);
    let hour_24 = (seconds_today / 3_600) as u32;
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        hour => hour,
    };
    let minute = (seconds_today / 60 % 60) as u32;
    let second = (seconds_today % 60) as u32;
    let am_pm = if hour_24 < 12 { "AM" } else { "PM" };

    let mut result = String::with_capacity(format.len());
    let mut chars = format.chars();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            result.push(ch);
            continue;
        }

        match chars.next() {
            Some('l') => result.push_str(&format!("{milliseconds:03}")),
            Some('s') => result.push_str(&format!("{second:02}")),
            Some('m') => result.push_str(&minute.to_string()),
            Some('h') => result.push_str(&hour_12.to_string()),
            Some('H') => result.push_str(&hour_24.to_string()),
            Some('c') => result.push_str(am_pm),
            Some('d') => result.push_str(&day.to_string()),
            Some('M') => result.push_str(&format!("{month:02}")),
            Some('y') => result.push_str(&format!("{year:04}")),
            Some(other) => {
                result.push('%');
                result.push(other);
            }
            None => result.push('%'),
        }
    }

    result
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = month_part + if month_part < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);

    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::localize;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn formats_the_challenge_samples() {
        let sample_time = UNIX_EPOCH + Duration::new(1_373_360_792, 429_000_000);

        assert_eq!(localize("%s.%l", sample_time), "32.429");
        assert_eq!(
            localize("%s:%m:%h %M/%d/%y", sample_time),
            "32:6:9 07/9/2013"
        );
        assert_eq!(
            localize("The minute is %m! The hour is %h.", sample_time),
            "The minute is 6! The hour is 9."
        );
        assert_eq!(localize("%H %c", sample_time), "9 AM");
    }

    #[test]
    fn preserves_unknown_and_trailing_percent_sequences() {
        assert_eq!(localize("100% done: %q%", UNIX_EPOCH), "100% done: %q%");
    }

    #[test]
    fn handles_midnight_as_twelve_am() {
        assert_eq!(localize("%h %H %c", UNIX_EPOCH), "12 0 AM");
    }
}
