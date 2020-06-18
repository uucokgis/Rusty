use std::process::id;

enum Message {
    Hello {idd: i32}
}

fn main () {
    let msg = Message::Hello {idd: 5};

    match msg {
        Message::Hello {
            idd: id_var @ 3..=7,
        } => println!("found an id in range : {}", id_var),

        Message::Hello {idd: 10..=12} => {
            println!("Found an id in range : {}", idd);
        }
    }
}