#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

use pest::iterators::Pair;
use pest::Parser;

pub fn roman_to_int(roman: &str) -> i32 {
    let mut sum = 0;
    let mut prev = 0;
    for digit in roman.chars().rev() {
        let val = match digit {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if val < prev {
            sum -= val;
        } else {
            sum += val;
        }
        prev = val;
    }
    return sum;
}

pub fn int_to_roman(num: i32) -> String {
    let mut res = String::new();
    let mut n = num;
    let digits = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
    ];
    for &(val, digit) in &digits {
        while n >= val {
            res.push_str(digit);
            n -= val;
        }
    }
    return res;
}

pub fn print_ast(pair: Pair<Rule>, indent: usize) {
    let rule_name = format!("{:?}", pair.as_rule());
    let span = pair.as_str();

    println!(
        "{}{} = \"{}\"",
        " ".repeat(indent),
        rule_name,
        span.replace("\n", "\\n")
    );

    for child in pair.into_inner() {
        print_ast(child, indent + 2);
    }
}

pub fn test_parse(input: &str) -> anyhow::Result<()> {
    println!("=== Parsing: {} ===", input);

    let pairs = Grammar::parse(Rule::program, input)?;
    let root = pairs.into_iter().next().unwrap();

    print_ast(root, 0);

    Ok(())
}

pub fn eval(pair: Pair<Rule>) -> i32 {
    match pair.as_rule() {
        Rule::program => eval(pair.into_inner().next().unwrap()),

        Rule::expr => eval(pair.into_inner().next().unwrap()),

        Rule::term => {
            let mut inner = pair.into_inner();
            let mut value = eval(inner.next().unwrap()); // перший strong_term

            while let Some(op_pair) = inner.next() {
                let rhs = eval(inner.next().unwrap());
                match op_pair.as_rule() {
                    Rule::operator_add => match op_pair.as_str() {
                        "+" => value += rhs,
                        "-" => value -= rhs,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            value
        }

        Rule::strong_term => {
            let mut inner = pair.into_inner();
            let mut value = eval(inner.next().unwrap()); // перший factor

            while let Some(op_pair) = inner.next() {
                let rhs = eval(inner.next().unwrap());
                match op_pair.as_str() {
                    "*" => value *= rhs,
                    "/" => value /= rhs,
                    _ => unreachable!(),
                }
            }
            value
        }

        Rule::factor => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            match first.as_rule() {
                Rule::operator_unary => {
                    let rhs = eval(inner.next().unwrap());
                    match first.as_str() {
                        "+" => rhs,
                        "-" => -rhs,
                        _ => unreachable!(),
                    }
                }
                Rule::roman => roman_to_int(first.as_str()),
                Rule::group => eval(first.into_inner().next().unwrap()),
                _ => unreachable!(),
            }
        }

        Rule::roman => roman_to_int(pair.as_str()),
        Rule::group => eval(pair.into_inner().next().unwrap()),

        _ => unreachable!(),
    }
}


  
