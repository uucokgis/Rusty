pub mod basics {
    // what is struct and sample

    fn main() {
        // sample struct
        struct Kullanici {
            kullanici_adi: String,
            email: String,
            giris_sayisi: u8,
            aktiflik: bool,
        }

        // this is default using struct as filling all fields.
        let k1 = Kullanici {
            kullanici_adi: String::from("kuttamuwa"),
            email: String::from("ucok_umut@yahoo.com.tr"),
            aktiflik: true,
            giris_sayisi: 13
        };

        fn kullanici_doldur(kullanici_adi: String, email: String) -> Kullanici {
            // we cant pass any arguments populating in kullanici
            // we cant use different parameters not in kullanici fields like
            // kullanici_new instead of kullanici_adi.

            Kullanici {
                kullanici_adi: String::from("aliveli"),
                email: String::from("ali@veli.com.tr"),
                aktiflik: true,
                giris_sayisi: 8
            }
        }


        fn kullanici_doldur_shorthand(kullanici_adi: String, email: String) -> Kullanici {
            // we will use short hand
            Kullanici {
                kullanici_adi,
                email,
                aktiflik: true,
                giris_sayisi: 9
            }
        }


    }
}

pub mod struct_instances {
    // struct - II
// struct instances

    fn main() {
        // sample struct
        struct Kullanici {
            kullanici_adi: String,
            email: String,
            giris_sayisi: u8,
            aktiflik: bool,
        }

        // lets define a user
        let user_one = Kullanici {
            kullanici_adi: String::from("kuttamuwa"),
            email: String::from("ucok_umut@yahoo.com.tr"),
            aktiflik: true,
            giris_sayisi: 13
        };

        /*
        There is very important something occurs. When we user user_one as filling mirror user one,
        we borrow kullanici adi from user one and didn't give back. Thus, we can't user one kullanıcı
        adı anymore.

        we cant use this mirror_user one either due to borrowed to mirror user two : )

        or we cant use out of kullanıcı adı on mirror user two because all of them are borrowed into
        mirror user three : )

        it was pretty case i wrote unwittingly
        */

        //

        let mirror_user_one = Kullanici {
            kullanici_adi: user_one.kullanici_adi,
            email: String::from("sample@example.com"), // another mail address
            aktiflik: false,
            giris_sayisi: 1

        };

        // we can pass all arguments like this
        let mirror_user_two = Kullanici {
            ..mirror_user_one
        };

        // or this
        let mirror_user_three = Kullanici {
            kullanici_adi: String::from("kullanıcıyım"),
            ..mirror_user_two // this is kind gonna be weird
        };


        println!("kullanıcı adı : {} \n \
    email : {} \n \
    aktiflik : {} \n\
    giriş sayısı : {} \n", mirror_user_two.kullanici_adi, mirror_user_three.email,
                 mirror_user_three.aktiflik, mirror_user_three.giris_sayisi);



    }
}

pub mod samples {
    // struct examples
    // how we use them

    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {width: 30, height: 50}; // instance of rectangle struct

        println!("Area of this rectangle is : {}", area(&rect1));

        //println!("rect1 : {}", rect1);
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

}

pub mod struct_tuples {
    pub fn run () {
        // tuple structs
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(3, 5, 12);
        let corner = Point(3, 3, 3);

        println!("black one : {}", black.0);
        println!("corner last : {}", corner.2);
    }
}

pub mod struct_crashcourse {
    /*
Structs used to create custom data types
*/

    // regular struct
    struct Color {
        red: u8,
        green: u8,
        blue: u8
    }

    // tuple struct
    struct ColorNew (u8, u8, u8);

    struct Person {
        firstname: String,
        lastname: String
    }

    impl Person {
        // constructor
        fn new(fname: &str, lname: &str) -> Person{
            Person {
                firstname: fname.to_string(),
                lastname: lname.to_string(),
            }
        }

        // get full name
        fn full_name(&self) -> String {
            format!("{} , {} ", self.firstname, self.lastname)  // no semicolon. this format! thing is gonna return that as string
        }

        // set last name
        fn set_last_name(&mut self, newlastname: &str) {
            self.lastname = newlastname.to_string();
        }

        // name to tuple
        fn to_tuple(self) -> (String, String) {
            (self.firstname, self.lastname)
        }

    }
    pub fn run () {
        let mut c = Color {
            red: 178,
            green: 121,
            blue: 159
        };

        c.blue = 0;

        println!("color : {} , {} , {} ", c.red, c.green, c.blue);

        let cnew = ColorNew(100, 150, 200);
        println!("color new : {} {} {}", cnew.0, cnew.1, cnew.2);  // it cannot use debug println trait :  {:?}

        // give me person
        let isim = String::from("umut");
        let soyisim = String::from("Ucok");

        let mut umutperson = Person::new(&isim, &soyisim);  // or "Umut", "Ucok" which are string literal and can be considered as &str
        println!("name : {} , surname : {}", umutperson.firstname, umutperson.lastname);
        println!("full name : {}", umutperson.full_name());

        // we wil change last name of umut
        umutperson.set_last_name("Doe");
        println!("last name of umut : {}", umutperson.lastname);

        println!("person tuple : {:?}", umutperson.to_tuple());
    }

}