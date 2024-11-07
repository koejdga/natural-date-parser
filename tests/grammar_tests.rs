use anyhow::{anyhow, Result};
use natural_date_parser::*;
use pest::Parser;

#[test]
fn test_relative_date_next_monday() -> Result<()> {
    let pair = Grammar::parse(Rule::relative_date, "next Monday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "next Monday");
    Ok(())
}

#[test]
fn test_relative_date_last_friday() -> Result<()> {
    let pair = Grammar::parse(Rule::relative_date, "last Friday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "last Friday");
    Ok(())
}

#[test]
fn test_invalid_relative_date_without_day() -> Result<()> {
    let parse_result = Grammar::parse(Rule::relative_date, "next");
    assert!(
        parse_result.is_err(),
        "Parsed an incomplete relative date 'next'"
    );
    Ok(())
}

#[test]
fn test_invalid_relative_date_invalid_day() -> Result<()> {
    let parse_result = Grammar::parse(Rule::relative_date, "next Funday");
    assert!(
        parse_result.is_err(),
        "Parsed an invalid relative date 'next Funday'"
    );
    Ok(())
}

#[test]
fn test_relative_term_today() -> Result<()> {
    let pair = Grammar::parse(Rule::relative_term, "today")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "today");
    Ok(())
}

#[test]
fn test_relative_term_yesterday() -> Result<()> {
    let pair = Grammar::parse(Rule::relative_term, "yesterday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "yesterday");
    Ok(())
}

#[test]
fn test_specific_day_and_time() -> Result<()> {
    let pair = Grammar::parse(Rule::specific_day_and_time, "Tuesday at 5:00 PM")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "Tuesday at 5:00 PM");
    Ok(())
}

#[test]
fn test_invalid_specific_day_and_time_missing_time() -> Result<()> {
    let parse_result = Grammar::parse(Rule::specific_day_and_time, "Tuesday at");
    assert!(
        parse_result.is_err(),
        "Parsed an incomplete specific day and time 'Tuesday at'"
    );
    Ok(())
}

#[test]
fn test_specific_day() -> Result<()> {
    let pair = Grammar::parse(Rule::specific_day, "Wednesday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "Wednesday");
    Ok(())
}

#[test]
fn test_specific_time_am() -> Result<()> {
    let pair = Grammar::parse(Rule::specific_time, "8:30 AM")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8:30 AM");
    Ok(())
}

#[test]
fn test_specific_time_pm() -> Result<()> {
    let pair = Grammar::parse(Rule::specific_time, "12:15 PM")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "12:15 PM");
    Ok(())
}

#[test]
fn test_invalid_specific_time_missing_am_pm() -> Result<()> {
    let parse_result = Grammar::parse(Rule::specific_time, "10:30");
    assert!(
        parse_result.is_err(),
        "Parsed an incomplete specific time '10:30'"
    );
    Ok(())
}

#[test]
fn test_future_time_in_days() -> Result<()> {
    let pair = Grammar::parse(Rule::future_time, "in 3 days")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "in 3 days");
    Ok(())
}

#[test]
fn test_future_time_in_weeks() -> Result<()> {
    let pair = Grammar::parse(Rule::future_time, "in 2 weeks")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "in 2 weeks");
    Ok(())
}

#[test]
fn test_invalid_future_time_missing_number() -> Result<()> {
    let parse_result = Grammar::parse(Rule::future_time, "in days");
    assert!(
        parse_result.is_err(),
        "Parsed an incomplete future time 'in days'"
    );
    Ok(())
}

#[test]
fn test_invalid_future_time_invalid_unit() -> Result<()> {
    let parse_result = Grammar::parse(Rule::future_time, "in 5 minutes");
    assert!(
        parse_result.is_err(),
        "Parsed an invalid future time 'in 5 minutes'"
    );
    Ok(())
}

#[test]
fn test_complete_date_expression_relative_date() -> Result<()> {
    let pair = Grammar::parse(Rule::date_expression, "next Monday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "next Monday");
    Ok(())
}

#[test]
fn test_complete_date_expression_specific_day_and_time() -> Result<()> {
    let pair = Grammar::parse(Rule::date_expression, "Tuesday at 5:00 PM")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "Tuesday at 5:00 PM");
    Ok(())
}

#[test]
fn test_invalid_expression() -> Result<()> {
    let parse_result = Grammar::parse(Rule::date_expression, "yesterday at");
    assert!(
        parse_result.is_err(),
        "Parsed an incomplete or invalid expression 'yesterday at'"
    );
    Ok(())
}

#[test]
fn test_invalid_date_expression_random_text() -> Result<()> {
    let parse_result = Grammar::parse(Rule::date_expression, "some random text");
    assert!(
        parse_result.is_err(),
        "Parsed an invalid date expression 'some random text'"
    );
    Ok(())
}
