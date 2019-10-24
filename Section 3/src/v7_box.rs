use std::fmt::Debug;

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

fn main() {
    let mut ll = LinkedList {
        data: 34,
        next: None,
    };

    ll.next = Some(Box::new(LinkedList {
        data: 23,
        next: None,
    }));

    let mut v: Vec<Box<Debug>> = Vec::new();

    v.push(Box::new(ll));
    v.push(Box::new("hello".to_string()));

    println!("v = {:?}", v);
}
