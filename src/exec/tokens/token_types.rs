use std::fmt;

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum TokenType {
    Number,
    Operator,
    None,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Number => write!(f, "number"),
            TokenType::Operator => write!(f, "operator"),
            TokenType::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TokenTypeChecker {
    pub token_type: TokenType,
    pub ends_with: fn(c: char) -> bool,
    pub starts_with: fn(c: char) -> bool,
}


pub const NUMBER_TOKEN_CHECKER: TokenTypeChecker = TokenTypeChecker {
    starts_with: is_numeric,
    ends_with: is_not_numeric,
    token_type: TokenType::Number,
};

fn is_numeric(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => true,
        _ => false,
    }
}

fn is_not_numeric(c: char) -> bool {
    !is_numeric(c)
}


pub const SPACE_TOKEN_CHECKER: TokenTypeChecker = TokenTypeChecker {
    starts_with: is_space,
    ends_with: is_not_space,
    token_type: TokenType::None,
};

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' => true,
        _ => false,
    }
}

fn is_not_space(c: char) -> bool {
    !is_space(c)
}


pub const OPERATOR_TOKEN_CHECKER: TokenTypeChecker = TokenTypeChecker {
    starts_with: is_operator,
    ends_with: allways_end,
    token_type: TokenType::Operator,
};

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn allways_end(_c: char) -> bool {
    true
}
