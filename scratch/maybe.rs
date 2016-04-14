#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

use Maybe::*;

impl <T> Maybe<T> {

    fn as_ref(&self) -> Maybe<&T> {
        match self {
            &Nothing => Nothing,
            &Just(ref value) => Just(value),
        }
    }

    fn as_mut(&mut self) -> Maybe<&mut T> {
        match self {
            &mut Nothing => Nothing,
            &mut Just(ref mut value) => Just(value),
        }
    }

    fn map<U, F>(self, f: F) -> Maybe<U>
        where F: FnOnce(T) -> U {

        match self {
            Nothing => Nothing,
            Just(val) => Just(f(val)),
        }
    }

}

fn main() {

    let mut maybe = Just(5);

    println!("{:?}", maybe);
    println!("{:?}", maybe.as_ref());

    maybe.as_mut().map(|val| { *val = 6 });

    println!("{:?}", maybe);

}
