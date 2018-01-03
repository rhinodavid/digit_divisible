extern crate digit_divisible;
use digit_divisible::*;
use std::process;

fn main() {
    let num = 211;

    match run(num) {
        Err(e) => {
            eprintln!("Error: \n{}", e);
            process::exit(1);
        }
        Ok(r) => {
            match r {
                true => println!("{:?} is divisible by all its digits", num),
                false => println!("{:?} is not divisible by all its digits", num),
            }
        }
    }
}
