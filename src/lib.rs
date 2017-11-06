mod ast;

use ast::operator::{Operator, BinaryOperator, BooleanOperator};
use ast::expression::{Expression, BinaryExpression};
use ast::evaluable::{Value, Evaluable};
use ast::variable::Variable;


pub struct RuleList {
    rules: Vec<Box<Expression>>,
}

pub struct Passcode {
    possible_values: Vec<[i8; 4]>,
    rule_list: RuleList,
}

impl RuleList {
    fn new() -> RuleList {
        RuleList {
            rules: Vec::new(),
        }
    }

    fn run_rules(&self, combination: &[i8; 4]) -> bool {
        for rule in self.rules.iter() {
            if let Ok(Value::Boolean(res)) = rule.evaluate(combination) {
                if !res {
                    return false;
                }
            }
        }

        true
    }

    fn add_rule(&mut self, rule: &str) -> bool {
        if let Ok(ast) = ast::convert_string_to_ast(rule) {
            self.rules.push(ast);
            return true;
        }

        false
    }
}

impl Passcode {
    pub fn new() -> Passcode {
        let mut combinations: Vec<[i8; 4]> = Vec::new();
        for a in 0..10 {
            for b in 0..10 {
                for c in 0..10 {
                    for d in 0..10 {
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

    pub fn eliminate_combinations(&mut self) -> usize {
        let mut bad_combinations = Vec::new();
        for (i, combination) in self.possible_values.iter().enumerate() {
            if !self.rule_list.run_rules(&combination) {
                bad_combinations.push(i);
            }
        }

        for bad_index in bad_combinations.iter().rev() {
            self.possible_values.remove(*bad_index);
        }

        bad_combinations.len()
    }

    pub fn add_rule(&mut self, rule: &str) -> bool {
        self.rule_list.add_rule(rule)
    }

    pub fn print_solutions(&self) {
        for c in self.possible_values.iter() {
            println!("{:?}", c);
        }
    }

    pub fn solution_exists() -> bool {
        true
    }
}

