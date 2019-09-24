/*

Box<T> for allocating values on the heap
Rc<T>, a reference counting type that enables multiple ownership
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

*/

/*

Some informations about Box<T>
Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

*/

/*

Recursive metotlarda şöyle bir şey var, datanın büyüklüğü bilinmediği için derleme zamanında
anlaşılamıyor. Rust'ın ise bu bilgiye ihtiyacı var bildiğin gibi.
Box kullandığın zaman onun da stack'te tutulduğunu biliyorsun (data heap'te pointer stack'te)
ama recursive types yaparak sürekli olarak tipini değiştirebilirsin.
Böylelikle büyüklüğü derleme zamanında bilinmeyen metotlarda kullanmak için iyi bir teknik.

*/

mod basics {
    use crate::basics::List::{Cons, Nil};

    enum List {
        Cons(i32, Box<List>),
        Nil
    }

    pub fn create_box () {
        let b = Box::new(5);
        println!("b : {}", b);

    }

    pub fn cons_list_broken () {
        // cons is not special keyword by the way
        //broken
//        enum List<Box> {
//            Cons(i32, List<Box>),
//            Nil
//        }
    }

    pub fn create_box_list() {
        let list = Cons(1,
                        Box::new(Cons(2,
                                      Box::new(Cons(3,
                                                    Box::new(Nil))))));
    }
}

mod basicsv2{
    // aslında bu kısmı daha önce anlatmalıydım:
    pub fn assign_value() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);

        println!("y : {}", y);  // baglanti kopmaz, kullanabilirsin.
    }
    // simdi burada y kullanmazsan, eşitlik operatoru integer ile &integer'in karşılaştırılmasının
    // olanaksız olduğunu söyleyecek o yüzden dereference ediyoruz. bu mesela smart pointerlar ile direk ilintili:
    // ch15-02

}

mod creating_deref {
    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // however, we cannot dereference MyBox which we define type. Because deref trait was not implemented.

    // Sometimes you may want to dereference any arguments in the function automatically
    // this is deref coercions.

    // if we did not have dereference coersions:
    // put silly hello function
    fn hello(name: &str) {
        println!("hello, {}", name);
    }

    pub fn not_deref_coersions () {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    pub fn deref_coersions () {
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
    }

}

mod createing_derefmut {
    /*
    Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

    Rust does deref coercion when it finds types and trait implementations in three cases:

    From &T to &U when T: Deref<Target=U>
    From &mut T to &mut U when T: DerefMut<Target=U>
    From &mut T to &U when T: Deref<Target=U>
    */

}

// Lets more dive in Rc<T>
/*
In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
However, there are cases when a single value might have multiple owners

Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16,
we’ll cover how to do reference counting in multithreaded programs.

*/