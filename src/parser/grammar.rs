#![allow(unknown_lints, never_loop)]

use super::ast::*;
use std::option::Option::{None, Some};

/// `IncompleteExpression` is used temporary to avoid left recursion
#[derive(Debug, PartialEq)]
enum IncompleteExpression {
    UnarySuffixOperation(SuffixOperator, Option<Box<IncompleteExpression>>),
}

fn finish_expression(left: Expression, rest: Option<Box<IncompleteExpression>>) -> Expression {
    match rest {
        None => left,
        Some(rest) => {
            let rest = *rest;
            match rest {
                IncompleteExpression::UnarySuffixOperation(op, new_rest) => finish_expression(
                    Expression::UnarySuffixOperation(Box::new(left), op),
                    new_rest,
                ),
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
