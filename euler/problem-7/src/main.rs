extern crate euler;

use euler::primes;

fn main() {

    let mut s = primes::Sieve::new(100000);
    let nth = s.nth(10000);

    match nth {
        Some(m) => println!("{}", m),
        None => ()
    }
}
