extern crate passcode_cracker;

use std::io;
use std::io::Write;
use passcode_cracker::{Passcode};

fn main() {
    'hi: loop {
        println!("Number of digits: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input == "q" {
            break;
        }
        let num_of_digits = input.trim().parse::<u32>().unwrap();
        let mut passcode_attempt = Passcode::new(num_of_digits);
        while passcode_attempt.solutions_left() > 1 {
            println!("Hint: ");
            input.clear();
            io::stdin().read_line(&mut input);
            if input == "q" {
                break 'hi;
            }
            println!("{:?}", input);
            passcode_attempt.add_rule(&input);
            passcode_attempt.eliminate_combinations();
            println!("Solutions left: {}", passcode_attempt.solutions_left());
        }

        println!("Solution: {:?}", passcode_attempt.solution().unwrap());
    }
}

