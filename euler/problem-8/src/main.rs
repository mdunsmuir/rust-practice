use std::env::{args};
use std::io::{stdin, Read};
use std::cmp::{max};

fn get_neighborhood_size() -> usize {
    let mut args = args();

    if args.len() != 2 {
        panic!("must give neighborhood size as argument")
    } else {
        match args.nth(1).unwrap().parse() {
            Ok(ns) => ns,
            Err(err) => panic!(err),
        }
    }
}


fn get_digits() -> Vec<u8> {
    let mut digits = Vec::new();

    for result in stdin().bytes() {
        match result {
            Err(err) => panic!(err),
            Ok(byte @ 48 ... 57) => digits.push(byte - 48),
            Ok(10) => (),
            Ok(_) => panic!("a non-digit, non-newline character was detected"),
        }
    }

    digits
}

fn largest_adjacent_digit_product(digits: &Vec<u8>, neighborhood_size: usize) -> u64 {
    let mut largest_product: u64 = 0;

    if neighborhood_size > digits.len() {
        panic!("the number of entered digits was less than the target neighborhood size");
    }

    for i in 0..(digits.len() - neighborhood_size + 1) {
        let neighborhood = &digits[i..(i + neighborhood_size)];
        largest_product =
            max(largest_product,
                neighborhood.iter().fold(1, |sum, val| sum * *val as u64));
    }

    largest_product
}

fn main() {
    let neighborhood_size = get_neighborhood_size();
    let digits = get_digits();
    println!("{}", largest_adjacent_digit_product(&digits, neighborhood_size));
}
