pub mod evaluable;
pub mod variable;
pub mod operator;
pub mod expression;

use self::evaluable::Value;
use self::variable::Variable;
use self::expression::{Expression, BinaryExpression};
use self::operator::Operator;
use self::operator::BinaryOperator::{Add, Subtract, Multiply};
use self::operator::BooleanOperator::{Equal, GreaterThan, LessThan};

pub fn convert_string_to_ast(input: &str) -> Result<Box<Expression>, &'static str> {
    let mut exp_vec: Vec<Box<Expression>> = Vec::new();
    let mut op_vec: Vec<Operator> = Vec::new();
    let mut has_failed = false;
    for c in input.chars() {
        match c {
            v @ 'a'...'d' => exp_vec.push(Box::new(Expression::Variable(Variable { name: v }))),
            '*' => op_vec.push(Operator::Binary(Multiply)),
            v @ '0'...'9' => {
                let number: i8 = match v {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => 0,
                };
                exp_vec.push(Box::new(Expression::Value(Value::Numerical(number))));
            },
            o @ '*'...'>' => {
                let op = match o {
                    '*' => Operator::Binary(Multiply),
                    '-' => Operator::Binary(Subtract),
                    '+' => Operator::Binary(Add),
                    '=' => Operator::Boolean(Equal),
                    '<' => Operator::Boolean(LessThan),
                    '>' => Operator::Boolean(GreaterThan),
                    _ => continue,
                };
                has_failed = !handle_ops(&mut exp_vec, &mut op_vec, op);
            },
            _ => { continue; },
        }

        if has_failed {
            return Err("Malformed input string");
        }
    }

    while let Some(op) = op_vec.pop() {
        let r_value = exp_vec.pop();
        let l_value = exp_vec.pop();
        match (l_value, r_value) {
            (Some(l), Some(r)) => {
                exp_vec.push(Box::new(Expression::BinaryExp(BinaryExpression { l_value: l, operator: op, r_value: r })));
            },
            _ => return Err("Malformed input string"),
        }
    }

    if let Some(final_exp) = exp_vec.pop() {
        Ok(final_exp)
    } else {
        Err("Something went wrong I guess")
    }
}

