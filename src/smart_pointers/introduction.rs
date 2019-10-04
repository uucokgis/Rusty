/*

Box<T> for allocating values on the heap
Rc<T>, a reference counting type that enables multiple ownership
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
With RefCell<T>, these invariants are enforced at runtime.

The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler
is unable to understand and guarantee that.

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a
compile-time error if you try using it in a multithreaded context


Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows
checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.

Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the
RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the interior mutability pattern.

The standard library has other types that provide interior mutability, such as Cell<T>, which is
similar except that instead of giving references to the inner value, the value is copied in and out
of the Cell<T>. There’s also Mutex<T>, which offers interior mutability that’s safe to use across
threads; we’ll discuss its use in Chapter 16. Check out the standard library docs for more details on
the differences between these types.
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

/*
References Cycle
Burayı türkçe anlatmak istiyorum.
Diyelim ki bir a variable'ı olsun. Bu değişken Rc'ler RefCell'ler gibi Box'lar tutuyor.
Bir de b variable'ı olsun. a ile b arasında da ilişki var. Şu şekilde:
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
Yani Rc::clone ile a referansı arttırılarak b'nin box'ının içine basılıyor.

Scope dışına çıktığımız zaman maalesef a'nın referans sayısı 0 olmayacak ve o değer silinmeyecek.
Çözüm:
- Referans cycle'lar her zaman oluşmaz. Bunu oluşturmamaya dikkat edebiliriz.
- Rust, senin interior mutability de kattığın iç içe Rc'li Refcell'li boxlardaki reference cycle'ları
tespit edemez. Reference cycle'lar logic bir bug da olabilir çünkü.
- Yapıyı değiştirmek. İç içe rc refcell'ler değiştirilir. Gerekirse data'yı gerçek clone'larsın.
Rc'leri Weak'lere çevirmek:
Weak'lar ch15-06'da anlatılan bir Box tipi. üretildiğinde sana bir Weak smart pointer döndürüyor.
Bu pointer Rc::downgrade ile referansları 0'a düşürebiliyor. Rc'den farkı silinmesi için 0'a
düşmesine gerek olmaması.

*/

/*
*leaf.parent.borrow_mut() = ... muhabbeti  EXPLAINED
şimdi şöyle ki, borrow mut dediğimiz zaman leaf.parent'ın &mut hali bulunur. Örneğin leaf
Rc<Node> ise ve leaf.parent da RefCell<Weak<Node>> ise, &mut Weak<Node> elde edersin.
Sonra bunu dereference ederek Weak<Node> eline geçer ki değiştirmek istediğimiz değer de tam olarak
budur. Bunu mesela Rc::downgrade(&leaf) ile kullanabilirsin.

*/