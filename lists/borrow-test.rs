#[derive(Debug)]
struct Foo<'a, T: 'a> {
    bar: &'a mut Bar<T>,
}

#[derive(Debug)]
struct Bar<T> { val: T, }

fn foo<'a, 'b, T>(f: &'b mut Foo<'a, T>) -> &'b mut Bar<T> { 
    let ref mut temp = f.bar;
    f.bar = Bar { val: 98 };
    f.bar
}

fn main() {
    let mut b = Bar { val: 42 };
    let mut f = Foo { bar: &mut b };
    
    {
        let br = foo(&mut f);
        println!("{:?}", f);
        *br = Bar { val: 56 };
    }

    println!("{:?}", f);
}
