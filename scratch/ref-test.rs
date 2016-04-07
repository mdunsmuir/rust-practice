fn main() {
    let mut a = vec![1, 2, 3];
    let mut b: &Vec<i32> = &a;
    let mut d = vec![1, 2, 3];
    let c: &mut &Vec<i32> = &mut b;

    *c = &mut d;

    for x in *c {
        println!("{}", x);
    }
}

/*
fn main() {
    let mut x = 5;
    let a = 2;
    let mut b = 3;
    let mut y = &mut x;
    let z: &mut &mut i32 = &mut y;

    println!("{}", z);

    *z = &mut b;
    println!("{}", z);
}
*/

/*
fn main() {
    let x = 5;
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1 = v2;
    v1.push(4);

    for num in v1 {
        println!("{}", num);
    }
}
*/

/*
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}
*/

/*
fn main() {

    let mut vec = vec![1, 2, 3];
    println!("printing vector");

    for num in &vec {
        println!("{}", num);
    }

    {
        push(&mut vec, 4)
    }

    vec.push(5);

    println!("printing vector");
    for num in &vec {
        println!("{}", num);
    }
}

fn push(vec: &mut Vec<i32>, num: i32) {
    vec.push(num);

    print_vec(vec);
}

fn print_vec(vec: &Vec<i32>) {
    println!("printing vector");
    for num in vec {
        println!("{}", num);
    }
}
*/

/*
fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }

    println!("{}", x);
}
*/

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
