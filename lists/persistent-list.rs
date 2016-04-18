pub mod list {

    use std::rc::{Rc};

    type Link<T> = Option<Rc<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        value: T,
        next: Link<T>,
    }

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }

    impl <T> List<T> {

        pub fn new() -> Self {
            List { head: None }
        }

        pub fn cons(&self, value: T) -> Self {
            List { head: Some(Rc::new(Node { value: value, next: self.head.clone() } )) }
        }

        pub fn uncons(&self) -> Option<(&T, Self)> {
            self.head.as_ref().map(|head| {
                (
                    &head.value,
                    List { head: head.next.as_ref().map(|rc| rc.clone()) },
                )
            })
        }

        pub fn iter(&self) -> Iter<T> {
            Iter { list: self.head.as_ref().map(|rc| rc.as_ref() ) }
        }
    }

    impl <T> Clone for List<T> {

        fn clone(&self) -> Self {
            List { head: self.head.as_ref().map(|rc| rc.clone()) }
        }

    }

    impl <T> Drop for List<T> {

        fn drop(&mut self) {
            while let Some(rc_head) = self.head.take() {
                match Rc::try_unwrap(rc_head) {
                    Err(_) => break,
                    Ok(head) => self.head = head.next,
                }
            }
        }

    }

    pub struct Iter<'a, T: 'a> {
        list: Option<&'a Node<T>>,
    }

    impl <'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.map(|head| {
                self.list = head.next.as_ref().map(|next| &**next);
                &head.value
            })
        }

    }

}

use list::*;

fn main() {
    let mut lista = List::new();

    for x in 0..10 {
        lista = lista.cons(x);
    }

    let mut listb = lista.clone();

    for x in 10..15 {
        listb = listb.cons(x);
    }

    for x in lista.iter() {
        println!("{}", x);
    }

    for x in listb.iter() {
        println!("{}", x);
    }
}
