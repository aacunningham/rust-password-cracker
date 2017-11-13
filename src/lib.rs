#![feature(test)]

extern crate test;

mod ast;

use test::Bencher;

use ast::expression::Expression;
use ast::evaluable::{Value, Evaluable};


pub struct RuleList {
    rules: Vec<Box<Expression>>,
}

pub struct Passcode {
    length: usize,
    possible_values: Vec<Vec<u8>>,
    rule_list: RuleList,
}

impl RuleList {
    fn new() -> RuleList {
        RuleList { rules: Vec::new() }
    }

    fn run_rules(&self, combination: &Vec<u8>) -> bool {
        for rule in self.rules.iter() {
            if let Ok(Value::Boolean(res)) = rule.evaluate(combination) {
                if !res { return false; }
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
    pub fn new(length: usize) -> Passcode {
        let total_combinations: usize = 10usize.pow(length as u32);
        let mut combinations: Vec<Vec<u8>> = CombinationGenerator::new(length).collect();

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
        self.solutions_left() > 0
    }

    pub fn solution(&self) -> Result<&Vec<u8>, &'static str> {
        if self.solutions_left() == 1 {
            Ok(&self.possible_values[0])
        } else {
            Err("Nope")
        }
    }
}

pub struct CombinationGenerator {
    length: usize,
    count: usize,
}

impl CombinationGenerator {
    pub fn new(length: usize) -> CombinationGenerator {
        CombinationGenerator { length: length, count: 0 }
    }
}

impl Iterator for CombinationGenerator {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Vec<u8>> {
        if self.count >= 10usize.pow(self.length as u32) {
            return None;
        }
        let mut ret_vec: Vec<u8> = vec![0; self.length];
        let mut count_copy = self.count;
        for i in (0..self.length).rev() {
            ret_vec[i] = (count_copy % 10) as u8;
            count_copy /= 10;
            if count_copy == 0 {
                break;
            }
        }
        self.count += 1;
        Some(ret_vec)
    }
}

#[bench]
fn bench_first_for_loop(b: &mut Bencher) {
    b.iter(|| {
        let total_combinations: usize = 10000;
        let mut combinations: Vec<Vec<u8>> = vec![Vec::with_capacity(4 as usize); total_combinations];
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
    });
}

#[bench]
fn bench_new_generator(b: &mut Bencher) {
    b.iter(|| {
        let mut combinations: Vec<Vec<u8>> = test::black_box(CombinationGenerator::new(4).collect());
    });
}

