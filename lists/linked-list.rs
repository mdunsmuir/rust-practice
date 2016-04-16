pub mod list {

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        next: Link<T>,
    }

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }

    impl <T> List<T> {

        pub fn new() -> Self {
            List { head: None  }
        }

        pub fn cons(&mut self, val: T) {
            let old_head = self.head.take();
            self.head = Some(Box::new(Node { value: val, next: old_head }));
        }

        pub fn uncons(&mut self) -> Option<T> {
            self.head.take().map(|head| {
                let head = *head;
                self.head = head.next;
                head.value
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|head| &head.value)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|head| &mut head.value)
        }

        pub fn iter(&self) -> ListIter<T> {
            ListIter { head: self.head.as_ref().map(|boxed| &**boxed) }
        }

        pub fn iter_mut(&mut self) -> ListIterMut<T> {
            ListIterMut { head: self.head.as_mut().map(|boxed| &mut **boxed) }
        }

    }

    pub struct ListIter<'a, T: 'a> {
        head: Option<&'a Node<T>>,
    }

    impl <'a, T: 'a> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.head.take().map(|head| {
                self.head = head.next.as_ref().map(|boxed| &**boxed);
                &head.value
            })
        }
    }

    pub struct ListIterMut<'a, T: 'a> {
        head: Option<&'a mut Node<T>>,
    }

    impl <'a, T: 'a> Iterator for ListIterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.head.take().map(|head| {
                self.head = head.next.as_mut().map(|boxed| &mut **boxed);
                &mut head.value
            })
        }
    }

    /*
    impl <'a, T> IntoIterator for &'a List<T> {
        type Item = &'a T;
        type IntoIter = ListIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl <'a, T: 'a> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next<'b>(&'b mut self) -> Option<Self::Item> {
            self.head.as_ref().map(|head| {
                self.head = &head.next;
                &head.value
            })
        }
    }
    */
}

fn main() {
    let mut list: list::List<usize> = list::List::new();

    for x in 0..10 { list.cons(x) };

    {
        let a = list.peek_mut();
        a.map(|val| *val = 20);
    }

    list.cons(40);

    for val in list.iter() {
        println!("{}", val);
    }

    for val in list.iter_mut() {
        *val += 7;
    }

    {
        let mut im = list.iter_mut();
        let a = im.next();
        let b = im.next();
    }

    list.cons(60);

    println!("{:?}", list);

    println!("{:?}", list.peek());
}
