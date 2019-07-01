// Enumerations - I

#![allow(unused_variables)]

fn main() {
    #[derive(Debug)]
    enum IpaddrKind {
        V4,
        V6
    }

    // enum instances
    let four = IpaddrKind::V4;
    let six = IpaddrKind::V6;

    // we can define a func to be used for both instances
    fn route(ip_type: IpaddrKind) { }

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
    println!("kriger kind : {:?}", kriger.kind); // struct'ın datasını nası çektiğimizi hatırla

}


fn ip_scope() {
    # [derive(Debug)]

    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IPAddr::V4(127, 0, 0, 1);

    let loopback = IPAddr::V6(String::from("::1"));


}