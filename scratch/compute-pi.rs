#![feature(libc)]
extern crate libc;

use libc::{rand, RAND_MAX};
use std::usize;

/// Get a "random" floating point number between 0 and 1
/// not sure if inclusive
fn frand() -> f64 {
    let base = unsafe { rand() };
    (base as f64) / (RAND_MAX as f64)
}

/// Distance Formula
fn dist(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

/// get number of samples from cmdline args
fn get_n_samples() -> usize {
    match std::env::args().nth(1) {
        None => panic!("must give a number of samples"),
        Some(ref arg_str) => match usize::from_str_radix(arg_str, 10) {
            Err(_) => panic!("failed to parse input"),
            Ok(n_samples) => n_samples,
        },
    }
}

fn main() {
    let n_samples = get_n_samples();
    let mut n_hit = 0;

    for _ in 0..n_samples {
        let x = frand() - 0.5;
        let y = frand() - 0.5;

        if dist(x, y) <= 0.5 {
            n_hit += 1;
        }
    }

    let pi = (n_hit as f64) / (n_samples as f64) / 0.25;
    println!("{}", pi);
}
