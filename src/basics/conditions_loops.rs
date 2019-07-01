// expressions.
// if'ler bir expression oldugu için true veya false döndürür bunun üzerine
// statement'lar çalıştırırsın.

fn main() {
    let num = 6;
    if num == 5 {
        println!("sayı 5 değildir !");
    }

    else if num % 3 != 0 { // buraya girmez hiç.
        println!("6 sayısı 3'e tam bölünür. ");
    }
    else {
        println!("hiçbir şey yapamaz")
    }

    yan();
    iterate_list_w_index();
    iterate_sample();
}

fn yan() {
    let cond = true;

    let k = if cond {
        5
    } else {
        6
    };

    println!("number is : {}", k);
}

//fn infiniteloop() {
//    loop {
//        println!("sonsuz döngü !");
//    }
//}

//fn ret_loop () {
//    let mut counter = 0;
//
//    let result = loop {
//        println!("counter nedir : {}", counter);
//        counter += 1;
//
//        if counter == 10 {
//            break counter * 2;
//        }
//    };
//    println!("result nedir : {}", result);
//    assert_eq!(result, 20);
//}

fn iterate_list_w_index() {
    let sample_list = [1, 2, 3, 4, 5];
    let mut cnt = 0;

    while cnt < sample_list.len() {
        println!("sample list degeri : {}", sample_list[cnt]);
        cnt += 1;
    }
}


fn iterate_list_w_for(){

    let sample_list = [1,2,3,4,5];

    for element in sample_list.iter() {
        println!("element nedir : ")
    }
}

fn iterate_sample() {
    for number in (1..5) { // eger (1..5).rev() yaparsam tersine çevirir.
        println!("{}", number);
    }
    println!("bitti")
}