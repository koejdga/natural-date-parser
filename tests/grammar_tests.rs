#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use natural_date_parser::{DateParser, Rule};
    use pest::Parser;

    fn parse_rule(rule: Rule, input: &str) -> Result<()> {
        DateParser::parse(rule, input)
            .map(|mut pairs| {
                pairs
                    .next()
                    .ok_or_else(|| anyhow!("No pair found for rule"))
            })?
            .map(|_| ())
            .map_err(|e| {
                anyhow!(
                    "Failed to parse input `{}` with rule {:?}: {}",
                    input,
                    rule,
                    e
                )
            })
    }

    #[test]
    fn test_date_expression() -> Result<()> {
        let expressions = [
            "next Monday at 10:30AM",
            "tomorrow",
            "today",
            "yesterday",
            "next Wednesday",
            "Saturday",
            "in 2 weeks",
        ];
        for expr in expressions {
            parse_rule(Rule::date_expression, expr)?;
        }
        Ok(())
    }

    #[test]
    fn test_specific_day() -> Result<()> {
        let days = [
            "Monday",
            "monday",
            "Tuesday",
            "tuesday",
            "Wednesday",
            "wednesday",
            "Thursday",
            "thursday",
            "Friday",
            "friday",
            "Saturday",
            "saturday",
            "Sunday",
            "sunday",
        ];

        for day in days {
            parse_rule(Rule::specific_day, day)?;
        }
        Ok(())
    }

    #[test]
    fn test_specific_time() -> Result<()> {
        let times = ["10:30AM", "10:30am", "01:45PM", "1:45pm"];
        for time in times {
            parse_rule(Rule::specific_time, time)?;
        }
        Ok(())
    }

    #[test]
    fn test_relative_term() -> Result<()> {
        let terms = [
            "Tomorrow",
            "tomorrow",
            "Today",
            "today",
            "Yesterday",
            "yesterday",
        ];
        for term in terms {
            parse_rule(Rule::relative_term, term)?;
        }
        Ok(())
    }

    #[test]
    fn test_relative_date() -> Result<()> {
        let dates = [
            "Next Monday",
            "next Tuesday",
            "Last Friday",
            "last saturday",
        ];
        for date in dates {
            parse_rule(Rule::relative_date, date)?;
        }
        Ok(())
    }

    #[test]
    fn test_relative_day_and_specific_time() -> Result<()> {
        let expressions = ["next Monday at 10:30AM", "yesterday at 5:15pm"];
        for expr in expressions {
            parse_rule(Rule::relative_day_and_specific_time, expr)?;
        }
        Ok(())
    }

    #[test]
    fn test_future_time() -> Result<()> {
        let times = ["in 2 days", "in 3 weeks", "in 1 month", "in 5 years"];
        for time in times {
            parse_rule(Rule::future_time, time)?;
        }
        Ok(())
    }

    #[test]
    fn test_time_unit() -> Result<()> {
        let units = [
            "day", "days", "week", "weeks", "month", "months", "year", "years",
        ];
        for unit in units {
            parse_rule(Rule::time_unit, unit)?;
        }
        Ok(())
    }

    #[test]
    fn test_next_or_last() -> Result<()> {
        let words = ["next", "last", "this", "Next", "Last", "This"];
        for word in words {
            parse_rule(Rule::next_or_last, word)?;
        }
        Ok(())
    }

    #[test]
    fn test_am_pm() -> Result<()> {
        let am_pm_cases = ["AM", "am", "PM", "pm"];
        for case in am_pm_cases {
            parse_rule(Rule::am_pm, case)?;
        }
        Ok(())
    }

    #[test]
    fn test_specific_day_and_time() -> Result<()> {
        let valid_cases = vec![
            "Monday at 10:00 AM",
            "Wednesday at 5:30 PM",
            "Friday at 12:45 PM",
        ];

        for case in &valid_cases {
            parse_rule(Rule::specific_day_and_time, case).map_err(|e| {
                anyhow!("Expected specific_day_and_time to parse '{}': {}", case, e)
            })?;
        }

        let invalid_cases = vec![
            "Monday 10:00 AM",
            "Sunday at 5 PM",
            "at 10:00 AM",
            "Thursday at five PM",
            "Holiday at 10:00 AM",
        ];

        for case in &invalid_cases {
            let result = parse_rule(Rule::specific_day_and_time, case);
            assert!(
                result.is_err(),
                "Unexpectedly parsed invalid input: '{}'",
                case
            );
        }

        Ok(())
    }

    #[test]
    fn test_hour() -> Result<()> {
        let valid_cases = vec!["0", "12", "23", "9", "01"];
        for case in &valid_cases {
            parse_rule(Rule::hour, case)
                .map_err(|e| anyhow!("Failed to parse valid hour '{}': {}", case, e))?;
        }

        let invalid_cases = vec!["-1", "a3", "xx"];
        for case in &invalid_cases {
            assert!(
                parse_rule(Rule::hour, case).is_err(),
                "Parsed invalid hour '{}'",
                case
            );
        }
        Ok(())
    }

    #[test]
    fn test_minute() -> Result<()> {
        let valid_cases = vec!["0", "59", "30", "09"];
        for case in &valid_cases {
            parse_rule(Rule::minute, case)
                .map_err(|e| anyhow!("Failed to parse valid minute '{}': {}", case, e))?;
        }

        let invalid_cases = vec!["a9", "xx"];
        for case in &invalid_cases {
            assert!(
                parse_rule(Rule::minute, case).is_err(),
                "Parsed invalid minute '{}'",
                case
            );
        }
        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let valid_cases = vec!["1", "10", "999", "42"];
        for case in &valid_cases {
            parse_rule(Rule::number, case)
                .map_err(|e| anyhow!("Failed to parse valid number '{}': {}", case, e))?;
        }

        let invalid_cases = vec!["", "xx"];
        for case in &invalid_cases {
            assert!(
                parse_rule(Rule::number, case).is_err(),
                "Parsed invalid number '{}'",
                case
            );
        }
        Ok(())
    }

    #[test]
    fn test_monday() -> Result<()> {
        let days = vec!["Monday", "monday"];

        for day in days {
            parse_rule(Rule::monday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_tuesday() -> Result<()> {
        let days = vec!["Tuesday", "tuesday"];

        for day in days {
            parse_rule(Rule::tuesday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_wednesday() -> Result<()> {
        let days = vec!["Wednesday", "wednesday"];

        for day in days {
            parse_rule(Rule::wednesday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_thursday() -> Result<()> {
        let days = vec!["Thursday", "thursday"];

        for day in days {
            parse_rule(Rule::thursday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_friday() -> Result<()> {
        let days = vec!["Friday", "friday"];

        for day in days {
            parse_rule(Rule::friday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_saturday() -> Result<()> {
        let days = vec!["Saturday", "saturday"];

        for day in days {
            parse_rule(Rule::saturday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_sunday() -> Result<()> {
        let days = vec!["Sunday", "sunday"];

        for day in days {
            parse_rule(Rule::sunday, day)
                .map_err(|e| anyhow!("Failed to parse valid day '{}': {}", day, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_tommorow() -> Result<()> {
        let terms = vec!["Tomorrow", "tomorrow"];
        for term in terms {
            parse_rule(Rule::tomorrow, term)
                .map_err(|e| anyhow!("Failed to parse valid relative term '{}': {}", term, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_today() -> Result<()> {
        let terms = vec!["Today", "today"];
        for term in terms {
            parse_rule(Rule::today, term)
                .map_err(|e| anyhow!("Failed to parse valid relative term '{}': {}", term, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_yesterday() -> Result<()> {
        let terms = vec!["Yesterday", "yesterday"];
        for term in terms {
            parse_rule(Rule::yesterday, term)
                .map_err(|e| anyhow!("Failed to parse valid relative term '{}': {}", term, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_next() -> Result<()> {
        let directions = vec!["Next", "next"];
        for direction in directions {
            parse_rule(Rule::next, direction).map_err(|e| {
                anyhow!(
                    "Failed to parse valid relative direction term '{}': {}",
                    direction,
                    e
                )
            })?;
        }
        Ok(())
    }

    #[test]
    fn test_this() -> Result<()> {
        let directions = vec!["This", "this"];
        for direction in directions {
            parse_rule(Rule::this, direction).map_err(|e| {
                anyhow!(
                    "Failed to parse valid relative direction term '{}': {}",
                    direction,
                    e
                )
            })?;
        }
        Ok(())
    }

    #[test]
    fn test_last() -> Result<()> {
        let directions = vec!["Last", "last"];
        for direction in directions {
            parse_rule(Rule::last, direction).map_err(|e| {
                anyhow!(
                    "Failed to parse valid relative direction term '{}': {}",
                    direction,
                    e
                )
            })?;
        }
        Ok(())
    }

    #[test]
    fn test_days() -> Result<()> {
        let units = vec!["day", "days"];
        for unit in units {
            parse_rule(Rule::day_s, unit)
                .map_err(|e| anyhow!("Failed to parse valid time unit '{}': {}", unit, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_weeks() -> Result<()> {
        let units = vec!["week", "weeks"];
        for unit in units {
            parse_rule(Rule::week_s, unit)
                .map_err(|e| anyhow!("Failed to parse valid time unit '{}': {}", unit, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_months() -> Result<()> {
        let units = vec!["month", "months"];
        for unit in units {
            parse_rule(Rule::month_s, unit)
                .map_err(|e| anyhow!("Failed to parse valid time unit '{}': {}", unit, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_years() -> Result<()> {
        let units = vec!["year", "years"];
        for unit in units {
            parse_rule(Rule::year_s, unit)
                .map_err(|e| anyhow!("Failed to parse valid time unit '{}': {}", unit, e))?;
        }
        Ok(())
    }

    #[test]
    fn test_whitespace_empty_output() -> Result<()> {
        let result = DateParser::parse(Rule::WHITESPACE, " ")
            .map_err(|e| anyhow!("Failed to parse ' ' as WHITESPACE: {}", e))?;

        assert!(
            result.clone().next().is_none(),
            "Expected an empty result, but got pairs: {:?}",
            result
        );

        Ok(())
    }
}
