#[cfg(test)]
mod tests {
    mod helping_functions {
        use chrono::{DateTime, Weekday};
        use chrono::{Datelike, Local};
        use natural_date_parser::ParseDateError;

        pub(super) fn assert_weekday_result(
            result: Result<Weekday, ParseDateError>,
            expected: Weekday,
        ) {
            match result {
                Ok(weekday) => assert_eq!(weekday, expected),
                Err(e) => panic!("Expected Ok but got error: {:?}", e),
            }
        }

        pub(super) fn assert_specific_day_result(
            result: Result<DateTime<Local>, ParseDateError>,
            expected_weekday: Weekday,
        ) {
            match result {
                Ok(datetime) => assert_eq!(datetime.weekday(), expected_weekday),
                Err(e) => panic!("Expected Ok but got error: {:?}", e),
            }
        }
    }

    #[cfg(test)]
    mod process_am_pm_tests {
        use natural_date_parser::date_parser::process_is_pm;
        use natural_date_parser::{DateParser, Rule};
        use pest::Parser;

        #[test]
        fn test_process_am_pm_for_pm() {
            let pair = DateParser::parse(Rule::am_pm, "PM")
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(process_is_pm(pair), Some(true));
        }

        #[test]
        fn test_process_am_pm_for_am() {
            let pair = DateParser::parse(Rule::am_pm, "AM")
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(process_is_pm(pair), Some(false));
        }

        #[test]
        fn test_process_am_pm_for_invalid_input() {
            let pair = DateParser::parse(Rule::date_expression, "today") // Use an appropriate invalid rule or input
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(process_is_pm(pair), None);
        }
    }

    #[cfg(test)]
    mod process_weekday_tests {
        use super::helping_functions::assert_weekday_result;
        use chrono::Weekday;
        use natural_date_parser::date_parser::process_weekday;
        use natural_date_parser::{ParseDateError, Rule};

        #[test]
        fn test_process_weekday_valid() {
            assert_weekday_result(process_weekday(Rule::monday), Weekday::Mon);
            assert_weekday_result(process_weekday(Rule::tuesday), Weekday::Tue);
            assert_weekday_result(process_weekday(Rule::wednesday), Weekday::Wed);
            assert_weekday_result(process_weekday(Rule::thursday), Weekday::Thu);
            assert_weekday_result(process_weekday(Rule::friday), Weekday::Fri);
            assert_weekday_result(process_weekday(Rule::saturday), Weekday::Sat);
            assert_weekday_result(process_weekday(Rule::sunday), Weekday::Sun);
        }

        #[test]
        fn test_process_weekday_invalid() {
            let invalid_weekday = Rule::EOI;
            let result = process_weekday(invalid_weekday);

            assert!(result.is_err());
            if let Err(ParseDateError::ParseError(msg)) = result {
                assert_ne!(msg, "");
            } else {
                panic!("Expected a message for an error for invalid weekday");
            }
        }
    }

    #[cfg(test)]
    mod process_specific_day_tests {
        use super::helping_functions::assert_specific_day_result;
        use chrono::{Local, Weekday};
        use natural_date_parser::date_parser::process_specific_day;
        use natural_date_parser::{ParseDateError, Rule};

        #[test]
        fn test_process_specific_day_valid() {
            let datetime = Local::now();

            let monday_rule = Rule::monday;
            let tuesday_rule = Rule::tuesday;
            let wednesday_rule = Rule::wednesday;
            let thursday_rule = Rule::thursday;
            let friday_rule = Rule::friday;
            let saturday_rule = Rule::saturday;
            let sunday_rule = Rule::sunday;

            assert_specific_day_result(process_specific_day(monday_rule, datetime), Weekday::Mon);
            assert_specific_day_result(process_specific_day(tuesday_rule, datetime), Weekday::Tue);
            assert_specific_day_result(
                process_specific_day(wednesday_rule, datetime),
                Weekday::Wed,
            );
            assert_specific_day_result(process_specific_day(thursday_rule, datetime), Weekday::Thu);
            assert_specific_day_result(process_specific_day(friday_rule, datetime), Weekday::Fri);
            assert_specific_day_result(process_specific_day(saturday_rule, datetime), Weekday::Sat);
            assert_specific_day_result(process_specific_day(sunday_rule, datetime), Weekday::Sun);
        }

        #[test]
        fn test_process_specific_day_invalid() {
            let invalid_rule = Rule::EOI;
            let datetime = Local::now();

            let result = process_specific_day(invalid_rule, datetime);

            assert!(result.is_err());
            if let Err(ParseDateError::ParseError(msg)) = result {
                assert_ne!(msg, "");
            } else {
                panic!("Expected an error for invalid weekday");
            }
        }

        #[test]
        fn test_process_specific_day_with_future_weekday() {
            let datetime = Local::now();
            let next_monday_rule = Rule::monday;
            let result = process_specific_day(next_monday_rule, datetime);

            assert_specific_day_result(result, Weekday::Mon);
        }
    }

    #[cfg(test)]
    mod process_specific_time_tests {
        use chrono::{DateTime, Local, TimeZone, Timelike};
        use natural_date_parser::date_parser::process_specific_time;
        use natural_date_parser::{DateParser, ParseDateError, Rule};
        use pest::iterators::Pair;
        use pest::Parser;

