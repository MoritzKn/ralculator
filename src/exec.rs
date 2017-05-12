use std::fmt;

use super::lexer::{tokenize, Token};
use super::token_types::TokenType;

enum OperationInput {
    StaticValue(f64),
    SubOperation(Box<Operation>, Sign),
    Unset,
}

impl OperationInput {
    fn get(&self) -> f64 {
        match *self {
            OperationInput::StaticValue(f) => f,
            OperationInput::SubOperation(ref ob, sign) => {
                let res = ob.calc();
                if sign == Sign::Minus {
                    res * -1f64
                } else {
                    res
                }
            }
            OperationInput::Unset => panic!("Can't get value of unset operation input"),
        }
    }

    fn add_sign(&mut self, new_sign: Sign) {

        println!("[debug] add sign {} to {}", new_sign, self.get());

        match *self {
            OperationInput::StaticValue(value) => {
                if new_sign == Sign::Minus {
                    *self = OperationInput::StaticValue(value * -1f64);
                }
            }
            OperationInput::SubOperation(_, ref mut sign) => {
                *sign = sign.applay(new_sign);
            }
            OperationInput::Unset => panic!("Can't add sing to unset operation input"),
        }
    }
}

enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unset,
}
impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperationType::Add => write!(f, "+"),
            OperationType::Subtract => write!(f, "-"),
            OperationType::Multiply => write!(f, "*"),
            OperationType::Divide => write!(f, "/"),
            OperationType::Unset => write!(f, "unset"),
        }
    }
}

struct Operation {
    right: OperationInput,
    operator: OperationType,
    left: OperationInput,
}

impl Operation {
    fn calc(&self) -> f64 {
        if let OperationInput::Unset = self.left {
            return 0f64;
        }

        match self.operator {
            OperationType::Add => self.left.get() + self.right.get(),
            OperationType::Subtract => self.left.get() - self.right.get(),
            OperationType::Multiply => self.left.get() * self.right.get(),
            OperationType::Divide => self.left.get() / self.right.get(),
            OperationType::Unset => self.left.get(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Sign {
    Plus,
    Minus,
}

impl Sign {
    fn invert(&self) -> Sign {
        match *self {
            Sign::Minus => Sign::Plus,
            Sign::Plus => Sign::Minus,
        }
    }
    fn applay(&self, sign: Sign) -> Sign {
        match sign {
            Sign::Minus => self.invert(),
            _ => *self,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sign::Minus => write!(f, "minus"),
            Sign::Plus => write!(f, "plus"),
        }
    }
}

fn to_operation(tokens: &[Token]) -> Result<Operation, (&str, usize)> {

    if tokens.is_empty() {
        return Result::Err(("nothing to calculate", 0));
    }

    #[derive(Debug, PartialEq, Eq)]
    enum State {
        NothingDone,
        RightSet,
        OperatorSet,
        LeftSet,
    }

    let mut state = State::NothingDone;

    let mut left = OperationInput::Unset;
    let mut left_num_sign = Sign::Plus;

    let mut operator = OperationType::Unset;

    let mut right = OperationInput::Unset;
    let mut right_num_sign = Sign::Plus;

    println!("[debug] tokens:");

    for (i, t) in tokens.iter().enumerate() {

        println!("[debug] - '{}' of type {}", t.content, t.token_type);

        match t.token_type {
            TokenType::Operator => {
                if state == State::LeftSet {
                    operator = match t.content.as_ref() {
                        "+" => OperationType::Add,
                        "-" => OperationType::Subtract,
                        "*" => OperationType::Multiply,
                        "/" => OperationType::Divide,
                        _ => panic!(format!("{} is not an operator", t.content)),
                    };

                    state = State::OperatorSet;
                } else {
                    let sign_optional = match t.content.as_ref() {
                        "+" => Option::Some(Sign::Plus),
                        "-" => Option::Some(Sign::Minus),
                        _ => Option::None,
                    };

                    if let Some(sign) = sign_optional {
                        match state {
                            State::NothingDone => {
                                left_num_sign = left_num_sign.applay(sign);
                            }
                            State::OperatorSet => {
                                right_num_sign = right_num_sign.applay(sign);
                            }
                            _ => return Result::Err(("unexpected operator", t.pos)),
                        }
                    } else {
                        return Result::Err(("unexpected operator", t.pos));
                    }
                }
            }
            TokenType::Number => {
                let value = t.content.replace(",", ".").parse().unwrap();

                match state {
                    State::NothingDone => {
                        left = OperationInput::StaticValue(value);
                        left.add_sign(left_num_sign);
                        state = State::LeftSet;
                    }
                    State::OperatorSet => {
                        if i == tokens.len() - 1 {
                            right = OperationInput::StaticValue(value);
                            if right_num_sign != Sign::Plus {
                                right.add_sign(right_num_sign);
                            }
                            state = State::RightSet;
                        } else {
                            let remaining = &tokens[i..tokens.len()];

                            match to_operation(remaining) {
                                Err((msg, pos)) => {
                                    return Result::Err((msg, pos));
                                }
                                Ok(operation) => {
                                    right = OperationInput::SubOperation(Box::new(operation),
                                                                         right_num_sign);
                                }
                            }
                            state = State::RightSet;
                            break;
                        }
                    }
                    _ => return Result::Err(("unexpected number", t.pos)),
                }
            }
            TokenType::None => {}
        }
    }

    if state == State::OperatorSet {
        if let Some(last_token) = tokens.last() {
            return Result::Err(("missing ending token", last_token.pos));
        }
    }

    Result::Ok(Operation {
                   left: left,
                   operator: operator,
                   right: right,
               })
}

pub fn exec_expression(expr_str: &str) -> Result<f64, (String, usize)> {
    match tokenize(expr_str) {
        Ok(tokens) => {
            match to_operation(tokens.as_slice()) {
                Ok(operation) => Result::Ok(operation.calc()),
                Err((msg, pos)) => Result::Err((String::from(msg), pos)),
            }
        }
        Err((msg, pos)) => Result::Err((String::from(msg), pos)),
    }
}