fn handle_ops(exp_vec: &mut Vec<Box<Expression>>, op_vec: &mut Vec<Operator>, new_op: Operator) -> bool {
    while let Some(x) = op_vec.pop() {
        if new_op <= x {
            let r_value = exp_vec.pop();
            let l_value = exp_vec.pop();
            match (l_value, r_value) {
                (Some(l), Some(r)) => {
                    exp_vec.push(Box::new(Expression::BinaryExp(BinaryExpression { l_value: l, operator: x, r_value: r })));
                },
                _ => return false,
            }
        } else {
            op_vec.push(x);
            break;
        }
    }
    op_vec.push(new_op);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_equal_ops() {
        let mut exp_vec: Vec<Box<Expression>> = Vec::new();
        let mut op_vec: Vec<Operator> = Vec::new();

        let l_var = Expression::Variable(Variable {name: 'a'});
        let r_var = Expression::Variable(Variable {name: 'b'});

        exp_vec.push(Box::new(l_var.clone()));
        exp_vec.push(Box::new(r_var.clone()));
        op_vec.push(Operator::Binary(Add));

        let expected_result = Some(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(l_var.clone()), operator: Operator::Binary(Add), r_value: Box::new(r_var.clone())})));

        let _ = handle_ops(&mut exp_vec, &mut op_vec, Operator::Binary(Subtract));
        assert_eq!(exp_vec.pop(), expected_result);
        assert_eq!(op_vec.pop(), Some(Operator::Binary(Subtract)));
    }

    #[test]
    fn handle_greater_ops() {
        let mut exp_vec: Vec<Box<Expression>> = Vec::new();
        let mut op_vec: Vec<Operator> = Vec::new();

        let l_var = Expression::Variable(Variable {name: 'a'});
        let r_var = Expression::Variable(Variable {name: 'b'});

        exp_vec.push(Box::new(l_var.clone()));
        exp_vec.push(Box::new(r_var.clone()));
        op_vec.push(Operator::Binary(Add));
        
        let expected_result = Some(Box::new(r_var));
        let _ = handle_ops(&mut exp_vec, &mut op_vec, Operator::Binary(Multiply));
        assert_eq!(exp_vec.pop(), expected_result);
        assert_eq!(op_vec.pop(), Some(Operator::Binary(Multiply)));
    }

    #[test]
    fn handle_lesser_ops() {
        let mut exp_vec: Vec<Box<Expression>> = Vec::new();
        let mut op_vec: Vec<Operator> = Vec::new();

        let l_var = Expression::Variable(Variable {name: 'a'});
        let r_var = Expression::Variable(Variable {name: 'b'});

        exp_vec.push(Box::new(l_var.clone()));
        exp_vec.push(Box::new(r_var.clone()));
        op_vec.push(Operator::Binary(Multiply));
        
        let expected_result = Some(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(l_var.clone()), operator: Operator::Binary(Multiply), r_value: Box::new(r_var.clone())})));
        let _ = handle_ops(&mut exp_vec, &mut op_vec, Operator::Boolean(Equal));
        assert_eq!(exp_vec.pop(), expected_result);
        assert_eq!(op_vec.pop(), Some(Operator::Boolean(Equal)));
    }

    #[test]
    fn pop_multiple_ops() {
        let mut exp_vec: Vec<Box<Expression>> = Vec::new();
        let mut op_vec: Vec<Operator> = Vec::new();

        let var1 = Expression::Variable(Variable {name: 'a'});
        let var2 = Expression::Variable(Variable {name: 'b'});
        let var3 = Expression::Variable(Variable {name: 'c'});

        exp_vec.push(Box::new(var1.clone()));
        exp_vec.push(Box::new(var2.clone()));
        exp_vec.push(Box::new(var3.clone()));
        op_vec.push(Operator::Binary(Add));
        op_vec.push(Operator::Binary(Multiply));
        
        let expected_result = Some(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(var1.clone()), operator: Operator::Binary(Add), r_value: Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(var2.clone()), operator: Operator::Binary(Multiply), r_value: Box::new(var3.clone())}))})));
        let _ = handle_ops(&mut exp_vec, &mut op_vec, Operator::Boolean(Equal));
        assert_eq!(exp_vec.pop(), expected_result);
        assert_eq!(op_vec.pop(), Some(Operator::Boolean(Equal)));
    }

    #[test]
    fn convert_boolean() {
        let input = "a = b";

        let var1 = Expression::Variable(Variable {name: 'a'});
        let var2 = Expression::Variable(Variable {name: 'b'});
        let expected_result = Ok(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(var1), operator: Operator::Boolean(Equal), r_value: Box::new(var2)})));
        assert_eq!(convert_string_to_ast(input), expected_result);
    }

    #[test]
    fn convert_binary() {
        let input = "a + b";

        let var1 = Expression::Variable(Variable {name: 'a'});
        let var2 = Expression::Variable(Variable {name: 'b'});
        let expected_result = Ok(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(var1), operator: Operator::Binary(Add), r_value: Box::new(var2)})));
        assert_eq!(convert_string_to_ast(input), expected_result);
    }

    #[test]
    fn convert_compound() {
        let input = "a * b = c";

        let var1 = Expression::Variable(Variable {name: 'a'});
        let var2 = Expression::Variable(Variable {name: 'b'});
        let var3 = Expression::Variable(Variable {name: 'c'});
        let expected_result = Ok(Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(Expression::BinaryExp(BinaryExpression {l_value: Box::new(var1), operator: Operator::Binary(Multiply), r_value: Box::new(var2)})), operator: Operator::Boolean(Equal), r_value: Box::new(var3)})));
        assert_eq!(convert_string_to_ast(input), expected_result);
    }
}

