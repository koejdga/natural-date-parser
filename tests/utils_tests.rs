#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, Local, TimeZone, Timelike, Weekday};
    use natural_date_parser::date_parser;
    use natural_date_parser::{ParseDateError, Rule};

    #[test]
    fn test_change_time_valid() {
        let now = Local::now();

        let new_time = date_parser::change_time(now, 16, 45);
        assert!(new_time.is_ok());

        let new_datetime = new_time.unwrap();
        assert_eq!(new_datetime.hour(), 16);
        assert_eq!(new_datetime.minute(), 45);
    }

    #[test]
    fn test_change_time_invalid_hour() {
        let now = Local::now();

        let new_time = date_parser::change_time(now, 25, 30);
        assert!(new_time.is_err());

        if let Err(ParseDateError::ParseError(msg)) = new_time {
            assert_eq!(msg, "Invalid date or time components");
        } else {
            panic!("Expected an error with invalid time");
        }
    }

    #[test]
    fn test_change_time_invalid_minute() {
        let now = Local::now();

        let new_time = date_parser::change_time(now, 14, 60);
        assert!(new_time.is_err());

        if let Err(ParseDateError::ParseError(msg)) = new_time {
            assert_eq!(msg, "Invalid date or time components");
        } else {
            panic!("Expected an error with invalid time");
        }
    }

    #[test]
    fn test_adjust_to_next_weekday() {
        // Monday
        let datetime = Local.with_ymd_and_hms(2024, 11, 11, 12, 0, 0).unwrap();

        let adjusted_date = date_parser::shift_to_weekday(datetime, Weekday::Fri, Rule::next);
        assert!(adjusted_date.is_ok());
        assert_eq!(adjusted_date.as_ref().unwrap().weekday(), Weekday::Fri);
        assert_eq!(adjusted_date.unwrap().year(), datetime.year());

        let adjusted_date = date_parser::shift_to_weekday(datetime, Weekday::Mon, Rule::next);
        assert!(adjusted_date.is_ok());
        assert_eq!(adjusted_date.as_ref().unwrap().weekday(), Weekday::Mon);
        assert_eq!(adjusted_date.unwrap().year(), datetime.year());
    }

    #[test]
    fn test_adjust_to_last_weekday() {
        let now = Local::now();
        let weekday = now.weekday();

        let adjusted_date = date_parser::shift_to_weekday(now, weekday, Rule::last);
        assert!(adjusted_date.is_ok());
        assert_eq!(adjusted_date.as_ref().unwrap().weekday(), weekday);
        assert_eq!(now - adjusted_date.unwrap(), Duration::days(7));
    }

    #[test]
    fn test_adjust_to_this_weekday() {
        let now = Local::now();
        let weekday = now.weekday();

        let adjusted_date = date_parser::shift_to_weekday(now, weekday, Rule::this);
        assert!(adjusted_date.is_ok());
        assert_eq!(adjusted_date.unwrap(), now);
    }
}
