pub mod evaluable;
pub mod variable;
pub mod operator;
pub mod expression;

use self::variable::Variable;
use self::expression::{Expression, BinaryExpression};
use self::operator::{Operator, BinaryOperator, BooleanOperator};

pub fn convert_string_to_ast(input: &str) -> Result<Box<Expression>, &'static str> {
    let mut exp_vec: Vec<Box<Expression>> = Vec::new();
    let mut op_vec: Vec<Operator> = Vec::new();
    for c in input.chars() {
        if c.is_whitespace() { continue; }
        match c {
            'a' => exp_vec.push(Box::new(Expression::Variable(Variable { name: 'a' }))),
            'b' => exp_vec.push(Box::new(Expression::Variable(Variable { name: 'b' }))),
            'c' => exp_vec.push(Box::new(Expression::Variable(Variable { name: 'c' }))),
            'd' => exp_vec.push(Box::new(Expression::Variable(Variable { name: 'd' }))),
            '*' => op_vec.push(Operator::Binary(BinaryOperator::Multiply)),
            o @ '-' | o @ '+' => {
                let operator = match o {
                    '-' => BinaryOperator::Subtract,
                    _ => BinaryOperator::Add,
                };
                let _ = handle_low_priority_op(&mut exp_vec, &mut op_vec, Operator::Binary(operator));
            },
            '=' | '<' | '>' => {
                
            },
            _ => { continue; },
        }
    }

    if let Some(final_exp) = exp_vec.pop() {
        Ok(final_exp)
    } else {
        Err("Something went wrong I guess")
    }
}

fn handle_low_priority_op(exp_vec: &mut Vec<Box<Expression>>, op_vec: &mut Vec<Operator>, new_op: Operator) -> bool {
    while let Some(x) = op_vec.pop() {
        match x {
            Operator::Binary(operator) => {
                let r_value = exp_vec.pop();
                let l_value = exp_vec.pop();
                match (l_value, r_value) {
                    (Some(l), Some(r)) => {
                        exp_vec.push(Box::new(Expression::BinaryExp(BinaryExpression { l_value: l, operator: Operator::Binary(operator), r_value: r })));
                    },
                    _ => return false,
                }
            },
            Operator::Boolean(operator) => {
                op_vec.push(Operator::Boolean(operator));
            },
        }
    }
    op_vec.push(new_op);
    true
}

