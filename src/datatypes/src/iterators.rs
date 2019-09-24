pub fn sample_iterator() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

pub fn iterator_with_closure() {
    let v1 = vec![1,2,3,4];

    let v1_added: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v1 added : {:?}", v1_added);
}

pub fn iterator_types () {
    let v1 = vec![1,2,3,4];
    for refvalue in v1.iter() {
        println!("ref value : {}", refvalue);
    }

    for owned in v1.clone().into_iter() {  // you can remove clone() but then you cant use v1 due to ownership
        println!("owned value : {}", owned);
    }

    let mut v2 = v1.clone();
    for owned_mutable in v2.iter_mut() {
        *owned_mutable += 5;  // dereferencing. I'm not sure that do we have to type this?
        println!("owned value added five : {}", owned_mutable);
    }

}

pub mod iterator_with_filters {
    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String
    }

    pub fn shoes_in_mysize(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|anlamsizvar| anlamsizvar.size == shoe_size).collect()
    }

    pub fn sample_func() {
        let myshoe = Shoe {size: 15, style: String::from("My style")};
        let othershoe = Shoe{size: 13, style: String::from("Your style")};

        let result = shoes_in_mysize(vec![myshoe, othershoe], 15);
        println!("suitable shoes for my shoe : {:?}", result);
        // othershow will not be appeared.
    }
}

pub mod creating_your_iterators_for_yourdatatypes {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new () -> Counter {
            Counter {count: 0}
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))  // iki Counter instance açılır ama ikincisi birincisinden bir ileridedir.
            .map(|(a, b)| a * b)  // alinan iki değer tuple olarak alınır ve çarpılarak döndürülür.
            .filter(|x| x % 3 == 0)  // bu döndürülen değerlerin içerisinden 5 ile bölünenler filtrelenir
            .sum();  // toplamı verilir.
        /*
        Daha genişçe açıklamak gerekirse,
        1-2 alınır, 1*2 = 2 döndürülür, 5 ile tam bölünmediği için filtreye girmez
        2-3 alınır,
        3-4
        4-5 alınır, 4*5 = 20 döndürülür, 5 ile tam bölündüğü için filtreye girer.

        Sadece 20 kalıyor.

        Peki nereye kadar? Counter struct'ına iterator trait'i implement ettik ve next'in sınırını belirledik
        Next None olana kadar devam ediyor. Ama none'a denk geldiğinde durması gereken kısmı nasıl ayalıyor emin değilim
        Muhtemelen map kısmında a : u32 b: u32 kısımlarında anlıyor.
        */
        println!("sum : {}", sum);
    }

    fn runforme() {

        let mut cnt = Counter::new();
        &cnt.next();
        &cnt.next();
        println!("counter : {}", cnt.count);
        using_other_iterator_trait_methods();
    }
}