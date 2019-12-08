use parser::ast::{BinaryOperator, Expression, PrefixOperator, SuffixOperator};

// TODO: add test
// TODO: handle semantic errors e.g. 1 / 0
pub fn execute_expression(expression: Expression) -> f64 {
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
    for i in 1..=value {
        curr *= i;
    }
    curr
}

#[test]
fn test_factorial_of_0() {
    assert_eq!(factorial(0u64), 1u64);
}

#[test]
fn test_factorial_of_1() {
    assert_eq!(factorial(1u64), 1u64);
}

#[test]
fn test_factorial_of_2() {
    assert_eq!(factorial(2u64), 2u64);
}

#[test]
fn test_factorial_of_3() {
    assert_eq!(factorial(3u64), 6u64);
}

#[test]
fn test_factorial_of_4() {
    assert_eq!(factorial(4u64), 24u64);
}

#[test]
fn test_factorial_of_5() {
    assert_eq!(factorial(5u64), 120u64);
}
