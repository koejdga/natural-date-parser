use {pest_derive::Parser, thiserror::Error};

/// A parser for date-related expressions using the `pest` parser library.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DateParser;

/// Enum representing errors that can occur while parsing a date.
#[derive(Debug, Error)]
pub enum ParseDateError {
    /// Error variant for failed date parsing. Includes the error message.
    #[error("Failed to parse date:\n{0}")]
    ParseError(String),
}

/// Module for parsing and processing date-related expressions.
pub mod date_parser {
    use {
        crate::{DateParser, ParseDateError, Rule},
        chrono::{DateTime, Datelike, Duration, Local, TimeZone, Weekday},
        chronoutil::delta::shift_months_opt,
        pest::{Parser, iterators::Pair},
    };

    /// Parses a string representing a date and returns the corresponding `DateTime<Local>`.
    ///
    /// This function takes a date string, parses it using the `pest` parser, and returns the
    /// resulting `DateTime<Local>` if successful, or an error if the string cannot be parsed.
    ///
    /// The reference date is automatically assumed to be Local::now().
    ///
    /// # Arguments
    /// * `string` - The string to be parsed as a date.
    ///
    /// # Returns
    /// * `Result<DateTime<Local>, ParseDateError>` - A `DateTime<Local>` if parsing is successful,
    ///   or a `ParseDateError` if there was an issue.
    pub fn from_string(string: &str) -> Result<DateTime<Local>, ParseDateError> {
        from_string_with_reference(string, Local::now())
    }

    /// Parses a string representing a date and returns the corresponding `DateTime<Local>`.
    ///
    /// This function takes a date string, parses it using the `pest` parser, and returns the
    /// resulting `DateTime<Local>` if successful, or an error if the string cannot be parsed.
    ///
    /// You specify a reference date from which to calculate relative dates. This allows for
    /// easier testing and better use in multi-timezone setups.
    ///
    /// # Arguments
    /// * `string` - The string to be parsed as a date.
    /// * `reference_date` - The DateTime representing "now", a moment from which relative dates and times will be calculated.
    ///
    /// # Returns
    /// * `Result<DateTime<Local>, ParseDateError>` - A `DateTime<Local>` if parsing is successful,
    ///   or a `ParseDateError` if there was an issue.
    pub fn from_string_with_reference(
        string: &str,
        reference_date: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let pairs = DateParser::parse(Rule::date_expression, string)
            .map_err(|e| ParseDateError::ParseError(e.to_string()))?;

        if let Some(pair) = pairs.clone().next() {
            match pair.as_rule() {
                Rule::date_expression => {
                    let datetime = process_date_expression(pair, reference_date)?;
                    return Ok(datetime);
                }
                _ => {
                    return Err(ParseDateError::ParseError(
                        "Unexpected rule encountered".to_string(),
                    ));
                }
            }
        }

        Err(ParseDateError::ParseError(
            "No valid date expression found".to_string(),
        ))
    }

