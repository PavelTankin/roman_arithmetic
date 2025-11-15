use anyhow::Result;
use pest::Parser;
use std::env;
use roman_arithmetic::{Grammar, Rule, eval, int_to_roman};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("help") => {
            print_help();
        }

        Some("credits") => {
            print_credits();
        }

        Some("ast") => {
            let input = args.next();
            if let Some(expr) = input {
                roman_arithmetic::test_parse(&expr)?; 
            } else {
                eprintln!("Provide expression for AST.\nUsage: cargo run -- ast \"X + V * II\"");
            }
        }

        Some("parse") => {
            let input = args.next();
            if let Some(expr) = input {
                let pairs = Grammar::parse(Rule::program, &expr)?;
                let pair = pairs.into_iter().next().unwrap();
                let result = eval(pair);
                println!("Result: {} = {}", expr, int_to_roman(result));
            } else {
                eprintln!("Please provide an expression to parse.\nUsage: cargo run -- parse \"X + V * II\"");
            }
        }

        Some("parse_file") => {
            let path = args.next();
            if let Some(path) = path {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path, e))?;
                
                let pairs = Grammar::parse(Rule::program, &content)?;
                let pair = pairs.into_iter().next().unwrap();
                let result = eval(pair);

                println!("Result from file '{}': {}", path, int_to_roman(result));
            } else {
                eprintln!("Please provide a file path.\nUsage: cargo run -- parse_file path/to/file.txt");
            }
        }


        Some(cmd) => {
            eprintln!("Unknown command: {cmd}");
            eprintln!("Use `help` for usage info.");
        }
    }

    Ok(())
}


fn print_help() {
    println!("Roman Arithmetic Parser");
    println!("Usage:");
    println!("  cargo run -- parse \"X + V * II\"");
    println!("  cargo run -- parse_file \"test.txt\"");
    println!("  cargo run -- credits");
    println!("  cargo run -- help");
}

fn print_credits() {
    println!("Roman Arithmetic Parser by Yurii N");
    println!("University project written in Rust + pest");
}

