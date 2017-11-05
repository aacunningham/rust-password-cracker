extern crate passcode_cracker;

use passcode_cracker::{Expression, Variable, Operator, BinaryOperator, BinaryExpression, BooleanOperator, Evaluable};


fn convert_string_to_ast(input: &str) -> Result<Box<Expression>, &'static str> {
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
    loop {

        if let Some(x) = op_vec.pop() {
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
        } else {
            break;
        };
    }
    op_vec.push(new_op);
    true
}

fn main() {
    let val_array = [3, 2, 3, 4];

    let x = Variable {
        name: 'a',
    };

    let y = Variable {
        name: 'b',
    };

    let z = BinaryExpression {
        l_value: Box::new(Expression::Variable(x)),
        operator: Operator::Boolean(BooleanOperator::Equal),
        r_value: Box::new(Expression::Variable(y)),
    };

    let ret = z.evaluate(&val_array);

    println!("{:?}", ret);

    let add = Operator::Binary(BinaryOperator::Add);
    let sub = Operator::Binary(BinaryOperator::Subtract);
    let mult = Operator::Binary(BinaryOperator::Multiply);
    let eq = Operator::Boolean(BooleanOperator::Equal);
    let less = Operator::Boolean(BooleanOperator::LessThan);
    let great = Operator::Boolean(BooleanOperator::GreaterThan);
    println!("eq == eq {:?}", eq == eq);
    println!("eq {:?} less", eq.cmp(&less));
    println!("eq {:?} greater", eq.cmp(&great));
    println!("add {:?} eq", add.cmp(&eq));
    println!("sub {:?} eq", sub.cmp(&eq));
}

