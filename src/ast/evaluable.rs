#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum Value {
    Numerical(u8),
    Boolean(bool),
}

pub trait Evaluable {
    fn evaluate(&self, arr: &Vec<u8>) -> Result<Value, &'static str>;
}

impl Evaluable for Value {
    fn evaluate(&self, _arr: &Vec<u8>) -> Result<Value, &'static str> {
        Ok(self.clone())
    }
}
