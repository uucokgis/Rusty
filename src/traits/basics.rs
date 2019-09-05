pub mod basics {

    pub fn introduction_to_traits () {
        /*
A type’s behavior consists of the methods we can call on that type. Different types share the
same behavior if we can call the same methods on all of those types. Trait definitions are a way
to group method signatures together to define a set of behaviors necessary to accomplish some
purpose.

For example, let’s say we have multiple structs that hold various kinds and amounts of text:
a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can
have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet,
or a reply to another tweet.

We want to make a media aggregator library that can display summaries of data that might be stored in
a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we need to request
that summary by calling a summarize method on an instance.
*/
        pub trait Summary {
            fn summarize(&self) -> String;

        }
    }

    pub fn create_traits_for_multiple_structs () {
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,

        }

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{} : {}", self.username, self.content)
            }
        }
    }
}

pub mod advanceds {
    // some codes may be duplicated with above
    use std::fmt::{Display, Debug};
    use std::iter::Sum;
    use std::borrow::Borrow;

    pub fn main () {
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,

        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        pub trait Summary {
            fn summarize(&self) -> String;

            fn summarize_with_author(&self) -> String;
        }

        impl Summary for NewsArticle {
            fn summarize (&self) -> String {
                format!("author : {} \n \
                     content : {}  via {} of location under {} context",
                        self.author, self.content, self.location, self.headline)
            }

            fn summarize_with_author(&self) -> String {
                format!("author : {}  ;  content : {}", self.author, self.content)
            }

        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{} nickname user wrote {} \n\
            reply : {} \n\
            retweet : {}", self.username, self.content, self.reply, self.retweet)
            }

            fn summarize_with_author(&self) -> String {
                format!("{} username and {} content ", self.username, self.content)
            }
        }

        pub fn notify(item: & impl Summary) {  // I want to write as borrowed.
            // Official document does not represent like this by the way
            // Because ownership will be disappeared after used this function
            // Bence bu bayaa mantıklı olmuş.
            println!("Breaking news : {}", &item.summarize());
        }

        pub fn notify_generic<T: Summary> (item: & T) {
            // Bu ornek notify ile aynı ama biraz daha detaylı ve esnek. Summary interface'ine
            // yönelik olmaktan ziyade Summary interface'ini kullanan objeye yönelik.
            println!("Breaking news on generic ! : {}", item.summarize());
        }

        pub fn notify_multi_generic <T: Summary> (item1: T, item2: T) {
            println!("Multi generic example");
            println!("breaking news for item1 : {}", item1.summarize());
            println!("Breaking news for item2 : {}", item2.summarize());
        }

        pub trait TestTrait {
            fn test_noself() -> String{
                format!("Test - noself")
            }
            fn test_self(&self) -> String{
                format!("Test  - self")
            }

        }

        // println!("Test trait no self : {}", TestTrait::test_noself());  HATALI NEDEN ANLAMADIM.

        pub fn notify_multi_genericv2 <T: Summary, U: TestTrait> (item1: T, item2: U) {
            println!("item1 : {}", item1.summarize());
            println!("item2 : {}", item2.test_self());
        }

        let tweet = Tweet {
            username: String::from("Umut"),
            content: String::from("oh my gosh !"),
            reply: true,
            retweet: false
        };

        let news = NewsArticle {
            headline: String::from("asdasd"),
            location: String::from("asdasd"),
            author: String::from("asdasd"),
            content: String::from("asdasd"),
        };
        println!("news summarize : {}", news.summarize());
        println!("tweet summarize : {}", tweet.summarize_with_author());

//    notify(news);  // süper değil mi

        // before running below statement, we need to implement testtrait into news
        impl TestTrait for NewsArticle {
            // using default implementation
        }
        notify(&news);

        // create 2 tweet objects too
        let tweet2 = Tweet {
            username: String::from("Umut"),
            content: String::from("oh my gosh !"),
            reply: true,
            retweet: false
        };
        let tweet3 = Tweet {
            username: String::from("Umut"),
            content: String::from("oh my gosh !"),
            reply: true,
            retweet: false
        };

        notify_generic(&tweet); // look at this example after understanding two statements below:

        notify_multi_genericv2(tweet, news);
        notify_multi_generic(tweet2, tweet3);  // well we could use borrowed style too.
        // However, we didnot implement borrowed parameter into this "trait bound". I mean function

        /*
        Let me explain notify_generic statement
        As you see on generic and genericv2 functions, we can not use borrowed item. Because we did not
        implement borrowed parameters. However, borrowing can be applied around everywhere. Look at
        notify_generic function. We implemented that and can be used. Note that, we cannot run
        notify_generic after generic functions. Because ownership was moved into those functions
        and we lost pointer (ownership) after those. This is great capabilities if you know what you do
        */


        // go go go
        // More details into trait bounds with "+" syntax

