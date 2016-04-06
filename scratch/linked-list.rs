
pub mod list {

#[derive(Debug)]
    enum ListNode<T> {
        Cons(T, Box<ListNode<T>>),
        Nil
    }

#[derive(Debug)]
    pub struct List<T> {
        head: ListNode<T>,
    }

    use std::mem;
    use list::ListNode::*;

    impl <T> List<T> {

        pub fn new() -> Self { List { head: Nil } }

        pub fn cons(&mut self, val: T) {
            let head = mem::replace(&mut self.head, Nil);
            self.head = Cons(val, Box::new(head));
        }

        pub fn uncons(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Nil) {
                Nil => None,
                Cons(val, next) => { self.head = *next; Some(val) }
            }
        }
    }

    impl <T> Iterator for List<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.uncons()
        }
    }

    impl <T> Drop for List<T> {
        fn drop(&mut self) {
            while let Some(_) = self.uncons() { () }
        }
    }

}

fn main() {
    let mut list = list::List::new();

    for x in 0..10 { list.cons(x) };

    println!("{:?}", list);
}
