use super::ast::{BinaryOperator, Expression, PrefixOperator, Root, SuffixOperator};
use super::parse;

use self::BinaryOperator::*;
use self::Expression::*;
use self::PrefixOperator::*;
use self::Root::*;
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
        Ok(Expression(UnaryPrefixOperation(
            MinusSign,
            Box::new(Number(42f64))
        ),))
    );
    assert_eq!(
        parse("+42"),
        Ok(Expression(UnaryPrefixOperation(
            PlusSign,
            Box::new(Number(42f64))
        ),))
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
        Ok(Expression(UnaryPrefixOperation(
            PlusSign,
            Box::new(UnaryPrefixOperation(PlusSign, Box::new(Number(42f64))),),
        )))
    );
    assert_eq!(
        parse("+-42"),
        Ok(Expression(UnaryPrefixOperation(
            PlusSign,
            Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(42f64))),),
        )))
    );

    assert_eq!(
        parse("-+-42"),
        Ok(Expression(UnaryPrefixOperation(
            MinusSign,
            Box::new(UnaryPrefixOperation(
                PlusSign,
                Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(42f64))),),
            )),
        )))
    );
}

#[test]
fn test_suffixed_number_expressions() {
    assert_eq!(
        parse("5!"),
        Ok(Expression(UnarySuffixOperation(
            Box::new(Number(5f64)),
            Factorial
        ),))
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
        Ok(Expression(UnarySuffixOperation(
            Box::new(UnarySuffixOperation(Box::new(Number(5f64)), Factorial),),
            Factorial,
        )))
    );
}

#[test]
fn test_nested_unary_expressions() {
    // IDEA: Swap order of operations
    assert_eq!(
        parse("-+5!"),
        Ok(Expression(UnaryPrefixOperation(
            MinusSign,
            Box::new(UnaryPrefixOperation(
                PlusSign,
                Box::new(UnarySuffixOperation(Box::new(Number(5f64)), Factorial),),
            )),
        )))
    );
}

#[test]
fn test_simple_binary_operation_plus() {
    assert_eq!(
        parse("5-10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Minus,
            Box::new(Number(10f64)),
        )))
    );
}

#[test]
fn test_simple_binary_operation_multiplication() {
    assert_eq!(
        parse("5*10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Multiplication,
            Box::new(Number(10f64)),
        )))
    );
}

#[test]
fn test_simple_binary_operation_devide() {
    assert_eq!(
        parse("5/10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Divide,
            Box::new(Number(10f64)),
        )))
    );
}

#[test]
fn test_simple_binary_operation_power() {
    assert_eq!(
        parse("5^10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Power,
            Box::new(Number(10f64)),
        )))
    );
}

#[test]
fn test_simple_binary_operation_white_spaces() {
    assert_eq!(
        parse("5 + 10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Plus,
            Box::new(Number(10f64)),
        )))
    );
    assert_eq!(
        parse("5+10"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(5f64)),
            Plus,
            Box::new(Number(10f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_plus() {
    assert_eq!(
        parse("5 + 10 + 8"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(5f64)),
                Plus,
                Box::new(Number(10f64)),
            )),
            Plus,
            Box::new(Number(8f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_plus_minus() {
    assert_eq!(
        parse("5 - 10 + 8"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(5f64)),
                Minus,
                Box::new(Number(10f64)),
            )),
            Plus,
            Box::new(Number(8f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_divide() {
    assert_eq!(
        parse("5 / 10 / 8"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(5f64)),
                Divide,
                Box::new(Number(10f64)),
            )),
            Divide,
            Box::new(Number(8f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_power() {
    assert_eq!(
        parse("1 ^ 2 ^ 3"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(1f64)),
            Power,
            Box::new(BinaryOperation(
                Box::new(Number(2f64)),
                Power,
                Box::new(Number(3f64)),
            )),
        )))
    );
}

#[test]
fn test_nested_binary_operations_mixed_operator_priorities_plus_divide() {
    assert_eq!(
        parse("1 + 2 / 3"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(1f64)),
            Plus,
            Box::new(BinaryOperation(
                Box::new(Number(2f64)),
                Divide,
                Box::new(Number(3f64)),
            )),
        )))
    );
}

#[test]
fn test_nested_binary_operations_mixed_operator_priorities_multiplication_minus() {
    assert_eq!(
        parse("1 * 2 - 3"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(1f64)),
                Multiplication,
                Box::new(Number(2f64)),
            )),
            Minus,
            Box::new(Number(3f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_mixed_operator_priorities_power_multiplication() {
    assert_eq!(
        parse("1 ^ 2 * 3"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(1f64)),
                Power,
                Box::new(Number(2f64)),
            )),
            Multiplication,
            Box::new(Number(3f64)),
        )))
    );
}

#[test]
fn test_nested_binary_operations_mixed_operator_priorities_complex() {
    assert_eq!(
        parse("2 * 1 ^ 2 + 3 * 9"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(2f64)),
                Multiplication,
                Box::new(BinaryOperation(
                    Box::new(Number(1f64)),
                    Power,
                    Box::new(Number(2f64)),
                )),
            )),
            Plus,
            Box::new(BinaryOperation(
                Box::new(Number(3f64)),
                Multiplication,
                Box::new(Number(9f64)),
            )),
        )))
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

