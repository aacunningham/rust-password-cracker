use ast::operator::{Operator, BinaryOperator, BooleanOperator};
use ast::evaluable::{Value, Evaluable};
use ast::variable::Variable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    BinaryExp(BinaryExpression),
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpression {
    pub l_value: Box<Expression>,
    pub operator: Operator,
    pub r_value: Box<Expression>,
}

impl Evaluable for Expression {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str> {
        match self {
            &Expression::Variable(ref var) => Ok(var.evaluate(&arr)?),
            &Expression::BinaryExp(ref bin_exp) => Ok(bin_exp.evaluate(&arr)?),
        }
    }
}

impl Evaluable for BinaryExpression {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str> {
        if let Operator::Binary(ref op) = self.operator {
            let (l, r) = match (self.l_value.evaluate(&arr)?, self.r_value.evaluate(&arr)?) {
                (Value::Numerical(l_val), Value::Numerical(r_val)) => (Ok(l_val), Ok(r_val)),
                _ => (Err("Not a number"), Err("Not a number")),
            };

            match (l, op, r) {
                (Ok(l_val), &BinaryOperator::Multiply, Ok(r_val)) => Ok(Value::Numerical(l_val * r_val)),
                (Ok(l_val), &BinaryOperator::Add, Ok(r_val)) => Ok(Value::Numerical(l_val + r_val)),
                (Ok(l_val), &BinaryOperator::Subtract, Ok(r_val)) => Ok(Value::Numerical(l_val - r_val)),
                _ => Err("Not a number"),
            }
        } else if let Operator::Boolean(ref op) = self.operator {
            let (l, r) = match (self.l_value.evaluate(&arr)?, self.r_value.evaluate(&arr)?) {
                (Value::Numerical(l_val), Value::Numerical(r_val)) => (Ok(l_val), Ok(r_val)),
                _ => (Err("Not a bool"), Err("Not a bool")),
            };

            match (l, op, r) {
                (Ok(l_val), &BooleanOperator::Equal, Ok(r_val)) => Ok(Value::Boolean(l_val == r_val)),
                (Ok(l_val), &BooleanOperator::GreaterThan, Ok(r_val)) => Ok(Value::Boolean(l_val > r_val)),
                (Ok(l_val), &BooleanOperator::LessThan, Ok(r_val)) => Ok(Value::Boolean(l_val < r_val)),
                _ => Err("Not a bool"),
            }
        } else {
            Err("Something went wrong...")
        }
    }
}

