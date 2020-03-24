fn main () {
    let values = vec![1,2,3,4,5];
    let l_values = largestv2(&values);

    let values2 = vec!["a", "b", "c"];
    let l_v2 = largestv2(&values2);

    println!("l_values : {:?} \n l_v2 : {:?}", l_values, l_v2);
}

fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut buyuk = &list[0];

    for value in list.iter() {
        if value > buyuk {
            buyuk = value;
        }
    }

    buyuk
}

fn largestv2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut buyuk = list[0].clone();

    for value in list.iter() {
        if value > &buyuk {
            buyuk = value.clone();
        }
    }

    buyuk
}