extern crate euler;

use euler::primes::Sieve;

fn main() {
    let sum: usize =
        Sieve::new(100000)
        .take_while(|&p| p < 2000000)
        .fold(0, |acc, x| acc + x);
    println!("{}", sum);
}
