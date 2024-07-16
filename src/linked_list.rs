#![allow(unused)]

/* This module will be taught in the class */

struct LinkedList<'a, T> {
    head: Option<Node<T>>,
    last: &'a Option<Node<T>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn append(&mut self, t : T) {
        let node = Node { val : t, next : None };
        if self.head.is_none() {
            self.head = Some(node);
            self.last = &self.head;
        }
    }

}
