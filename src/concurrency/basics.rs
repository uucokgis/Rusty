mod intro {
    pub fn example () {
        thread::spawn(|| {
            for i in 1..10000 {
                println!("hi number {} from the spawned thread !", i);
//            thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn examplev2 () {
        let handle = thread::spawn(||{
            for i in 1..10 {
                println!("hi number {} from the spawned thread ! ", i);
            }
        });

        for i in 1..3 {
            println!("hi number {} from the main thread ! ", i);
//        thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }

    pub fn blow () {
        let v: Vec<i32> = vec![1,2,3];

        let handle = thread::spawn(|| {
            println!("vector : {:?}", v) // &v will be exploit in any case.
        });
    }

    pub fn not_leave_threads_without_running () {
        let v: Vec<i32> = vec![1,2,3,4,5];
        let handle = thread::spawn(move || {
            println!("the vector : {:?}", v)
        });

        handle.join().unwrap();
    }
}