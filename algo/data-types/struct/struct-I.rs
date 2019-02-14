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