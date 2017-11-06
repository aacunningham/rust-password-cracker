use std::cmp::Ordering;

#[derive(Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

impl PartialEq for BinaryOperator {
    fn eq(&self, _other: &BinaryOperator) -> bool {
        true
    }
}

impl PartialOrd for BinaryOperator {
    fn partial_cmp(&self, other: &BinaryOperator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BinaryOperator {
    fn cmp(&self, other: &BinaryOperator) -> Ordering {
        match (self, other) {
            (&BinaryOperator::Multiply, &BinaryOperator::Multiply) => Ordering::Equal,
            (&BinaryOperator::Multiply, _) => Ordering::Greater,
            (_, &BinaryOperator::Multiply) => Ordering::Less,
            (_, _) => Ordering::Equal,
        }
    }
}

