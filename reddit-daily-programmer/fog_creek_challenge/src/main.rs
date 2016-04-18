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
        pub len: usize,
        pub start: usize,
        pub end: usize,
        pub chr: u8,
    }

    impl ClosedPair {

        pub fn len(&self) -> usize {
            self.len
            //self.end - self.start + 1
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

        pub fn next_char(mut self, chr: u8, i: usize) -> RangeResult {
            let not_already_found = self.found.insert(chr);

            if not_already_found {
                if chr == self.chr {
                    let closed = ClosedPair {
                        len: self.found.len() + 1,
                        start: self.start,
                        end: i,
                        chr: self.chr
                    };
                    Both(self, closed)

                } else {
                    Open(self)
                }

            } else {
                if chr == self.chr {
                    Closed(ClosedPair {
                        len: self.found.len() + 2,
                        start: self.start,
                        end: i,
                        chr: self.chr
                    })

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

fn find_pairs(string: &Vec<Option<u8>>) -> Option<VecDeque<ClosedPair>> {
    let mut closed = VecDeque::new();
    let mut open: VecDeque<OpenPair> = VecDeque::new();

    for (i, opt_chr) in string.into_iter().enumerate() {
        match *opt_chr {
            None => continue,

            Some(chr) => {
                for _ in 0..open.len() {
                    let pair = open.pop_front().unwrap();
                    match pair.next_char(chr, i) {
                        Open(pair) => open.push_back(pair),
                        Closed(pair) => closed.push_back(pair),
                        Both(opair, cpair) => {
                            open.push_back(opair);
                            closed.push_back(cpair);
                        },
                        _ => (),
                    }
                }

                open.push_back(OpenPair::start(chr, i));
            },
        }
    }

    if closed.len() > 0 {
        Some(closed)
    } else {
        None
    }
}

fn process_string(string: &Vec<u8>) -> Vec<u8> {
    let mut opt_string = Vec::new();
    for chr in string {
        opt_string.push(Some(*chr));
    }

    while let Some(mut pairs) = find_pairs(&opt_string) {
        let mut max = pairs.pop_front().unwrap();

        for pair in pairs {
            if pair.len() > max.len() {
                max = pair;
            }
        }

        opt_string[max.start] = None;
        let to_end = opt_string[max.end].take();
        opt_string.push(to_end);
    }

    opt_string.into_iter()
              .filter(|opt_chr| opt_chr.is_some())
              .map(|opt_chr| opt_chr.unwrap())
              .take_while(|chr| *chr != 95)
              .collect::<Vec<u8>>()
}

fn main() {
    let mut stdin = stdin();
    let mut string = Vec::new();
    stdin.read_to_end(&mut string);

    string = string.into_iter().filter(|chr| *chr != 10).collect();

    let processed = process_string(&string);

    match str::from_utf8(&processed) {
        Ok(string) => println!("{}", string),
        err => println!("{:?}", err),
    }
}
