use anyhow::anyhow;
use natural_date_parser::*;
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::date_expression, "next Monday")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    dbg!(pair);
    Ok(())
}
