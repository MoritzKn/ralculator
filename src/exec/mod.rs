mod expression;

use self::expression::execute_expression;
use input_error::InputError;
use parser::ast::Root;
use parser::parse;

pub fn parse_and_execute(input: &str) -> Result<f64, InputError> {
    match parse(input) {
        Ok(expression) => Ok(execute(expression)),
        Err(err) => Err(InputError::from_parser_error(err)),
    }
}

fn execute(input: Root) -> f64 {
    match input {
        Root::Expression(expression) => execute_expression(expression),
    }
}
