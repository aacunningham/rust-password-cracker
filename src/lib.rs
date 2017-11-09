mod ast;

use ast::expression::Expression;
use ast::evaluable::{Value, Evaluable};


pub struct RuleList {
    rules: Vec<Box<Expression>>,
}

pub struct Passcode {
    length: u32,
    possible_values: Vec<Vec<i8>>,
    rule_list: RuleList,
}

impl RuleList {
    fn new() -> RuleList {
        RuleList {
            rules: Vec::new(),
        }
    }

    fn run_rules(&self, combination: &Vec<i8>) -> bool {
        for rule in self.rules.iter() {
            if let Ok(Value::Boolean(res)) = rule.evaluate(combination) {
                if !res {
                    return false;
                }
            }
        }

        true
    }

    fn add_rule(&mut self, rule: &str, length: u8) -> Result<(), String> {
        let ast = ast::convert_string_to_ast(rule, length)?;
        self.rules.push(ast);
        Ok(())
    }
}

impl Passcode {
    pub fn new(length: u32) -> Passcode {
        let total_combinations: usize = 10usize.pow(length);
        let mut combinations: Vec<Vec<i8>> = vec![Vec::with_capacity(length as usize); total_combinations];
        let mut chunksize = 1;
        while chunksize < total_combinations {
            let mut current_val = 0;
            for combination_chunk in combinations.chunks_mut(chunksize) {
                for combination in combination_chunk.iter_mut() {
                    combination.push(current_val);
                }
                current_val += 1;
                current_val = current_val % 10;
            }
            chunksize = chunksize * 10;
        }

        Passcode {
            length: length,
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

    pub fn add_rule(&mut self, rule: &str) -> Result<(), String> {
        self.rule_list.add_rule(rule, self.length as u8)
    }

    pub fn print_solutions(&self) {
        for c in self.possible_values.iter() {
            println!("{:?}", c);
        }
    }

    pub fn solutions_left(&self) -> usize {
        self.possible_values.len()
    }

    pub fn solution_exists(&self) -> bool {
        self.possible_values.len() > 0
    }

    pub fn solution(&self) -> Result<&Vec<i8>, &'static str> {
        if self.solutions_left() == 1 {
            Ok(&self.possible_values[0])
        } else {
            Err("Nope")
        }
    }
}

