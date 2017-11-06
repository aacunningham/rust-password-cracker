#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Numerical(i8),
    Boolean(bool),
}

pub trait Evaluable {
    fn evaluate(&self, arr: &[i8]) -> Result<Value, &'static str>;
}

