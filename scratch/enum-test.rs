enum Foo {
    Bar(usize),
    Baz
}

fn main() {
    let foo: Foo = Foo::Bar(6);

    match foo {
        Foo::Bar(n) => println!("{}", n),
        Foo::Baz => ()
    }
}
