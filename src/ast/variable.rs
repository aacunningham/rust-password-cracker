use ast::evaluable::{Value, Evaluable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    pub name: char,
}

impl Evaluable for Variable {
    fn evaluate(&self, arr: &Vec<u8>) -> Result<Value, &'static str> {
        match self.name {
            c @ 'a'...'z' => Ok(Value::Numerical(arr[(c as usize) - 97])),
            _ => Err("Incorrect variable name"),
        }
    }
}

