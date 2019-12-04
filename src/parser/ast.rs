use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Root {
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    UnaryPrefixOperation(PrefixOperator, Box<Expression>),
    UnarySuffixOperation(Box<Expression>, SuffixOperator),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::BinaryOperation(ref left, op, ref right) => {
                write!(f, "{} {} {}", left, op, right)
            }
            Expression::UnaryPrefixOperation(ref op, ref exp) => write!(f, "{}{}", op, exp),
            Expression::UnarySuffixOperation(ref exp, ref op) => write!(f, "{}{}", exp, op),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiplication,
    Divide,
    Power,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Multiplication => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Power => write!(f, "^"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrefixOperator {
    PlusSign,
    MinusSign,
}

impl fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrefixOperator::PlusSign => write!(f, "+"),
            PrefixOperator::MinusSign => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SuffixOperator {
    Factorial,
}

impl fmt::Display for SuffixOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuffixOperator::Factorial => write!(f, "!"),
        }
    }
}
