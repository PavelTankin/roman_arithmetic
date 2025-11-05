use anyhow::anyhow;
use roman_arithmetic::*;
use pest::Parser;

#[test]
fn test_one_digit()-> anyhow::Result<()> {
    let input = "X";
    let result = Grammar::parse(Rule::expr, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(result.as_str(), "X");

    Ok(())
}