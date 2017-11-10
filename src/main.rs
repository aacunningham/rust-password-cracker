extern crate passcode_cracker;

use std::io;
use std::io::Write;
use passcode_cracker::Passcode;

fn main() {
    'main: loop {
        let num_of_digits;
        let mut input = String::new();
        'digit: loop {
            println!("Number of digits: ");
            io::stdin().read_line(&mut input).ok();
            input = input.trim().to_owned();
            if input == "q" {
                break 'main;
            }
            match input.parse::<usize>() {
                Ok(value) => {
                    num_of_digits = value;
                    break 'digit;
                },
                Err(_) => {
                    println!("Invalid digit, try again.");
                    input.clear();
                },
            };
        }
        let mut passcode_attempt = Passcode::new(num_of_digits);
        while passcode_attempt.solutions_left() > 1 {
            println!("Solutions left: {}", passcode_attempt.solutions_left());
            println!("Hint: ");
            input.clear();
            io::stdin().read_line(&mut input).ok();
            input = input.trim().to_owned();
            if input == "q" {
                break 'main;
            }
            match passcode_attempt.add_rule(&input) {
                Ok(_) => {
                    passcode_attempt.eliminate_combinations();
                },
                Err(message) => {
                    io::stdout().write(message.as_bytes()).ok();
                    io::stdout().write(b"\n").ok();
                },
            };
        }

        println!("Solution: {:?}", passcode_attempt.solution().unwrap());
    }
}

