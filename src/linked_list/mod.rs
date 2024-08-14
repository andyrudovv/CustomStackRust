use node::{Node, Link};
use std::mem;

mod node;

pub struct IntoIter<T>(LinkedList<T>);

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    lenght: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, lenght: 0 }
    }
    
    pub fn add(&mut self, value: T) {
        let new_node = Box::new(
            Node {
                value: value,
                next: mem::replace(&mut self.head, None)
            }
        );

        self.head = Some(new_node);
        self.lenght += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.lenght -= 1;
            node.value
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    pub fn get_len(&self) -> usize {
        self.lenght
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut node_box) = current_node {
            current_node = node_box.next.take();
            // mem::replace(&mut node_box.next, None) == self.node_box.next.take()
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}