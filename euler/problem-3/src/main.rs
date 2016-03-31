extern crate primal;
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(input) => match input.parse() {
            Ok(parsed_input) => for fac in prime_factors(parsed_input) {
                println!("{}", fac);
            },
            Err(e) => println!("{}", e),
        },
        None => println!("Must supply an input integer"),
    }
}

fn prime_factors(mut x: i64) -> Vec<i64> {
    let mut facs: Vec<i64> = vec![];

    'outer: loop {
        let sqrt_x = (x as f64).sqrt();
        let candidates = primal::Primes::all().take_while(|c| (*c as f64) <= sqrt_x);

        for candidate in candidates {
            let icandidate = candidate as i64;
            if x % icandidate == 0 {
                facs.push(icandidate);
                x = x / icandidate;
                continue 'outer;
            }
        }

        facs.push(x);
        break;
    }

    return facs;
}
