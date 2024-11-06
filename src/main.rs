use pest::Parser;
use anyhow::anyhow;
use natural_date_parser::*;

fn main() -> anyhow::Result< () > {
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    dbg!(pair);
    Ok(())
}