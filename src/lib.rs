#![feature(test)]

extern crate test;

mod ast;

use test::Bencher;

use ast::expression::Expression;
use ast::evaluable::{Value, Evaluable};

struct RuleList {
    rules: Vec<Box<Expression>>,
}

impl RuleList {
    fn new() -> RuleList {
        RuleList { rules: Vec::new() }
    }

    fn add_rule(&mut self, rule: &str, length: u8) -> Result<(), String> {
        let ast = ast::convert_string_to_ast(rule, length)?;
        self.rules.push(ast);
        Ok(())
    }

    fn run_rules(&self, combination: &Vec<u8>) -> bool {
        for rule in self.rules.iter() {
            if let Ok(Value::Boolean(res)) = rule.evaluate(combination) {
                if !res { return false; }
            }
        }
        true
    }
}

/// The Passcode struct is used to store all of the data involved in an
/// attempt to solve a given passcode:
/// * the length of the passcode
/// * all of the possible solutions
/// * the rules given by the user
pub struct Passcode {
    length: usize,
    possible_values: Vec<Vec<u8>>,
    rule_list: RuleList,
}

impl Passcode {
    /// 
    /// ```
    /// use passcode_cracker::Passcode;
    /// 
    /// let passcode = Passcode::new(4);
    /// ```
    pub fn new(length: usize) -> Passcode {
        let combinations: Vec<Vec<u8>> = CombinationGenerator::new(length).collect();

        Passcode {
            length: length,
            possible_values: combinations,
            rule_list: RuleList::new(),
        }
    }

    /// Runs through the list of possible solution against the list of
    /// supplied rules. All solutions that are proven invalid are
    /// removed from the struct. Returns the number of bad combinations
    /// removed as a result of running.
    /// 
    /// ```
    /// use passcode_cracker::Passcode;
    /// 
    /// let mut passcode = Passcode::new(2);
    /// assert_eq!(passcode.eliminate_combinations(), 0);
    /// passcode.add_rule("2 + 2 = a");
    /// assert_eq!(passcode.eliminate_combinations(), 90);
    /// ```
    /// 
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

    /// Adds a rule to the passcode, allowing it to remove more
    /// possible solutions as invalid. Accepts a &str formatted
    /// as a mathematical equation with at least one boolean operator.
    /// 
    /// ```
    /// use passcode_cracker::Passcode;
    /// 
    /// let mut passcode = Passcode::new(2);
    /// let res = match passcode.add_rule("2 + 2 = a") {
    ///     Ok(_) => true,
    ///     Err(_) => false,
    /// };
    /// ```
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

struct CombinationGenerator {
    length: usize,
    count: usize,
}

impl CombinationGenerator {
    fn new(length: usize) -> CombinationGenerator {
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
        let _: Vec<Vec<u8>> = test::black_box(CombinationGenerator::new(4).collect());
    });
}

