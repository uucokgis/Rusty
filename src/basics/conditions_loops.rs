// expressions.
// if'ler bir expression oldugu için true veya false döndürür bunun üzerine
// statement'lar çalıştırırsın.

pub mod basics {
    pub fn main() {
        let num = 6;
        if num == 5 {
            println!("sayı 5 değildir !");
        } else if num % 3 != 0 { // buraya girmez hiç.
            println!("6 sayısı 3'e tam bölünür. ");
        } else {
            println!("hiçbir şey yapamaz")
        }

        yan();
        iterate_list_w_index();
        iterate_sample();
    }

    pub fn yan() {
        let cond = true;

        let k = if cond {
            5
        } else {
            6
        };

        println!("number is : {}", k);
    }

    pub fn infiniteloop() {
        loop {
            println!("sonsuz döngü !");
        }
    }

    pub fn ret_loop () {
        let mut counter = 0;

        let result = loop {
            println!("counter nedir : {}", counter);
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("result nedir : {}", result);
        assert_eq!(result, 20);
    }

    pub fn iterate_list_w_index() {
        let sample_list = [1, 2, 3, 4, 5];
        let mut cnt = 0;

        while cnt < sample_list.len() {
            println!("sample list degeri : {}", sample_list[cnt]);
            cnt += 1;
        }
    }


    pub fn iterate_list_w_for() {
        let sample_list = [1, 2, 3, 4, 5];

        for element in sample_list.iter() {
            println!("element nedir : ")
        }
    }

    pub fn iterate_sample() {
        for number in (1..5) { // eger (1..5).rev() yaparsam tersine çevirir.
            println!("{}", number);
        }
        println!("bitti")
    }
}

pub mod loops_crashcourse {
    pub fn run () {
        let mut count = 0;

        // infinite loop
        loop {
            count += 1;

            if count == 5 {
                println!("count is : {}", count);
                break;
            }
        }

        // While loop (FizzBuzz)
        while count <= 100 {
            if count % 15 == 0{
                println!("fizzbuzz");

            } else if count % 3 == 0 {
                println!("fizz");
            } else if count % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", count);
            }

            // increment
            count += 1;
        }

        println!("for loop has started");

        // For range loop
        for x in 0..100 {

            if x % 15 == 0{
                println!("fizzbuzz");

            } else if x % 3 == 0 {
                println!("fizz");
            } else if x % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", x);
            }
        }
    }
}

pub mod loops_for_mid {
    pub fn basic_forloop() {
        // i just want to state that
        for i in 1.100 {
            println!{"i : {}", i};
        }
    }

    pub fn for_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("this is ferris");

                _ => {}
            }
        }
    }

    pub fn for_into_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter(){
            match name {
                "Ferris" => println!("This is ferriiis");

                _ => {}
            }
        }
    }

    pub fn for_iter_mut() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "Tahsiiim",

                _ => "Hellooğ"
            }
        }
        println!("all : {:?}", names);
    }
}