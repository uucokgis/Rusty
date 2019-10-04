use std::sync::{MutexGuard, Mutex};
use std::thread;

mod intro {
    pub fn basics () {
        let m = Mutex::new(5);

        {
            let mut num:MutexGuard<i32>= m.lock().unwrap(); // To access the data inside the mutex, we use the lock method to acquire the lock.
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    #[forbid()]
    pub fn sharing_mutex_between_multiple_threads_blow_code() {
        let counter = Mutex::new(5);
        let mut handles = vec![];

        let t1 = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num = 6;

        });
        handles.push(t1);

        let t2 = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num = 8;
        });

        handles.push(t2);

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result : {}", *counter.lock().unwrap());
    }

    #[forbid()]
    pub fn multiple_ownership_with_multiple_threads_blow_code() {
        let counter = Rc::new(Mutex::new(5));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("The result : {:?}", *counter.lock().unwrap());
    }
    
    pub fn example_fromthe_book() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("The result : {}", *counter.lock().unwrap());

        // Note from the book
        /*
        You might have noticed that counter is immutable but we could get a mutable reference to the value
        inside it; this means Mutex<T> provides interior mutability, as the Cell family does.
    
        In the same way we used RefCell<T> in Chapter 15 to allow us to mutate contents inside an Rc<T>,
        we use Mutex<T> to mutate contents inside an Arc<T>.
    
        */
    }
}