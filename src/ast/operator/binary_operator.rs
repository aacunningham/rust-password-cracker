use std::cmp::Ordering;

#[derive(Eq, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

impl PartialEq for BinaryOperator {
    fn eq(&self, other: &BinaryOperator) -> bool {
        match self {
            &BinaryOperator::Add | &BinaryOperator::Subtract => {
                if let &BinaryOperator::Multiply = other {
                    false
                } else {
                    true
                }
            },
            _ => {
                if let &BinaryOperator::Multiply = other {
                    true
                } else {
                    false
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let (a, s, m) = (BinaryOperator::Add, BinaryOperator::Subtract, BinaryOperator::Multiply);
        assert_eq!(a, a); assert_eq!(a, s); assert_ne!(a, m);
        assert_eq!(s, a); assert_eq!(s, s); assert_ne!(s, m);
        assert_ne!(m, a); assert_ne!(m, s); assert_eq!(m, m);
    }

    #[test]
    fn priority() {
        let (a, s, m) = (BinaryOperator::Add, BinaryOperator::Subtract, BinaryOperator::Multiply);
        assert!(a <= s);    assert!(a >= s);    assert!(!(a < s)); assert!(!(a > s));
        assert!(a <= m);    assert!(!(a >= m)); assert!(a < m);    assert!(!(a > m));
        assert!(s <= a);    assert!(s >= a);    assert!(!(s < a)); assert!(!(s > a));
        assert!(s <= m);    assert!(!(s >= m)); assert!(s < m);    assert!(!(s > m));
        assert!(!(m <= a)); assert!(m >= a);    assert!(!(m < a)); assert!(m > a);
        assert!(!(m <= s)); assert!(m >= s);    assert!(!(m < s)); assert!(m > s);
    }
}

