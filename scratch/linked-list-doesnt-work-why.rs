pub mod list {

    #[derive(Debug)]
    struct Node<T>(T, Option<Box<Node<T>>>);

    #[derive(Debug)]
    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    impl <T> List<T> {

        pub fn new() -> Self {
            List { head: None  }
        }

        pub fn cons(&mut self, val: T) {
            let old_head = self.head.take();
            self.head = Some(Box::new(Node(val, old_head)));
        }

        pub fn uncons(&mut self) -> Option<T> {
            self.head.take().map(|head| {
                let Node(val, next) = *head;
                self.head = next;
                val
            })
        }

    }

}

fn main() {
    let mut list: list::List<usize> = list::List::new();

    for x in 0..10 { list.cons(x) };

    println!("{:?}", list);
}