        fn get_test_datetime() -> DateTime<Local> {
            Local.with_ymd_and_hms(2024, 11, 11, 12, 0, 0).unwrap()
        }

        fn parse_input(input: &str) -> Result<Pair<'_, Rule>, pest::error::Error<Rule>> {
            let pair = DateParser::parse(Rule::specific_time, input)?;
            Ok(pair.into_iter().next().unwrap())
        }

        #[test]
        fn test_process_specific_time_am() {
            let datetime = get_test_datetime();
            let input = "9:45AM";

            let pair = parse_input(input).unwrap();

            let result = process_specific_time(pair, datetime);

            assert!(result.is_ok());
            let modified_datetime = result.unwrap();
            assert_eq!(modified_datetime.hour(), 9);
            assert_eq!(modified_datetime.minute(), 45);
        }

        #[test]
        fn test_process_specific_time_pm() {
            let datetime = get_test_datetime();
            let pair = parse_input("5:30PM").unwrap();

            let result = process_specific_time(pair, datetime);

            assert!(result.is_ok());
            let modified_datetime = result.unwrap();
            assert_eq!(modified_datetime.hour(), 17);
            assert_eq!(modified_datetime.minute(), 30);
        }

        #[test]
        fn test_process_specific_time_midnight() {
            let datetime = get_test_datetime();
            let pair = parse_input("12:00AM").unwrap();
            let result = process_specific_time(pair, datetime);

            assert!(result.is_ok());
            let modified_datetime = result.unwrap();
            assert_eq!(modified_datetime.hour(), 0);
            assert_eq!(modified_datetime.minute(), 0);
        }

        #[test]
        fn test_process_specific_time_noon() {
            let datetime = get_test_datetime();
            let pair = parse_input("12:00PM").unwrap();
            let result = process_specific_time(pair, datetime);

            assert!(result.is_ok());
            let modified_datetime = result.unwrap();
            assert_eq!(modified_datetime.hour(), 12);
            assert_eq!(modified_datetime.minute(), 0);
        }

        #[test]
        fn test_process_specific_time_invalid() {
            let datetime = get_test_datetime();

            let pair = parse_input("25:00PM").unwrap();
            let result = process_specific_time(pair, datetime);

            assert!(result.is_err());
            if let Err(ParseDateError::ParseError(msg)) = result {
                assert!(msg.contains("Invalid hour"));
            } else {
                panic!("Expected error for invalid time");
            }
        }

        #[test]
        fn test_process_specific_time_same_time() {
            let datetime = get_test_datetime();

            let pair = parse_input("10:30AM").unwrap(); // create Pair for "10:30AM" input;
            let result = process_specific_time(pair, datetime);

            assert!(result.is_ok());
            let modified_datetime = result.unwrap();
            assert_eq!(modified_datetime.hour(), 10);
            assert_eq!(modified_datetime.minute(), 30);
        }
    }

    #[cfg(test)]
    mod process_relative_term_tests {
        use chrono::DateTime;
        use chrono::{Datelike, Duration, Local};
        use natural_date_parser::date_parser::process_relative_term;
        use natural_date_parser::{DateParser, Rule};
        use pest::Parser;

        fn test_relative_term_rule(input: &str, expected_datetime: DateTime<Local>) {
            let pair = DateParser::parse(Rule::relative_term, input)
                .unwrap()
                .next()
                .unwrap();

            let result = process_relative_term(pair);
            assert!(result.is_ok());
            assert_eq!(result.as_ref().unwrap().year(), expected_datetime.year());
            assert_eq!(result.as_ref().unwrap().month(), expected_datetime.month());
            assert_eq!(result.unwrap().day(), expected_datetime.day());
        }

        #[test]
        fn test_process_relative_term_tomorrow() {
            let tomorrow = Local::now() + Duration::days(1);
            test_relative_term_rule("tomorrow", tomorrow);
        }

        #[test]
        fn test_process_relative_term_today() {
            let today = Local::now();
            test_relative_term_rule("today", today);
        }

        #[test]
        fn test_process_relative_term_yesterday() {
            let yesterday = Local::now() - Duration::days(1);
            test_relative_term_rule("yesterday", yesterday);
        }
    }

    #[cfg(test)]
    mod process_relative_date_tests {
        use chrono::{Datelike, Duration, Local};
        use natural_date_parser::date_parser::process_relative_date;
        use natural_date_parser::{DateParser, Rule};
        use pest::Parser;

        #[test]
        fn test_process_relative_date_next_monday() {
            let today_weekday = Local::now().weekday();
            let target_weekday = chrono::Weekday::Mon;
            let days_offset =
                if today_weekday.num_days_from_monday() <= target_weekday.num_days_from_monday() {
                    (target_weekday.num_days_from_monday() as i64)
                        - (today_weekday.num_days_from_monday() as i64)
                } else {
                    7 - ((today_weekday.num_days_from_monday() as i64)
                        - (target_weekday.num_days_from_monday() as i64))
                };

            let pair = DateParser::parse(Rule::relative_date, "next Monday")
                .unwrap()
                .next()
                .unwrap();

            let result = process_relative_date(pair);
            println!("res {:#?}", result);
            assert!(result.is_ok());

            let expected_date = Local::now() + Duration::days(days_offset);
            assert_eq!(result.unwrap().date_naive(), expected_date.date_naive());
        }
    }
}
