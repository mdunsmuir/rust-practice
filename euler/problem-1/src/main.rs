fn main() {
    let mut sum = 0;

    for x in 3..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    println!("{}", sum);
}
