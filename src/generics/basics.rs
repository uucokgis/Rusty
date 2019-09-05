pub mod basics {

    pub fn generic_sample_introduction() {
        // Consider a short program that finds the largest number in a list
        let number_list = vec![34, 50, 25, 100, 65];
        let char_list = vec!['y', 'm', 'a', 'q'];

        fn find_largest_number(list: &[i32]) -> i32 {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item
                }
            }

            largest
        }

        fn find_largest_char(list: &[char]) -> char {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item
                }
            }

            largest
        }

        /*
Aslında bu iki fonksiyonun gövdeleri aynı. Boşu boşuna ikisini de yazıp tutmaktansa generics
kullanacağız. Bunu Rust'ta anlatmak için T yazacağız - sık sık karşımıza çıkan zımbırtı -
T özel bişi değil. Başka bir harf veya kelime de kullanabilirsin ama Type anlamına gelen kısaltma
olan T çok yaygın.
*/
        fn largest<T>(list: &[T]) -> T {
            let mut largest = list[0];  // bu kod patlar.

            for &item in list.iter() {
                if item > largest {
                    largest = item
                }
            }
            largest
        }
    }

    pub fn using_generics_in_structs() {
        struct Point <T> {
            x: T,
            y: T
        }

        // below one will not work
//    let wont_work = Point {x:6, y:1.2};

        /*
        Çünkü Point struct T tipinde bir type değişkeni alıyor ve x ve y'nin aynı değişken tipinde
        olacağını öngörüyor.
        O yüzden ikinci bir tip tanımlamamız gerek.
        */

        struct Pointv2 <T, U> {
            x: T,
            y: U,
        }
        let will_work = Pointv2 {x: 12, y: 5.123123};
        let will_work2 = Pointv2 {x: 12.232, y: 51.123}; // this can work also
    }

    pub fn using_generics_in_enums () {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E)
        }
    }

    pub fn using_generics_in_methods () {
        /*
    Note that we have to declare T just after impl so we can use it to specify that we’re
    implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can
    identify that the type in the angle brackets in Point is a generic type rather than a
    concrete type.
    */

        struct Point<T> {
            x: T,
            y: T
        }

        impl <T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

//    impl <T> Point<T> {
//        fn y(&self) -> T {
//            &self.y
//        }
//    }

        let p = Point {
            x: 124,
            y: 51
        };

        println!("x of Point : {}", p.x());

        /*
        We could, for example, implement methods only on Point<f32> instances rather than on Point<T>
        instances with any generic type. In Listing 10-10 we use the concrete type f32, meaning we don’t
        declare any types after impl.
        */
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        /*
        This code means the type Point<f32> will have a method named distance_from_origin and other
        instances of Point<T> where T is not of type f32 will not have this method defined. The method
        measures how far our point is from the point at coordinates (0.0, 0.0) and uses mathematical
        operations that are available only for floating point types.
        */

    }

    pub fn using_generics_with_impl () {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl <T, U> Point<T, U> {
            fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y
                }
            }
        }

        let p1 = Point {x: 5, y: 10.5};
        let p2 = Point {x: "Hello", y: 'c'};
        let p3 = p1.mixup(p2);
        println!("p3.x : {}  ;  p3.y : {}", p3.x, p3.y);
    }

    pub fn using_generics_with_impl_reverse () {
        struct Point<T, U> {
            x: T,
            y: U
        }

        impl <T,U> Point<T, U> {
            fn mixup<V, W>(self, other_point: Point<V,W>) -> Point<V,U> {
                Point {
                    x: other_point.x,
                    y: self.y,
                }
            }
        }

        let p1 = Point {
            x: 5,
            y: "asdasd"
        };
        let p2 = Point {
            x: 12,
            y: String::from("asdasd")
        };

        let p3 = p1.mixup(p2);
        println!("reverse of p3. x : {}   ;   y: {}", p3.x, p3.y);
    }
}