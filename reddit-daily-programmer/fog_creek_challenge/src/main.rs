mod ranges {

    use std::collections::HashSet;

    #[derive(Debug)]
    pub enum RangeResult {
        Open(OpenPair),
        Closed(ClosedPair),
        Both(OpenPair, ClosedPair),
        Aborted,
    }

    use self::RangeResult::*;

    #[derive(Debug)]
    pub struct OpenPair {
        chr: u8,
        start: usize,
        found: HashSet<u8>,
    }

    #[derive(Debug)]
    pub struct ClosedPair {
        pub start: usize,
        pub end: usize,
        pub chr: u8,
    }

    impl ClosedPair {

        pub fn len(&self) -> usize {
            self.end - self.start + 1
        }

    }

    impl OpenPair {

        pub fn start(chr: u8, start: usize) -> OpenPair {
            OpenPair {
                chr: chr,
                start: start,
                found: HashSet::new()
            }
        }

        fn close(&self) -> ClosedPair {
            ClosedPair {
                chr: self.chr,
                start: self.start,
                end: self.start + self.found.len(),
            }
        }

        pub fn next_char(mut self, chr: u8) -> RangeResult {
            let not_already_found = self.found.insert(chr);

            if not_already_found {
                if chr == self.chr {
                    let closed = self.close();
                    Both(self, closed)
                } else {
                    Open(self)
                }

            } else {
                if chr == self.chr {
                    let mut closed = self.close();
                    closed.end += 1;
                    Closed(closed)
                } else {
                    Aborted
                }
            } // end if found
        }

    }

}

use std::io::{stdin};
use std::io::Read;
use std::collections::VecDeque;
use std::str;

use ranges::*;
use ranges::RangeResult::*;

fn find_pairs(string: &Vec<u8>) -> Option<VecDeque<ClosedPair>> {
    let mut closed = VecDeque::new();
    let mut open: VecDeque<OpenPair> = VecDeque::new();

    for (i, chr) in string.into_iter().enumerate() {
        for _ in 0..open.len() {
            let pair = open.pop_front().unwrap();
            match pair.next_char(*chr) {
                Open(pair) => open.push_back(pair),
                Closed(pair) => closed.push_back(pair),
                Both(opair, cpair) => {
                    open.push_back(opair);
                    closed.push_back(cpair);
                },
                _ => (),
            }
        }

        open.push_back(OpenPair::start(*chr, i));
    }

    if closed.len() > 0 {
        Some(closed)
    } else {
        None
    }
}

fn process_string(string: &mut Vec<u8>) {
    while let Some(mut pairs) = find_pairs(string) {
        let mut max = pairs.pop_front().unwrap();

        for pair in pairs {
            if pair.len() > max.len() {
                max = pair;
            }
        }

        string.remove(max.start);
        let last = string.remove(max.end - 1);
        string.push(last);
    }

    *string = string.into_iter()
                    .take_while(|chr| **chr != 95)
                    .map(|chr| *chr)
                    .collect();
}

fn main() {
    let mut stdin = stdin();
    let mut string = Vec::new();

    stdin.read_to_end(&mut string);
    string = string.into_iter().filter(|chr| *chr != 10).collect();
    process_string(&mut string);

    match str::from_utf8(&string) {
        Ok(string) => println!("{}", string),
        err => println!("{:?}", err),
    }
}
