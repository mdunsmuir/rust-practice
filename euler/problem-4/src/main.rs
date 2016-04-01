extern crate euler;

use euler::digits;

fn main() {
    let mut ans = 0;

    for i in 100..999 {
        for j in i..999 {
            let prod = i * j;

            if digits::is_palindromic(prod) && prod > ans {
                ans = prod;
            }
        }
    }

    println!("{}", ans);
}
