pub use std::thread;
pub use std::sync::mpsc;
pub use std::time::Duration;

mod tx_rx {

    pub fn multiple_values_recv_waiting () {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec!
            [String::from("hi"),
             String::from(" from"),
             String::from(" the"),
             String::from(" spawned"),
             String::from(" thread")];

            for val in vals {
                tx.send(val).unwrap();
                //            thread::sleep(Duration::from_secs(1));
            } }
        );

        println!("before receiving");
        thread::spawn(|| {
            println!("one dummy thread");  // can be run anytime
        });

        let msg = rx.try_recv();
        println!("msg : {:?}", msg);  // we interfered for once.

        for received in rx {
            println!("Got : {}", received);
        }
}
}

mod multisender {
    pub fn example () {
        let (tx, rx) = mpsc::channel();

        {
            // multiple sender (producer) one receiver (consumer) via cloning the transmitting half of the channel
            let tx1 = mpsc::Sender::clone(&tx);

            thread::spawn(move || {
                let vals = vec!
                [String::from("hi"),
                 String::from(" from"),
                 String::from(" the"),
                 String::from(" spawned"),
                 String::from(" thread")
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }

        {
            let tx2 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx2.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }

        for received in rx {
            println!("Got : {}", received);
        }
    }

    pub fn book_example () {
        let (tx, rx) = mpsc::channel();

        // multiple sender (producer) one receiver (consumer) via cloning the transmitting half of the channel
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec!
            [String::from("hi"),
             String::from(" from"),
             String::from(" the"),
             String::from(" spawned"),
             String::from(" thread")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        let tx2 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got : {}", received);
        }
    }
}