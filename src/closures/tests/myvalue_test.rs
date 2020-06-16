use std::thread;
use std::time::Duration;
use std::thread::sleep;

struct MyValues {
    value: i32,
    opt: Option<i32>
}

fn main () {
    let mv = MyValues {
        value: 5,
        opt: Some(3)
    };

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread value : {}", i);
            // sleep(Duration::from_secs(1));
        }
    });

    let check_handle = thread::spawn(move || {
        match mv.opt {
            Some(t) => println!("Okay"),
            None => panic!("Not okay")
        }
    });

    for i in 1..5 {
        println!("main thread value : {}", i);
        // sleep(Duration::from_secs(1));
    }
    check_handle.join().unwrap();
    handle.join().unwrap();
    // drop(mv);
}