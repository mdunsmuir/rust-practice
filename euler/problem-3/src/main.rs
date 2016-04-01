extern crate euler;

use std::env;
use euler::primes;

fn main() {
    match env::args().nth(1) {
        Some(input) => match input.parse() {
            Ok(parsed_input) => for fac in primes::factors(parsed_input) {
                println!("{}", fac);
            },
            Err(e) => println!("{}", e),
        },
        None => println!("Must supply an input integer"),
    }
}
