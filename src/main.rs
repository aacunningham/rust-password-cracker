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


enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

enum BooleanOperator {
    Equal,
    GreaterThan,
    LessThan,
}

enum Expression<'a> {
    BinaryExp(BinaryExpression<'a>),
    Variable(Variable),
}

enum Operator {
    Binary(BinaryOperator),
    Boolean(BooleanOperator),
}

struct BinaryExpression<'a> {
    l_value: &'a Expression<'a>,
    operator: Operator,
    r_value: &'a Expression<'a>,
}

trait Evaluable {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str>;
}

impl<'a> Evaluable for Expression<'a> {
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

impl<'a> Evaluable for BinaryExpression<'a> {
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

fn main() {
    let val_array = [3, 2, 3, 4];

    let x = Variable {
        name: 'b',
    };

    let y = Variable {
        name: 'b',
    };

    let z = BinaryExpression {
        l_value: &Expression::Variable(x),
        operator: Operator::Boolean(BooleanOperator::Equal),
        r_value: &Expression::Variable(y),
    };

    let ret = z.evaluate(&val_array);

    println!("{:?}", ret);
}
