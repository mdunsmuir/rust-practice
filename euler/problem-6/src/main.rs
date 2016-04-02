fn main() {
    let (mut sum, mut sq_sum): (i64, i64) = (0, 0);

    for n in 1..101 {
        sum += n;
        sq_sum += n.pow(2);
    }

    let sum_sq = sum.pow(2);

    println!("{}", sum_sq - sq_sum);
}