        pub fn notifyv2(item: impl Summary + Display) { // display built in trait
            // Note that we cannot override functions of Display.
            // Because that trait was written outside of this scope.
            // I cant use this because I dont know how to implement Display trait into any struct
        }

        pub fn notifyv3<T: Summary + TestTrait> (item: T) {
            println!("notify v3");
            // I shoudl put underscore before item parameter. Because we dont use in this method.
            // For details, see : some_function_easier_toread method below
        }
        impl TestTrait for Tweet {

        }
        let tweet4 = Tweet {
            username: String::from("Umut"),
            content: String::from("oh my gosh !"),
            reply: true,
            retweet: false
        };

        notifyv3(tweet4);

        // Official document states that using trait bound has many downsides if you use those so many times.
        // Because you or your team may confuse reading signatures of methods and names.
        // That's why they implement a keyword : where

        // long and confusing way:
        fn some_function_long <T: Display + Clone, U: Clone + Debug> (t: T, u: U) -> i32 {
            5
        }
        let _sfl = some_function_long(5, 12);  // Clone trait was implemented into numbers

        fn some_function_easier_toread<T, U> (_t: T, _u: U) -> i32  // I used "_" prefix. Why? Let me explain that:
        /*
        In rust, everything is mutable default and every variable should be used due to avoiding dead code.
        However, if that variable will be ignored, because you did not use, you can put that prefix.
        Second meaning is macros. You may put your macros and planned to use in the future. Even if this, probable you might want to
        document why you ignore this macro
        Third meaning is RAII. Like C++ Rust destruct objects which are not used no more and this prefix helps it.

        */
            where T: Display + Clone,
                  U: Clone + Debug
        {
            println!("t : {}", _t); // we used and no error raises by the way
            12 }

        let _sfet = some_function_easier_toread(5, 12);

        // Alright lets keep moving
        // What if we want to implement a trait to returned variable? Let me show:
        fn returns_summarizable () -> impl Summary {
            /* I am gonna write this anyway because that I have to return something.
            WARNING: This returned variable has to containt Summary trait. For example, you cannot return an integer or string
            Because Summary trait was not implemented to those types.

            The point is if we want to return a data but no matter which and what data and only
            worry about implemented trait, you may want to use it.
            For example we could return News also. Or any data and object which contains Summary trait.
            */

            // IMPORTANT
            /*
            However, you can only use "impl trait" if you return a single type. For example, if we write
            an if condition which can return either news or twitter object, it would not work.
            See : Chapter 17 from the book.
            */
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
        let tweet5 = returns_summarizable();

        // Remember:
        // Listing 10-5 example of the book states that you cannot use comparing operator on a "T" type.
        // Because there may be an input parameter which does not contain PartialOrd trait.
        // Lets solve it.

        fn find_largest_in_list<T: PartialOrd + Copy>(list: &[T]) -> T {
            // Purpose is finding the "largest" item in a list whether including chars or numbers or anything
            // IMPORTANT : WE HAVE TO IMPLEMENT COPY TRAIT INTO FUNCTION AS TRAIT BOUND.
            /*
            Because list variable can be anything according to generic rules. Even if i32 and char can be stored into
            stack, there may be an object which stored in heap and does not contain Copy trait (objects in stack have always copy trait)

            */
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item
                }
            }
            largest
        }

        // Using trait bounds to conditionally implement methods
        /*
        For example, the type Pair<T> in Listing 10-16 always implements the new function. But Pair<T>
        only implements the cmp_display method if its inner type T implements the PartialOrd trait that
        enables comparison and the Display trait that enables printing.
        */

        mod conditional_trait {
            use std::fmt::Display;

            pub fn sample () {

                struct Pair <T> {
                    x: T,
                    y: T
                }

                impl <T> Pair<T> {
                    fn new(x: T, y: T) -> Self {
                        Self {
                            x,
                            y
                        }
                    }
                }

                impl <T: Display + PartialOrd> Pair<T> {
                    fn cmp_display(&self) {
                        if self.x > self.y {
                            println!("The largest number is x : {}", self.x);
                        }
                        else {
                            println!("The largest number is y : {}", self.y);
                        }
                    }
                }

                /*
                 Mesela bir trait'i ancak bizim istediğimiz trait'e sahip olan bir type
                 geldiğinde implement edecek bir şey yazabiliriz.

                 Örneğin ToString trait'i builtin bir trait. Bunu sadece Display trait'i alan objeler
                 için implement edelim

                  For example, we can turn integers into their corresponding String values like this
                  because integers implement Display
                */
//            impl <T: Display> ToString for T {
                // error is : // Mesela bir trait'i ancak bizim istediğimiz trait'e sahip olan bir type
                //            // geldiğinde implement edecek bir şey yazabiliriz.
            }

        }
    }

}