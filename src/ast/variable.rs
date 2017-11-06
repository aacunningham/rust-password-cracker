use ast::evaluable::{Value, Evaluable};

pub struct Variable {
    pub name: char,
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

