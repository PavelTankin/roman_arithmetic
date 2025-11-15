# Roman Arithmetic Parser

This project is my **Roman numeral arithmetic parser** in Rust using the pest parsing library. It allows users to parse and evaluate arithmetic expressions written with Roman numerals, supporting addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and nested parentheses for grouping.

The parser includes a **command-line interface (CLI)** for interactive use, expression evaluation, AST visualization, and file-based input.



## Parsing Process

The parser uses a formal grammar defined in `grammar.pest` to recognize Roman numerals and arithmetic expressions. The key components of the grammar are:

1. **Roman Numerals**  
   Recognizes valid Roman numbers.

2. **Factors**  
   For a single Roman numeral, a parenthesized expression `( … )`, or an expression with a unary operator (`+` or `-`).

3. **Strong Terms**  
   For multiplication and division of factors. This ensuring that multiplication/division is evaluated before addition/subtraction.

4. **Terms**  
   For addition and subtraction of strong terms. By separating terms from strong terms, the parser correctly handles arithmetic precedence.

5. **Expressions**  
   Top-level expressions may consist of multiple terms combined with addition/subtraction operators.

6. **Operators**  
   Operators are explicitly parsed for use in evaluation:  
   - `operator_add` → `+` / `-`  
   - `operator_mul` → `*` / `/`  
   - `operator_unary` → unary `+` / `-`

7. **Groups**  
   Parentheses `( … )` are parsed as groups to override the default precedence rules.



## Evaluation

Once an expression is successfully parsed into an **Abstract Syntax Tree (AST)**:

1. The tree is traversed recursively.  
2. Leaf nodes (Roman numerals) are converted to integers using a `roman_to_int` function.  
3. Internal nodes corresponding to operators apply the correct arithmetic operations:  
   - Multiplication and division are applied at the `strong_term` level.  
   - Addition and subtraction are applied at the `term` level.  
4. Parentheses and unary operators are correctly evaluated using the recursion over `factor` and `group` nodes.  
5. The final result is converted back to a Roman numeral using `int_to_roman`.



## Command-Line Interface

The parser provides the following CLI commands:

- `parse "EXPR"` → Evaluate a single arithmetic expression.  
- `ast "EXPR"` → Print the AST of an expression for debugging or visualization.  
- `parse_file PATH` → Evaluate multiple expressions from a text file, one expression per line.  
- `credits` → Show project credits.  
- `help` → Show usage instructions.

---

## Usage Example

```bash
cargo run -- parse "X + V * (II - I)"
# Result: X + V * (II - I) = XV

cargo run -- parse_file "test.txt"
# Prints result for test.txt
