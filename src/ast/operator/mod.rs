mod binary_operator;
mod boolean_operator;

pub use self::binary_operator::BinaryOperator;
pub use self::boolean_operator::BooleanOperator;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Operator {
    Boolean(BooleanOperator),
    Binary(BinaryOperator),
}

