mod ast;

pub use ast::operator::{Operator, BinaryOperator, BooleanOperator};
pub use ast::expression::{Expression, BinaryExpression};
pub use ast::evaluable::{Value, Evaluable};
pub use ast::variable::Variable;


pub struct RuleList {
    rules: Vec<Box<Expression>>,
}

pub struct Passcode {
    possible_values: Vec<[i8; 4]>,
    rule_list: RuleList,
}

    fn all_combinations(&self) -> Vec<[i8; 4]> {
        let combinations: Vec<[i8; 4]> = Vec::new();
        for a in self.a {
            for b in self.b {
                for c in self.c {
                    for d in self.d {
                        combinations.push([a, b, c, d]);
                    }
                }
            }
        }

        combinations
    }
}

impl RuleList {
    fn new() -> RuleList {
        RuleList {
            rules: Vec::new(),
        }
    }

    fn add_rule(&self, rule: Box<Expression>) {
        self.rules.push(rule);
    }
}

impl Passcode {
    fn new() -> Passcode {
        let combinations: Vec<[i8; 4]> = Vec::new();
        for a in self.a {
            for b in self.b {
                for c in self.c {
                    for d in self.d {
                        combinations.push([a, b, c, d]);
                    }
                }
            }
        }
        Passcode {
            possible_values: combinations,
            rule_list: RuleList::new(),
        }
    }

    fn eliminate_values(&self) -> u32 {
        for combination in &mut self.combinations {

        }
    }

    fn solution_exists() -> bool {

    }

