use anyhow::anyhow;
use roman_arithmetic::*;
use pest::Parser;

// === Roman ===
#[test]
fn test_roman() -> anyhow::Result<()> {
    let inputs = ["IX", "LII", "MDCXX", "I", "L", "CX", "CCC", "MMMCMXCIX"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(result.as_str(), input, "Failed on input: {}", input);
    }
    Ok(())
}

#[test]
fn test_fail_roman() -> anyhow::Result<()> {
    let inputs = ["", "A", "i", "AXA", " "];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman, input);
        assert!(result.is_err(), "Have to fail on input: {} but {:?}", input, result);
    }
    Ok(())
}

// === Factor ===
#[test]
fn test_factor() -> anyhow::Result<()> {
    let inputs = ["X", "(V + II)", "-IX", "+L"];
    for &input in &inputs {
        let pair = Grammar::parse(Rule::factor, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), input, "Failed on factor: {}", input);
    }
    Ok(())
}

// === Strong Term ===
#[test]
fn test_strong_term() -> anyhow::Result<()> {
    let inputs = ["X", "V*II", "III*IV*V"];
    for &input in &inputs {
        let pair = Grammar::parse(Rule::strong_term, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), input, "Failed on strong_term: {}", input);
    }
    Ok(())
}

// === Term ===
#[test]
fn test_term() -> anyhow::Result<()> {
    let inputs = ["X+V", "II*V+X", "I+II+III"];
    for &input in &inputs {
        let pair = Grammar::parse(Rule::term, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), input, "Failed on term: {}", input);
    }
    Ok(())
}

// === Expr ===
#[test]
fn test_expr() -> anyhow::Result<()> {
    let inputs = ["X+V*II", "III*(IV+V)-VI"];
    for &input in &inputs {
        let pair = Grammar::parse(Rule::expr, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), input, "Failed on expr: {}", input);
    }
    Ok(())
}

// === Group ===
#[test]
fn test_group() -> anyhow::Result<()> {
    let inputs = ["(X+V)", "(II*III)"];
    for &input in &inputs {
        let pair = Grammar::parse(Rule::group, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), input, "Failed on group: {}", input);
    }
    Ok(())
}

// === Operators ===
#[test]
fn test_operators() -> anyhow::Result<()> {
    let add = Grammar::parse(Rule::operator_add, "+")?.next().unwrap();
    let sub = Grammar::parse(Rule::operator_add, "-")?.next().unwrap();
    let mul = Grammar::parse(Rule::operator_mul, "*")?.next().unwrap();
    let div = Grammar::parse(Rule::operator_mul, "/")?.next().unwrap();
    let unary = Grammar::parse(Rule::operator_unary, "-")?.next().unwrap();

    assert_eq!(add.as_str(), "+");
    assert_eq!(sub.as_str(), "-");
    assert_eq!(mul.as_str(), "*");
    assert_eq!(div.as_str(), "/");
    assert_eq!(unary.as_str(), "-");

    Ok(())
}
