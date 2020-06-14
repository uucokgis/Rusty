mod advancedv1 {
    struct CustomSmartPointer {
        data: String
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data : {}", &self.data);

        }
    }

    // normalde burda scope yoktu, ekleyince daha net anlaşılıyor ne yapıp yapmadığı
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("your stuff")};
    std::mem::drop(c);  // ahaha double free olmayacağı için pas geçiyor yapmıyor, müthiş.
    println!("c variable bilerek drop edildi");
    println!("CustomSmartPointers created");
    //    println!("c data : {}", c.data);
}

mod advancedv2 {

/*
The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’
implementations of clone do. The call to Rc::clone only increments the reference count,
which doesn’t take much time.
*/
use crate::List::{Cons, Nil};
use std::rc::Rc;
    enum List {
    Cons(i32, Rc<List>),
    Nil }



pub fn sample() {
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));
}
}


advancedv3 {
enum List {
Cons(i32, Rc<List>),
Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

pub fn sample () {
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("Count after creating a : {}", Rc::strong_count(&a));

let b = Cons(3, Rc::clone(&a));
println!("Count after creating b : {}", Rc::strong_count(&a));

{
let c = Cons(4, Rc::clone(&a));
println!("Count after creating c in scope : {}", Rc::strong_count(&a));
}

println!("count after c goes out of scope : {}", Rc::strong_count(&a));
}
}

mod RcFefCellCombination {
#[derive(Debug)]
enum List {
Cons(Rc<RefCell<i32>>, Rc<List>),
Nil
}

use crate::List::*;
use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));

let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

*value.borrow_mut() += 10;

println!("a after = {:?}", a);
println!("b after = {:?}", b);
println!("c after = {:?}", c);
}


mod reference_cycles {
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::*;

#[derive(Debug)]
#[allow(dead_code)]

enum List {
Cons(i32, RefCell<Rc<List>>),
Nil
}

impl List {
fn tail(&self) -> Option<&RefCell<Rc<List>>> {
match self {
Cons(_, item) => Some(item),
Nil => None,
}
}
}
pub fn sample () {
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
println!("a initial rc count = {}", Rc::strong_count(&a));  // 1
println!("a next item = {:?}", a.tail());  // (Rc::new(Nil)))


let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
println!("a rc count after b creation = {}", Rc::strong_count(&a));  // 2

println!("b initial rc count = {}", Rc::strong_count(&b));  // 1
println!("b next item = {:?}", b.tail()); // RefCell::new(Rc::clone(&a))


if let Some(link) = a.tail() {
*link.borrow_mut() = Rc::clone(&b);
}
println!("b rc count after changing a = {}", Rc::strong_count(&b));
println!("a rc count after changing a = {}", Rc::strong_count(&a));

// Uncomment the next line to see that we have a cycle;
// it will overflow the stack
//     println!("a next item = {:?}", a.tail());
}
}

mod weak_strong_referencing {
use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
value: i32,
children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct NodeWithParent {
value: i32,
children: RefCell<Vec<Rc<NodeWithParent>>>,
parent: RefCell<Weak<NodeWithParent>>,
}

pub fn sample () {
let leaf = Rc::new(Node {
value: 3,
children: RefCell::new(vec![])
});

let branch = Rc::new(Node {
value: 5,
children: RefCell::new(vec![Rc::clone(&leaf)])
});

/*
if a parent node is dropped, its child nodes should be dropped as well.

However, a child should not own its parent:
if we drop a child node, the parent should still exist. This is a case for weak references!
*/

let leafv2 = Rc::new(NodeWithParent {
value: 3,
parent: RefCell::new(Weak::new()),
children: RefCell::new(vec![]),
});
println!("leaf parent = {:?}", leafv2.parent.borrow().upgrade());

let branchv2 = Rc::new(NodeWithParent {
value: 5,
parent: RefCell::new(Weak::new()),
children: RefCell::new(vec![Rc::clone(&leafv2)])
});
// and then we use the Rc::downgrade function to create a Weak<Node> reference to branch from the Rc<Node> in branch.
*leafv2.parent.borrow_mut() = Rc::downgrade(&branchv2);

println!("leaf parent = {:?}", leafv2.parent.borrow().upgrade());

// test reference count
println!("leaf reference count = {:?}", Rc::strong_count(&leafv2));  // 2
println!("branch reference count = {:?}", Rc::strong_count(&branchv2));  // 1

println!("leaf reference count = {:?}", Rc::weak_count(&leafv2)); // 0
println!("branch reference count = {:?}", Rc::weak_count(&branchv2));  // 1
}
}

mod another {
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
value: i32,
children: RefCell<Vec<Rc<Node>>>,
parent: RefCell<Weak<Node>>
}

pub fn example () {
let leaf = Rc::new(Node {
value: 3,
parent: RefCell::new(Weak::new()),
children: RefCell::new(vec![])
});
println!("leaf reference counts -> strong = {}  ;  weak = {}",
Rc::strong_count(&leaf),
Rc::weak_count(&leaf));

{
println!("Inside of scope..");
let branch = Rc::new(Node {
value: 5,
parent: RefCell::new(Weak::new()),
children: RefCell::new(vec![Rc::clone(&leaf)])
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

println!("branch reference counts -> strong = {}  ;  weak = {}",
Rc::strong_count(&branch), Rc::weak_count(&branch));

println!("leaf reference counts -> strong = {}  ;  weak = {}",
Rc::strong_count(&leaf), Rc::weak_count(&leaf))
}

println!("Outside of scope..");

println!("leaf parent = {:?}", leaf.parent.borrow_mut().upgrade());

println!("Leaf reference counts -> strong = {}  ;   weak = {}",
Rc::strong_count(&leaf),
Rc::weak_count(&leaf)
);
}
}