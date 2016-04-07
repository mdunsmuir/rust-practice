/*
 * tuple struct tests
 */

struct Foo<T> (T, T);

fn test1() -> usize {
    let a = Foo(1, 2);
    let Foo(b, c) = a;
    c
}

fn test2() -> usize {
    let a = Box::new(Foo(1, 2));
    let Foo(b, c) = *a;
    c
}

fn test3<T>(a: Foo<T>) -> T {
    let Foo(b, c) = a;
    c
}

fn test4<T: Copy>(a: Box<Foo<T>>) -> T {
    let Foo(b, c) = *a;
    c
}


fn test5<T>(a: Box<Foo<T>>) -> T {
    let Foo(b, c) = *a;
    c
}

fn test6<T>(a: &Foo<T>) -> T {
    let Foo(b, c) = *a;
    c
}

/*
 * regular struct tests
 */

struct Bar<T> {
    fst: T,
    snd: T,
}

fn test7<T>(a: Box<Bar<T>>) -> T {
    let a = *a;
    let b = a.fst;
    let c = a.snd;
    c
}

fn main() {
    println!("{}", test1());
    println!("{}", test2());
    println!("{}", test3(Foo(1, 2)));
    println!("{}", test4(Box::new(Foo(1, 2))));
    println!("{}", test5(Box::new(Foo(1, 2))));

    println!("{}", test7(Box::new(Bar { fst: 1, snd: 2 })));
}
