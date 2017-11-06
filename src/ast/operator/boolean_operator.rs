use std::cmp::Ordering;

#[derive(Eq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let (e, g, l) = (BooleanOperator::Equal, BooleanOperator::GreaterThan, BooleanOperator::LessThan);
        assert_eq!(e, e); assert_eq!(e, g); assert_eq!(e, l);
        assert_eq!(g, e); assert_eq!(g, g); assert_eq!(g, l);
        assert_eq!(l, e); assert_eq!(l, g); assert_eq!(l, l);
    }

    #[test]
    fn priority() {
        let (e, g, l) = (BooleanOperator::Equal, BooleanOperator::GreaterThan, BooleanOperator::LessThan);
        assert!(e <= g);  assert!(e >= g);  assert!(!(e < g));  assert!(!(e > g));
        assert!(e <= l);  assert!(e >= l);  assert!(!(e < l));  assert!(!(e > l));
        assert!(g <= e);  assert!(g >= e);  assert!(!(g < e));  assert!(!(g > e));
        assert!(g <= l);  assert!(g >= l);  assert!(!(g < l));  assert!(!(g > l));
        assert!(l <= e);  assert!(l >= e);  assert!(!(l < e));  assert!(!(l > e));
        assert!(l <= g);  assert!(l >= g);  assert!(!(l < g));  assert!(!(l > g));
    }
}

