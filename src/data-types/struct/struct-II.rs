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