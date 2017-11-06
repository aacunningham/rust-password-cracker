use std::cmp::Ordering;

#[derive(Eq)]
pub enum BooleanOperator {
    Equal,
    GreaterThan,
    LessThan,
}

impl PartialEq for BooleanOperator {
    fn eq(&self, _other: &BooleanOperator) -> bool {
        true
    }
}

impl PartialOrd for BooleanOperator {
    fn partial_cmp(&self, other: &BooleanOperator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BooleanOperator {
    fn cmp(&self, _other: &BooleanOperator) -> Ordering {
        Ordering::Equal
    }
}

