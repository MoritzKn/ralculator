use super::parser::{parse, ParseError};
use super::parser::ast::{Root, Expression, BinaryOperator, PrefixOperator, SuffixOperator};
use super::text_range::TextRange;

pub struct InputError {
    /// The error message
    pub msg: String,
    /// The columns in which the error occurred
    pub pos: TextRange,
}

impl InputError {
    fn from_parse_error(err: ParseError) -> InputError {
        InputError {
            // TODO: Formt message
            msg: String::from(""),
            pos: TextRange {
                start: err.offset,
                end: err.column,
            },
        }
    }
}

// TODO: add test
pub fn parse_and_execute(input: &str) -> Result<f64, InputError> {
    match parse(input) {
        Ok(expression) => Ok(execute(expression)),
        Err(err) => Err(InputError::from_parse_error(err)),
    }
}

fn execute(input: Root) -> f64 {
    match input {
        Root::Expression(expression) => execute_expression(expression),
    }
}

// TODO: add test
// TODO: handle semantic errors e.g. 1 / 0
fn execute_expression(expression: Expression) -> f64 {
    match expression {
        Expression::Number(n) => n,
        Expression::BinaryOperation(left, op, right) => {
            let left = execute_expression(*left);
            let right = execute_expression(*right);

            match op {
                BinaryOperator::Plus => left + right,
                BinaryOperator::Minus => left - right,
                BinaryOperator::Multiplication => left * right,
                BinaryOperator::Divide => left / right,
                BinaryOperator::Power => left.powf(right),
            }
        }
        Expression::UnaryPrefixOperation(op, right) => {
            let right = execute_expression(*right);

            match op {
                PrefixOperator::PlusSign => right,
                PrefixOperator::MinusSign => -right,
            }
        }
        Expression::UnarySuffixOperation(left, op) => {
            let left = execute_expression(*left);

            match op {
                SuffixOperator::Factorial => factorial(left as u64) as f64,
            }
        }
    }
}

fn factorial(value: u64) -> u64 {
    let mut curr = 1;
    for i in 1..(value + 1) {
        curr *= i;
    }
    curr
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0u64), 1u64);
    assert_eq!(factorial(1u64), 1u64);
    assert_eq!(factorial(2u64), 2u64);
    assert_eq!(factorial(3u64), 6u64);
    assert_eq!(factorial(4u64), 24u64);
    assert_eq!(factorial(5u64), 120u64);
}