    pub fn process_date_expression(
        pair: Pair<'_, Rule>,
        datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::relative_date => {
                    let parsed = process_relative_date(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                Rule::relative_term => {
                    let parsed = process_relative_term(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                Rule::specific_time => {
                    let parsed = process_specific_time(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                Rule::specific_day => {
                    if let Some(inner) = inner_pair.into_inner().next() {
                        let parsed = process_specific_day(inner.as_rule(), datetime)?;
                        return Ok(parsed);
                    }
                }
                Rule::specific_day_and_time => {
                    let parsed = process_specific_day_and_time(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                Rule::relative_day_and_specific_time => {
                    let parsed = process_relative_day_and_specific_time(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                Rule::future_time => {
                    let parsed = process_future_time(inner_pair, datetime)?;
                    return Ok(parsed);
                }
                _ => {
                    return Err(ParseDateError::ParseError(
                        "Unexpected rule encountered".to_string(),
                    ));
                }
            }
        }

        Err(ParseDateError::ParseError(
            "No date expression found".to_string(),
        ))
    }

    pub fn process_future_time(
        pair: Pair<'_, Rule>,
        mut datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let mut duration = 0;
        let mut unit: Option<Rule> = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::number => {
                    duration = inner_pair.as_str().trim().parse::<i32>().map_err(|_| {
                        ParseDateError::ParseError("Invalid duration value".to_string())
                    })?;
                }
                Rule::minute_s
                | Rule::hour_s
                | Rule::day_s
                | Rule::week_s
                | Rule::month_s
                | Rule::year_s => {
                    unit = Some(inner_pair.as_rule());
                }
                _ => {
                    return Err(ParseDateError::ParseError("Unexpected rule".to_string()));
                }
            }
        }

        if let Some(unit) = unit {
            datetime = match unit {
                Rule::minute_s => datetime + Duration::minutes(duration as i64),
                Rule::hour_s => datetime + Duration::hours(duration as i64),
                Rule::day_s => datetime + Duration::days(duration as i64),
                Rule::week_s => datetime + Duration::weeks(duration as i64),
                Rule::month_s => shift_months_opt(datetime, duration).ok_or_else(|| {
                    ParseDateError::ParseError("Invalid month adjustment".to_string())
                })?,
                Rule::year_s => {
                    datetime
                        .with_year(datetime.year() + duration)
                        .ok_or_else(|| {
                            ParseDateError::ParseError("Invalid year adjustment".to_string())
                        })?
                }
                _ => {
                    return Err(ParseDateError::ParseError("Invalid time unit".to_string()));
                }
            };
            Ok(datetime)
        } else {
            Err(ParseDateError::ParseError(
                "Time unit not provided".to_string(),
            ))
        }
    }

    pub fn process_specific_day_and_time(
        pair: Pair<'_, Rule>,
        mut datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::specific_day => {
                    datetime = process_specific_day(inner_pair.as_rule(), datetime)?;
                }
                Rule::specific_time => {
                    datetime = process_specific_time(inner_pair, datetime)?;
                }
                _ => {
                    return Err(ParseDateError::ParseError(format!(
                        "Unexpected rule in specific date and time: {:?}",
                        inner_pair.as_rule()
                    )));
                }
            }
        }
        Ok(datetime)
    }

    pub fn process_relative_day_and_specific_time(
        pair: Pair<'_, Rule>,
        mut datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::relative_date => {
                    datetime = process_relative_date(inner_pair, datetime)?;
                }
                Rule::relative_term => {
                    datetime = process_relative_term(inner_pair, datetime)?;
                }
                Rule::specific_time => {
                    datetime = process_specific_time(inner_pair, datetime)?;
                }
                _ => {}
            }
        }
        Ok(datetime)
    }

    pub fn process_relative_date(
        pair: Pair<'_, Rule>,
        datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let inner_pairs: Vec<_> = pair.clone().into_inner().collect();

        if inner_pairs.len() == 2 {
            let first_pair = &inner_pairs[0];
            let second_pair = &inner_pairs[1];

            if first_pair.as_rule() == Rule::next_or_last
                && second_pair.as_rule() == Rule::specific_day
            {
                let direction = first_pair.clone().into_inner().last().unwrap().as_rule();

                if let Some(inner_pair) = second_pair.clone().into_inner().next() {
                    match process_weekday(inner_pair.as_rule()) {
                        Ok(target_weekday) => {
                            return shift_to_weekday(datetime, target_weekday, direction);
                        }
                        Err(e) => {
                            return Err(ParseDateError::ParseError(format!(
                                "Unrecognized relative date: {:?}",
                                e.to_string()
                            )));
                        }
                    }
                }

                Err(ParseDateError::ParseError(format!(
                    "Unrecognized relative date: {:?}",
                    second_pair.to_string()
                )))
            } else {
                Err(ParseDateError::ParseError(
                    "Pair did not match expected structure for relative_date.".to_string(),
                ))
            }
        } else {
            Err(ParseDateError::ParseError(
                "Unexpected number of inner pairs in relative_date.".to_string(),
            ))
        }
    }

    pub fn process_relative_term(
        pair: Pair<'_, Rule>,
        datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        if let Some(inner_pair) = pair.clone().into_inner().next() {
            match inner_pair.as_rule() {
                Rule::tomorrow => {
                    return Ok(datetime + Duration::days(1));
                }
                Rule::today => {
                    return Ok(datetime);
                }
                Rule::yesterday => {
                    return Ok(datetime - Duration::days(1));
                }
                _ => {
                    return Err(ParseDateError::ParseError(format!(
                        "Unexpected relative term: {:?}",
                        pair
                    )));
                }
            }
        }

        Err(ParseDateError::ParseError(
            "Invalid relative term".to_string(),
        ))
    }

    pub fn process_specific_time(
        pair: Pair<'_, Rule>,
        datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let mut hour: u32 = 0;
        let mut minute: u32 = 0;
        let mut is_pm = false;

        // Iterate through inner pairs to capture hour, minute, and am_pm
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::hour => {
                    hour = inner_pair.as_str().parse::<u32>().map_err(|e| {
                        ParseDateError::ParseError(format!("Failed to parse hour: {}", e))
                    })?;

                    if hour > 23 {
                        return Err(ParseDateError::ParseError(format!(
                            "Invalid hour: {:?}",
                            hour
                        )));
                    }
                }
                Rule::minute => {
                    minute = inner_pair.as_str().parse::<u32>().map_err(|e| {
                        ParseDateError::ParseError(format!("Failed to parse minute: {}", e))
                    })?;
                }
                Rule::am_pm => {
                    if let Some(res) = process_is_pm(inner_pair) {
                        is_pm = res;
                    }
                }
                _ => {
                    return Err(ParseDateError::ParseError(
                        "Unexpected rule in specific_time".to_string(),
                    ));
                }
            }
        }

        if is_pm && hour < 12 {
            hour += 12;
        } else if !is_pm && hour == 12 {
            hour = 0;
        }

        let modified_datetime = change_time(datetime, hour, minute)?;

        Ok(modified_datetime)
    }

