extern crate digit_divisible;
use digit_divisible::*;
use std::process;

fn main() {
    let num = 211;

    let result = run(num).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    match result {
        true => println!("{:?} is divisible by all its digits", num),
        false => println!("{:?} is not divisible by all its digits", num),
    }
}
