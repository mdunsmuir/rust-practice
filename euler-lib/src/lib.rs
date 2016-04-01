/// Functionality related to prime numbers
pub mod primes {

    extern crate primal;

    pub fn factors(mut x: i64) -> Vec<i64> {
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
}

/// Functionality involving the digits of numbers
pub mod digits {

    /// Determine whether a number's digits are palindromic
    pub fn is_palindromic(x: i32) -> bool {
        let digs = digits(x);

        for i in 0..(digs.len() / 2) {
            if digs[i] != digs[digs.len() - i - 1] {
                return false;
            }
        }

        return true;
    }

    /// Returns the digits of the given integer in reverse order.
    /// Usually the ordering doesn't matter for Euler problems.
    pub fn digits(mut x: i32) -> Vec<u8> {
        let base = 10;
        let mut digs = vec![];

        while x > 0 {
            digs.push((x % base) as u8);
            x = x / base;
        }

        return digs
    }
}
