fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }

    println!("{}", x);
}

/*
fn main() {
    let x = 5;
    foo(&x);
    println!("{}", x);

    let mut y = 10;
    bar(&mut y);
    println!("{}", y);
}

fn foo(x: &i32) {
    println!("{}", x + 5);
}

fn bar(x: &mut i32) {
    *x = *x + 5;
}
*/
