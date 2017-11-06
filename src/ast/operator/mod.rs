mod binary_operator;
mod boolean_operator;

pub use self::binary_operator::BinaryOperator;
pub use self::boolean_operator::BooleanOperator;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operator {
    Boolean(BooleanOperator),
    Binary(BinaryOperator),
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::BinaryOperator::{Add, Subtract, Multiply};
    use super::BooleanOperator::{Equal, GreaterThan, LessThan};

    #[test]
    fn equality() {
        let (a, s, m) = (Operator::Binary(Add), Operator::Binary(Subtract), Operator::Binary(Multiply));
        let (e, g, l) = (Operator::Boolean(Equal), Operator::Boolean(GreaterThan), Operator::Boolean(LessThan));
        assert_eq!(a, a); assert_eq!(a, s); assert_ne!(a, m);
        assert_eq!(s, a); assert_eq!(s, s); assert_ne!(s, m);
        assert_ne!(m, a); assert_ne!(m, s); assert_eq!(m, m);

        assert_eq!(e, e); assert_eq!(e, g); assert_eq!(e, l);
        assert_eq!(g, e); assert_eq!(g, g); assert_eq!(g, l);
        assert_eq!(l, e); assert_eq!(l, g); assert_eq!(l, l);

        assert_ne!(a, e); assert_ne!(a, g); assert_ne!(a, l);
        assert_ne!(s, e); assert_ne!(s, g); assert_ne!(s, l);
        assert_ne!(m, e); assert_ne!(m, g); assert_ne!(m, l);
    }

    #[test]
    fn priority() {
        let (a, s, m) = (Operator::Binary(Add), Operator::Binary(Subtract), Operator::Binary(Multiply));
        let (e, g, l) = (Operator::Boolean(Equal), Operator::Boolean(GreaterThan), Operator::Boolean(LessThan));
        assert!(a <= s);    assert!(a >= s);    assert!(!(a < s)); assert!(!(a > s));
        assert!(a <= m);    assert!(!(a >= m)); assert!(a < m);    assert!(!(a > m));
        assert!(s <= a);    assert!(s >= a);    assert!(!(s < a)); assert!(!(s > a));
        assert!(s <= m);    assert!(!(s >= m)); assert!(s < m);    assert!(!(s > m));
        assert!(!(m <= a)); assert!(m >= a);    assert!(!(m < a)); assert!(m > a);
        assert!(!(m <= s)); assert!(m >= s);    assert!(!(m < s)); assert!(m > s);

        assert!(e <= g);  assert!(e >= g);  assert!(!(e < g));  assert!(!(e > g));
        assert!(e <= l);  assert!(e >= l);  assert!(!(e < l));  assert!(!(e > l));
        assert!(g <= e);  assert!(g >= e);  assert!(!(g < e));  assert!(!(g > e));
        assert!(g <= l);  assert!(g >= l);  assert!(!(g < l));  assert!(!(g > l));
        assert!(l <= e);  assert!(l >= e);  assert!(!(l < e));  assert!(!(l > e));
        assert!(l <= g);  assert!(l >= g);  assert!(!(l < g));  assert!(!(l > g));

        assert!(m >= e); assert!(m > e); assert!(!(m <= e)); assert!(!(m < e));
        assert!(a >= e); assert!(a > e); assert!(!(a <= e)); assert!(!(a < e));
        assert!(m >= a); assert!(m > a); assert!(!(m <= a)); assert!(!(m < a));
    }
}

