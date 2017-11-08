#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum Value {
    Numerical(i8),
    Boolean(bool),
}

pub trait Evaluable {
    fn evaluate(&self, arr: &Vec<i8>) -> Result<Value, &'static str>;
}

impl Evaluable for Value {
    fn evaluate(&self, _arr: &Vec<i8>) -> Result<Value, &'static str> {
        Ok(self.clone())
    }
}
