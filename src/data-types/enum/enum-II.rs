// Enumerations - II
#![allow(unused_variables)]

fn main() {
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ben uydurdum
    fn quit() {
        panic!();
    }

    struct QuitMessage;
    struct MoveMessage {x: i32, y: i32,}
    struct WriteMessage(String); // tuple
    struct ChangeColorMsg(i32, i32, i32);

    impl Message {
        fn call(&self) {

        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}