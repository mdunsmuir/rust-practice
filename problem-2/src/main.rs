fn main() {
    let (mut f1, mut f2, mut sum) = (1, 2, 0);
    
    while f2 <= 4000000 {
        if f2 % 2 == 0 {
            sum += f2;
        }

        let tmp = f1;
        f1 = f2;
        f2 = tmp + f1;
    }

    println!("{}", sum);
}
