use pest::Parser;
use anyhow::anyhow;
use natural_date_parser::*;

#[test]
fn field_test() -> anyhow::Result< () > {


    // Тестуємо простий числовий вхід
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    // Тест на неправильний вхід
    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());
    // Тест на порожній вхід
    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}


#[test]
fn record_test() -> anyhow::Result< () > {

    // Тестуємо правильний формат запису
    let pair = Grammar::parse(Rule::record, "-273.15,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15,99" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );

    // Неправильний запис без чисел
    let pair = Grammar::parse(Rule::record, "");
    assert!( pair.is_err() );

    Ok( () )
}
