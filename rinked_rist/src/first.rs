use std::mem;

struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    Next(Box<Node>),
}

pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::Next(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Next(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
