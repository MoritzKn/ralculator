use super::parse;
use super::ast::{Root, Expression, BinaryOperator, PrefixOperator, SuffixOperator};

use self::Root::*;
use self::Expression::*;
use self::BinaryOperator::*;
use self::PrefixOperator::*;
use self::SuffixOperator::*;

#[test]
fn test_number_expressions() {
    assert_eq!(parse("0"), Ok(Expression(Number(0f64))));
    assert_eq!(parse("9"), Ok(Expression(Number(9f64))));
    assert_eq!(parse("15"), Ok(Expression(Number(15f64))));
}

#[test]
fn test_prefixed_number_expressions() {
    assert_eq!(
        parse("-42"),
        Ok(Expression(UnaryPrefixOperation(MinusSign, Box::new(Number(42f64)))))
    );
    assert_eq!(
        parse("+42"),
        Ok(Expression(UnaryPrefixOperation(PlusSign, Box::new(Number(42f64)))))
    );
}

#[test]
fn test_prefixed_number_expressions_white_spaces() {
    assert!(parse("-42").is_ok());
    assert!(parse("+42").is_ok());
    assert!(parse("- 42").is_ok());
    assert!(parse("- 42").is_ok());
}

#[test]
fn test_nested_prefixed_expressions() {
    assert_eq!(
        parse("++42"),
        Ok(
            Expression(
                UnaryPrefixOperation(
                    PlusSign,
                    Box::new(UnaryPrefixOperation(PlusSign, Box::new(Number(42f64)))),
                )
            )
        )
    );
    assert_eq!(
        parse("+-42"),
        Ok(
            Expression(
                UnaryPrefixOperation(
                    PlusSign,
                    Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(42f64)))),
                )
            )
        )
    );

    assert_eq!(
        parse("-+-42"),
        Ok(
            Expression(
                UnaryPrefixOperation(
                    MinusSign,
                    Box::new(
                        UnaryPrefixOperation(
                            PlusSign,
                            Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(42f64)))),
                        )
                    ),
                )
            )
        )
    );
}

#[test]
fn test_suffixed_number_expressions() {
    assert_eq!(
        parse("5!"),
        Ok(Expression(UnarySuffixOperation(Box::new(Number(5f64)), Factorial)))
    );
}

#[test]
fn test_suffixed_number_expressions_white_spaces() {
    assert!(parse("42!").is_ok());
    assert!(parse("42 !").is_ok());
}

#[test]
fn test_nested_suffixed_expressions() {
    assert_eq!(
        parse("5!!"),
        Ok(
            Expression(
                UnarySuffixOperation(
                    Box::new(UnarySuffixOperation(Box::new(Number(5f64)), Factorial)),
                    Factorial,
                )
            )
        )
    );
}

#[test]
fn test_nested_unary_expressions() {
    // IDEA: Swap order of operations
    assert_eq!(
        parse("-+5!"),
        Ok(
            Expression(
                UnaryPrefixOperation(
                    MinusSign,
                    Box::new(
                        UnaryPrefixOperation(
                            PlusSign,
                            Box::new(UnarySuffixOperation(Box::new(Number(5f64)), Factorial)),
                        )
                    ),
                )
            )
        )
    );
}

#[test]
fn test_simple_binary_operation() {
    assert_eq!(
        parse("5 + 10"),
        Ok(Expression(BinaryOperation(Box::new(Number(5f64)), Plus, Box::new(Number(10f64)))))
    );
    assert_eq!(
        parse("5+10"),
        Ok(Expression(BinaryOperation(Box::new(Number(5f64)), Plus, Box::new(Number(10f64)))))
    );

    assert_eq!(
        parse("5-10"),
        Ok(Expression(BinaryOperation(Box::new(Number(5f64)), Minus, Box::new(Number(10f64)))))
    );
    assert_eq!(
        parse("5*10"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(Number(5f64)),
                    Multiplication,
                    Box::new(Number(10f64)),
                )
            )
        )
    );
    assert_eq!(
        parse("5/10"),
        Ok(Expression(BinaryOperation(Box::new(Number(5f64)), Divide, Box::new(Number(10f64)))))
    );

    assert_eq!(
        parse("5^10"),
        Ok(Expression(BinaryOperation(Box::new(Number(5f64)), Power, Box::new(Number(10f64)))))
    );
}