    pub fn process_specific_day(
        rule: Rule,
        datetime: DateTime<Local>,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let target_weekday = process_weekday(rule)?;
        let current_weekday = datetime.weekday();

        let target_day_num = target_weekday.num_days_from_sunday();
        let current_day_num = current_weekday.num_days_from_sunday();

        let days_difference = if target_day_num >= current_day_num {
            (target_day_num - current_day_num) as i64
        } else {
            -((current_day_num - target_day_num) as i64)
        };

        let target_date = datetime + Duration::days(days_difference);
        Ok(target_date)
    }

    pub fn process_weekday(day: Rule) -> Result<Weekday, ParseDateError> {
        match day {
            Rule::monday => Ok(Weekday::Mon),
            Rule::tuesday => Ok(Weekday::Tue),
            Rule::wednesday => Ok(Weekday::Wed),
            Rule::thursday => Ok(Weekday::Thu),
            Rule::friday => Ok(Weekday::Fri),
            Rule::saturday => Ok(Weekday::Sat),
            Rule::sunday => Ok(Weekday::Sun),
            _ => Err(ParseDateError::ParseError(format!(
                "Invalid weekday: {:?}",
                day
            ))),
        }
    }

    pub fn change_time(
        datetime: DateTime<Local>,
        hour: u32,
        minute: u32,
    ) -> Result<DateTime<Local>, ParseDateError> {
        match Local.with_ymd_and_hms(
            datetime.year(),
            datetime.month(),
            datetime.day(),
            hour,
            minute,
            0,
        ) {
            chrono::LocalResult::Single(new_datetime) => Ok(new_datetime),
            chrono::LocalResult::None => Err(ParseDateError::ParseError(
                "Invalid date or time components".to_string(),
            )),
            chrono::LocalResult::Ambiguous(_, _) => Err(ParseDateError::ParseError(
                "Ambiguous date and time".to_string(),
            )),
        }
    }

    pub fn shift_to_weekday(
        now: DateTime<Local>,
        target_weekday: Weekday,
        direction: Rule,
    ) -> Result<DateTime<Local>, ParseDateError> {
        let current_weekday = now.weekday();

        let num_from_curr = current_weekday.num_days_from_sunday() as i32;
        let num_from_target = target_weekday.num_days_from_sunday() as i32;

        let days_difference: i32 = match direction {
            Rule::next => {
                if num_from_target == 0 {
                    7 - num_from_curr + 7
                } else {
                    7 - num_from_curr + num_from_target
                }
            }

            Rule::last => {
                if num_from_target == 0 {
                    -num_from_curr
                } else {
                    -num_from_curr - 7 + num_from_target
                }
            }
            Rule::this => {
                let diff = (num_from_target as i64) - (num_from_curr as i64);
                if diff >= 0 {
                    diff as i32
                } else {
                    (diff + 7) as i32
                }
            }
            _ => -100,
        };

        if days_difference < -7 {
            return Err(ParseDateError::ParseError(format!(
                "Expected last, this or next, got {:?}",
                direction
            )));
        }

        // println!("days_difference {:?}", days_difference);

        Ok(now + Duration::days(days_difference as i64))
    }

    pub fn process_is_pm(pair: Pair<'_, Rule>) -> Option<bool> {
        if let Some(inner_pair) = pair.into_inner().next() {
            if inner_pair.as_rule() == Rule::pm {
                return Some(true);
            } else if inner_pair.as_rule() == Rule::am {
                return Some(false);
            } else {
                return None;
            }
        }
        None
    }
}
