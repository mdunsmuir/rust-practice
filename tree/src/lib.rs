use std::rc::*;
use std::mem;

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

use Side::*;

#[derive(Debug)]
struct BSTNode<T> {
    contents: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Rc<BSTNode<T>>>;

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
    size: usize,
}

impl <T: PartialOrd + Copy> BSTNode<T> {

    fn insert<'a>(mut ptr: Option<&'a BSTNode<T>>,
                  mut stack: Vec<(&'a BSTNode<T>, Side)>,
                  item: T) -> Rc<BSTNode<T>> {

        while let Some(node) = ptr {
            let &BSTNode { contents, ref left, ref right } = node;

            if item <= contents {
                stack.push((node, Left));
                ptr = node.left.as_ref().map(|rc| rc.as_ref() );
            } else {
                stack.push((node, Right));
                ptr = node.right.as_ref().map(|rc| rc.as_ref() );
            }
        }

        let mut node_below = BSTNode { contents: item, left: None, right: None };

        while let Some((node_above, side)) = stack.pop() {

            match side {
                Left => {
                    let mut node_tmp = BSTNode { 
                        contents: node_above.contents,
                        left: None,
                        right: node_above.right.clone(),
                    };

                    node_tmp = mem::replace(&mut node_below, node_tmp);
                    node_below.left = Some(Rc::new(node_tmp));
                },

                Right => {
                    let mut node_tmp = BSTNode { 
                        contents: node_above.contents,
                        right: None,
                        left: node_above.left.clone(),
                    };

                    node_tmp = mem::replace(&mut node_below, node_tmp);
                    node_below.right = Some(Rc::new(node_tmp));
                },

            }
        }

        Rc::new(node_below)
    }

}

impl <T> BST<T> {

    pub fn new() -> Self {
        BST { root: None, size: 0 }
    }

}

impl <T: PartialOrd + Copy> BST<T> {

    pub fn insert(&self, item: T) -> Self {
        let stack = Vec::new();
        let root_ref = self.root.as_ref().map(|root| root.as_ref() );

        BST {
            root: Some(BSTNode::insert(root_ref, stack, item)),
            size: self.size + 1,
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let ta = BST::new();
        println!("{:?}", ta);
        println!("{:?}", ta.insert(5).insert(3).insert(8).insert(1));
    }
}
