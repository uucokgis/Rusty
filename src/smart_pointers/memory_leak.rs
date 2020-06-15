use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};

// Node example
// please read here again:
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

#[derive(Debug)]
struct Node {
    point: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

fn main () {
    let leaf = Rc::new(Node {
        point: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("leaf is : {:?}", leaf);
    println!("leaf parent is : {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        point: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch : {:?}", branch);

    println!("leaf parent is : {:?}", leaf.parent.borrow().upgrade());
}