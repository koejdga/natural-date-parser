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
}
