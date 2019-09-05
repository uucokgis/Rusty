
pub mod enum_basics {
    // Enumerations - I

    pub fn basics() {
        enum IpaddrKind {
            V4,
            V6
        }

        // enum instances
        let four = IpaddrKind::V4;
        let six = IpaddrKind::V6;

        // we can define a func to be used for both instances
        fn route(ip_type: IpaddrKind) {}

        // running the func
        route(IpaddrKind::V4);

        // enum ile sadece matrix field'larını koymuş gibi olduk. Ama içlerini de doldurmamız lazım:

        struct IpAddr {
            kind: IpaddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpaddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpaddrKind::V6,
            address: String::from("::1"),
        };

        let kriger = IpAddr {
            kind: IpaddrKind::V4,
            address: String::from("::0"),
        };

        println!("kriger addr : {}", kriger.address);
        // println!("kriger kind : {:?}", kriger.kind); // struct'ın datasını nası çektiğimizi hatırla
    }

    pub fn example_ip_scope() {
        enum IPAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IPAddr::V4(127, 0, 0, 1);
        let loopback = IPAddr::V6(String::from("::1"));
    }

    pub mod enum_adv {
        pub fn message_enum_example() {
            fn main() {
                enum Message {
                    Quit,
                    Move { x: i32, y: i32 },
                    Write(String),
                    ChangeColor(i32, i32, i32),
                }

                // ben uydurdum
                fn quit() {
                    panic!();
                }

                struct QuitMessage;
                struct MoveMessage { x: i32, y: i32, }
                struct WriteMessage(String); // tuple
                struct ChangeColorMsg(i32, i32, i32);

                impl Message {
                    fn call(&self) {}
                }

                let m = Message::Write(String::from("hello"));
                m.call();
            }
        }
    }
}

pub mod enum_crashcourse {
    // enums are types which have a few definite values

    enum Movement {
        Up,
        Down,
        Left,
        Right,
    }

    fn move_avatar(m: Movement) {
        match m {
            Movement::Up => println!("up direction"),
            Movement::Down => println!("Down direction"),
            Movement::Left => println!("Left direction"),
            Movement::Right => println!("Right direction"),
        }

    }
    pub fn run () {
        let move_dir = Movement::Up;
        move_avatar(move_dir);
    }
}
