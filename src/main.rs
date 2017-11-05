use std::cmp::Ordering;

struct Variable {
    name: char,
}

#[derive(Debug)]
enum Value {
    Numerical(i8),
    Boolean(bool),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if let (&Value::Numerical(l_val), &Value::Numerical(r_val)) = (self, other) {
            Some(l_val.cmp(&r_val))
        } else {
            None
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        if let (&Value::Numerical(l_val), &Value::Numerical(r_val)) = (self, other) {
            l_val == r_val
        } else {
            false
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

#[derive(Eq)]
enum BooleanOperator {
    Equal,
    GreaterThan,
    LessThan,
}

impl PartialEq for BooleanOperator {
    fn eq(&self, other: &BooleanOperator) -> bool {
        true
    }
}

impl PartialOrd for BooleanOperator {
    fn partial_cmp(&self, other: &BooleanOperator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BooleanOperator {
    fn cmp(&self, other: &BooleanOperator) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Operator {
    Boolean(BooleanOperator),
    Binary(BinaryOperator),
}

enum Expression {
    BinaryExp(BinaryExpression),
    Variable(Variable),
}

struct BinaryExpression {
    l_value: Box<Expression>,
    operator: Operator,
    r_value: Box<Expression>,
}

trait Evaluable {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str>;
}

impl Evaluable for Expression {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str> {
        match self {
            &Expression::Variable(ref var) => Ok(var.evaluate(&arr)?),
            &Expression::BinaryExp(ref bin_exp) => Ok(bin_exp.evaluate(&arr)?),
        }
    }
}

impl Evaluable for Variable {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str> {
        match self.name {
            'a' => Ok(Value::Numerical(arr[0])),
            'b' => Ok(Value::Numerical(arr[1])),
            'c' => Ok(Value::Numerical(arr[2])),
            'd' => Ok(Value::Numerical(arr[3])),
            _ => Err("Incorrect variable name"),
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

