#![allow(unused)]
use std::rc::Rc;

/* This module will be taught in the class */

#[derive(Clone)]
struct LinkedList<T> {
    head: Option<Node<T>>,
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