// FIXME: make tests pass
// #[test]
// fn test_binary_and_unary_operations() {
//     assert_eq!(
//         parse("1 * -3"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(Number(1f64)),
//                 Multiplication,
//                 Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))),
//             )
//         ))
//     );
// }
//
// #[test]
// fn test_binary_and_unary_operations_minus_minus() {
//     assert_eq!(
//         parse("1 - -3"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(Number(1f64)),
//                 Minus,
//                 Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))),
//             )
//         ))
//     );
// }
//
// #[test]
// fn test_binary_and_unary_operations_minus_minus_minus() {
//     assert_eq!(
//         parse("1 - - -3"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(Number(1f64)),
//                 Minus,
//                 Box::new(
//                     UnaryPrefixOperation(
//                         MinusSign,
//                         Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))),
//                     )
//                 ),
//             )
//         ))
//     );
// }
//
// #[test]
// fn test_binary_and_unary_operations_with_leading_prefix() {
//     assert_eq!(
//         parse("-1 - 3"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(1f64)))),
//                 Minus,
//                 Box::new(Number(3f64)),
//             )
//         ))
//     );
// }
//
// #[test]
// fn test_binary_and_unary_operations_with_prefixed_parentheses() {
//     assert_eq!(
//         parse("1 * -(3)"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(Number(1f64)),
//                 Multiplication,
//                 Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))),
//             )
//         ))
//     );
// }
//
// #[test]
// fn test_binary_and_unary_operations_factorial_suffix() {
//     assert_eq!(
//         parse("1! * 3"),
//         Ok(Expression(
//             BinaryOperation(
//                 Box::new(UnarySuffixOperation(Box::new(Number(1f64)), Factorial)),
//                 Multiplication,
//                 Box::new(Number(3f64)),
//             )
//         ))
//     );
// }

#[test]
fn test_parentheses_left_in_plus_expression() {
    assert_eq!(
        parse("(1 + 2) + 3"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(Number(1f64)),
                Plus,
                Box::new(Number(2f64)),
            )),
            Plus,
            Box::new(Number(3f64)),
        )))
    );
}

#[test]
fn test_parentheses_right_in_plus_expression() {
    assert_eq!(
        parse("1 + (2 + 3)"),
        Ok(Expression(BinaryOperation(
            Box::new(Number(1f64)),
            Plus,
            Box::new(BinaryOperation(
                Box::new(Number(2f64)),
                Plus,
                Box::new(Number(3f64)),
            )),
        )))
    );
}

#[test]
fn test_multipe_parentheses_in_plus_expression() {
    assert_eq!(
        parse("(2 + 3) + 1 + (2 + 3)"),
        Ok(Expression(BinaryOperation(
            Box::new(BinaryOperation(
                Box::new(BinaryOperation(
                    Box::new(Number(2f64)),
                    Plus,
                    Box::new(Number(3f64)),
                )),
                Plus,
                Box::new(Number(1f64)),
            )),
            Plus,
            Box::new(BinaryOperation(
                Box::new(Number(2f64)),
                Plus,
                Box::new(Number(3f64)),
            )),
        )))
    );
}

#[test]
fn test_empty_parentheses() {
    assert!(parse("()").is_err());
}

#[test]
fn test_parentheses_with_number() {
    assert_eq!(parse("(0)"), Ok(Expression(Number(0f64))));
}

#[test]
fn test_nested_parentheses() {
    assert_eq!(parse("((0))"), Ok(Expression(Number(0f64))));
    assert_eq!(parse("(((0)))"), Ok(Expression(Number(0f64))));
}

// FIXME: make tests pass
// #[test]
// fn test_parentheses_with_plus_prefix() {
//     assert_eq!(
//         parse("+(2)"),
//         Ok(Expression(UnaryPrefixOperation(PlusSign, Box::new(Number(3f64)))))
//     );
// }
//
// #[test]
// fn test_parentheses_with_minus_prefix() {
//     assert_eq!(
//         parse("-(2)"),
//         Ok(Expression(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))))
//     );
// }
//
// #[test]
// fn test_parentheses_with_nested_prefix() {
//     assert_eq!(
//         parse("+-(2)"),
//         Ok(
//             Expression(
//                 UnaryPrefixOperation(
//                     PlusSign,
//                     Box::new(UnaryPrefixOperation(MinusSign, Box::new(Number(3f64)))),
//                 )
//             )
//         )
//     );
// }
//
// #[test]
// fn test_parentheses_with_factorial_suffix() {
//     assert_eq!(
//         parse("(2)!"),
//         Ok(Expression(UnarySuffixOperation(Box::new(Number(3f64)), Factorial)))
//     );
// }
//
// #[test]
// fn test_parentheses_with_nested_suffix() {
//     assert_eq!(
//         parse("(2)!!"),
//         Ok(
//             Expression(
//                 UnarySuffixOperation(
//                     Box::new(UnarySuffixOperation(Box::new(Number(3f64)), Factorial)),
//                     Factorial,
//                 )
//             )
//         )
//     );
// }

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
    assert!(parse("^").is_err());
}

#[test]
fn test_invalidly_placed_operators_in_expressions() {
    assert!(parse("1 / * 2").is_err());
    assert!(parse("1 * / 2").is_err());
    assert!(parse("1 ^ / 2").is_err());
    assert!(parse("/ 2").is_err());
    assert!(parse("2 *").is_err());
    assert!(parse("5 -").is_err());
    assert!(parse("5 +").is_err());
    assert!(parse("(5) +").is_err());
    assert!(parse("(5) *").is_err());
    assert!(parse("/ (5)").is_err());
}

#[test]
fn test_error_detection() {
    {
        let err = parse("*5").unwrap_err();
        assert_eq!(err.offset, 0);
        assert_eq!(err.column, 1);
    }
    {
        let err = parse(" *5").unwrap_err();
        assert_eq!(err.offset, 1);
        assert_eq!(err.column, 2);
    }
    {
        let err = parse("5*").unwrap_err();
        assert_eq!(err.offset, 2);
        assert_eq!(err.column, 3);
    }
    {
        let err = parse("5 * * 5").unwrap_err();
        assert_eq!(err.offset, 4);
        assert_eq!(err.column, 5);
    }
}
