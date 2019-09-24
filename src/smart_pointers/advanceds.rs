mod advancedv1 {
    struct CustomSmartPointer {
        data: String
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            let data = &self.data;
            println!("data : {}", data);
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