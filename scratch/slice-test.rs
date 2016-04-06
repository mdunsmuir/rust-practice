struct Foo {
    things: Vec<i64>,
}

impl Foo {
    fn new() -> Foo {
        Foo { things: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

impl std::ops::Index<usize> for Foo {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 { &self.things[index as usize] }
}

fn main() {
    let f = Foo::new();
    let s = &f[3];

    println!("{}", s);
}
