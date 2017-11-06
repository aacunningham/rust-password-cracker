extern crate passcode_cracker;

use passcode_cracker::{Passcode};

fn main() {
    let mut passcode_attempt = Passcode::new();
    
    passcode_attempt.add_rule("a + b + c = d");
    passcode_attempt.add_rule("3 + c = d");
    passcode_attempt.add_rule("a = 1");
    // passcode_attempt.add_rule("b = 2");
    passcode_attempt.add_rule("c = 3");
    // passcode_attempt.add_rule("d = 6");

    println!("{:?}", passcode_attempt.eliminate_combinations());
    passcode_attempt.print_solutions();
}

