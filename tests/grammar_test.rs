use anyhow::anyhow;
use roman_arithmetic::*;
use pest::Parser;

#[test]
fn test_unit_test() {
    let test_cases = vec![
        ("X + V * II", "XX"), 
        ("(X + V) * II", "XXX"), 
        ("C - L + X", "LX"),
        ("M / (II * V)", "C"), 
        ("IV * V + X", "XXX"),
        ("L + X * (X - V) / II", "LXXV"),
    ];

    for (input, expected) in test_cases {
        let pairs = Grammar::parse(Rule::program, input)
            .unwrap_or_else(|e| panic!("Failed to parse '{}': {}", input, e));
        let pair = pairs.into_iter().next().unwrap();
        let result_int = eval(pair);
        let result_str = int_to_roman(result_int);
        assert_eq!(
            result_str, expected, 
            "Logic error for input: '{}'. Got {}, expected {}.", 
            input, result_str, expected
        );
    }
}

// === Roman Sub-rules ===

#[test]
fn test_roman_thousands() -> anyhow::Result<()> {
    let inputs = ["M", "MM", "MMM"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman_thousands, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_roman_hundreds() -> anyhow::Result<()> {
    let inputs = ["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman_hundreds, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_roman_tens() -> anyhow::Result<()> {
    let inputs = ["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman_tens, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_roman_units() -> anyhow::Result<()> {
    let inputs = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman_units, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

// === Roman ===

#[test]
fn test_roman() -> anyhow::Result<()> {
    let inputs = ["IX", "LII", "MDCXX", "I", "L", "CX", "CCC", "MMMCMXCIX"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input, "Failed on input: {}", input);
    }
    Ok(())
}

#[test]
fn test_fail_roman() -> anyhow::Result<()> {
    let inputs = ["", "A", "i", "AXA"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::roman, input);
        assert!(result.is_err(), "Have to fail on input: {} but {:?}", input, result);
    }
    Ok(())
}

// === Operator Add ===

#[test]
fn test_operator_add() -> anyhow::Result<()> {
    let inputs = ["+", "-"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_add, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_fail_operator_add() -> anyhow::Result<()> {
    let inputs = ["*", "/", "_", " "];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_add, input);
        assert!(result.is_err(), "Should fail on: {}", input);
    }
    Ok(())
}

// === Operator Mul ===

#[test]
fn test_operator_mul() -> anyhow::Result<()> {
    let inputs = ["*", "/"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_mul, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_fail_operator_mul() -> anyhow::Result<()> {
    let inputs = ["+", "-", "x", "%", " "];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_mul, input);
        assert!(result.is_err(), "Should fail on: {}", input);
    }
    Ok(())
}

// === Operator Unary ===

#[test]
fn test_operator_unary() -> anyhow::Result<()> {
    let inputs = ["+", "-"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_unary, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_fail_operator_unary() -> anyhow::Result<()> {
    let inputs = ["*", "/", " ", "$"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::operator_unary, input);
        assert!(result.is_err(), "Should fail on: {}", input);
    }
    Ok(())
}

// === Group ===

#[test]
fn test_group() -> anyhow::Result<()> {
    let inputs = ["(I)", "(X+V)", "(M*D)", "((I))"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::group, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_fail_group() -> anyhow::Result<()> {
    let inputs = ["I", "(I", "I)", "()", "( )"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::group, input);
        assert!(result.is_err(), "Should fail on: {}", input);
    }
    Ok(())
}

// === Factor ===

#[test]
fn test_factor() -> anyhow::Result<()> {
    let inputs = ["X", "(X)", "-X", "+(V)", "-(-I)"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::factor, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

// === Strong Term ===

#[test]
fn test_strong_term() -> anyhow::Result<()> {
    let inputs = ["X", "X*V", "X/I", "X*V/I", "(X+I)*V"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::strong_term, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_fail_strong_term() -> anyhow::Result<()> {
    let hard_fails = ["*", "/"];
    for &input in &hard_fails {
        let result = Grammar::parse(Rule::strong_term, input);
        assert!(result.is_err(), "Should fail on: {}", input);
    }
    Ok(())
}

// === Term ===

#[test]
fn test_term() -> anyhow::Result<()> {
    let inputs = ["X", "X+V", "X-I", "X*V+I", "X+V*I"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::term, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}

// === Expr ===

#[test]
fn test_expr() -> anyhow::Result<()> {
    let inputs = ["X", "X+V", "X*V"];
    for &input in &inputs {
        let result = Grammar::parse(Rule::expr, input)?
            .next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), input);
    }
    Ok(())
}