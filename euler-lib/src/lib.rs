/// Functionality related to prime numbers
pub mod primes {

    pub fn factors(mut x: i64) -> Vec<i64> {
        let mut facs: Vec<i64> = vec![];

        'outer: loop {
            let sqrt_x = (x as f64).sqrt();
            let sieve = Sieve::new(100000);
            let candidates = sieve.take_while(|c| (*c as f64) <= sqrt_x);

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

    pub struct Sieve {
        pub primes: Vec<usize>,
        segment_size: usize,
        i_segment: usize,
        n: usize
    }

    impl Sieve {
        pub fn new(segment_size: usize) -> Sieve {
            Sieve {
                primes: Vec::new(),
                segment_size: segment_size,
                i_segment: 0,
                n: 0
            }
        }

        pub fn do_segment(&mut self) {
            let start = self.i_segment * self.segment_size;
            let mut sieve = vec![false; self.segment_size];
            let mut new_primes = Vec::new();

            if self.i_segment == 0 { 
                sieve[0] = true;
                sieve[1] = true;
                sieve[2] = true;
                new_primes.push(2);
            }

            for p in &self.primes {
                if *p == 2 { continue }
                if start + self.segment_size <= p.pow(2) { break }

                let base = *p * (start / *p);
                let mut m = if base < start { base + *p  } else { base };

                while m < start + self.segment_size {
                    sieve[m - start] = true;
                    m += *p;
                }
            }

            let mut n = start;
            if n % 2 == 0 {n += 1 };
            while n < start + self.segment_size {
                if !sieve[n - start] {
                    new_primes.push(n);

                    let mut m = n * 2;
                    while m < start + self.segment_size {
                        sieve[m - start] = true;
                        m += n;
                    }
                }

                n += 2;
            }

            self.primes.append(&mut new_primes);
            self.i_segment += 1;
        }
    }

    impl Iterator for Sieve {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            while self.n >= self.primes.len() {
                self.do_segment();
            }
            self.n += 1;
            Some(self.primes[self.n - 1])
        }
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