#[test]
fn test_nested_binary_operations_plus() {
    assert_eq!(
        parse("5 + 10 + 8"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(Box::new(Number(5f64)), Plus, Box::new(Number(10f64))),
                    ),
                    Plus,
                    Box::new(Number(8f64)),
                )
            )
        )
    );
}

#[test]
fn test_nested_binary_operations_plus_minus() {
    assert_eq!(
        parse("5 - 10 + 8"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(Box::new(Number(5f64)), Minus, Box::new(Number(10f64))),
                    ),
                    Plus,
                    Box::new(Number(8f64)),
                )
            )
        )
    );

}

#[test]
fn test_nested_binary_operations_plus_divide() {
    assert_eq!(
        parse("5 / 10 / 8"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(Box::new(Number(5f64)), Divide, Box::new(Number(10f64))),
                    ),
                    Divide,
                    Box::new(Number(8f64)),
                )
            )
        )
    );
}

#[test]
fn test_nested_binary_operations_power() {
    assert_eq!(
        parse("1 ^ 2 ^ 3"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(Number(1f64)),
                    Power,
                    Box::new(
                        BinaryOperation(Box::new(Number(2f64)), Power, Box::new(Number(3f64))),
                    ),
                )
            )
        )
    );
}

#[test]
fn test_nested_binary_operations_mixed_operators_priorities() {
    assert_eq!(
        parse("1 + 2 / 3"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(Number(1f64)),
                    Plus,
                    Box::new(
                        BinaryOperation(Box::new(Number(2f64)), Divide, Box::new(Number(3f64))),
                    ),
                )
            )
        )
    );

    assert_eq!(
        parse("1 * 2 - 3"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(
                            Box::new(Number(1f64)),
                            Multiplication,
                            Box::new(Number(2f64)),
                        )
                    ),
                    Minus,
                    Box::new(Number(3f64)),
                )
            )
        )
    );

    assert_eq!(
        parse("1 ^ 2 * 3"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(Box::new(Number(1f64)), Power, Box::new(Number(2f64))),
                    ),
                    Multiplication,
                    Box::new(Number(3f64)),
                )
            )
        )
    );

    assert_eq!(
        parse("2 * 1 ^ 2 + 3 * 9"),
        Ok(
            Expression(
                BinaryOperation(
                    Box::new(
                        BinaryOperation(
                            Box::new(Number(2f64)),
                            Multiplication,
                            Box::new(
                                BinaryOperation(
                                    Box::new(Number(1f64)),
                                    Power,
                                    Box::new(Number(2f64)),
                                )
                            ),
                        )
                    ),
                    Plus,
                    Box::new(
                        BinaryOperation(
                            Box::new(Number(3f64)),
                            Multiplication,
                            Box::new(Number(9f64)),
                        )
                    ),
                )
            )
        )
    );
}

#[test]
fn test_nested_mixed_operations_white_spaces() {
    assert!(parse("5 + 5 + 5").is_ok());
    assert!(parse("5+5+5").is_ok());
    assert!(parse("5+5*5").is_ok());
    assert!(parse("-5+5*5").is_ok());
    assert!(parse("- 5 + 5 * 5").is_ok());
}


#[test]
fn test_binary_and_unary_operations() {
    // FIXME: make test pass
    // assert_eq!(
    //     parse("1 * -3"),
    //     Ok(
    //         BinaryOperation(
    //             Box::new(Number(1f64)),
    //             Multiplication,
    //             Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(1f64)))),
    //         )
    //     )
    // );

    // TODO: add more tests on this
}

#[test]
fn test_simple_invalid_expression() {
    assert!(parse("foo").is_err());
}

#[test]
fn test_invalidly_placed_operators() {
    assert!(parse("+").is_err());
    assert!(parse("-").is_err());
    assert!(parse("*").is_err());
    assert!(parse("+").is_err());
    assert!(parse("* 1").is_err());
    assert!(parse("/ 1").is_err());
}
