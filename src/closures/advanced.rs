struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>, // sayı alabilir veya None olabilir.
}

impl <T> Cacher<T>
    where T: Fn(u32) -> u32

{
    fn new(calculation: T) -> Cacher<T>  // Cacher struct'ı T genericsi aldigi için implementasyon
    // eger Cacher dondurecekse T tipinde obje ile çalışmalı. Bu obje sonradan Cacher instance'ının calculation
    // fieldı olacak.
    {
        Cacher
            {
                calculation,
                value: None
            }
    }

    fn value(&mut self, arg: u32) -> u32  // bunu yapmak için mutable instance gerekir. Çünkü self.value değeri değişecek.
    /*
    Eger self.value degeri bir sayı ise aynı şekilde dursun ve o deger dönsün. Aslında
    bu fieldın getter setter'ı gibi bir şey oluyor.

    Eğer değer none ise, yani yukarıdaki new fonksiyonu kullanılarak instance oluşturulmuş,
    şimdi bunu none bırakmayalım derseniz, cacher_instance::new ile kullanacağız.

    Peki değeri ne verelim? Verilecek değer T generic type'ından gelecek. Hatırlarsan bu type
    bir fonksiyon olmalıydı, yukarıda öyle tanımladık. Fonksiyon u32 tipinde bir değer
    alıyor ve onu döndürüyor. Dolayısıyla biz de cacher instance'ımızın value değerini
    getlemek istiyorsak önce onu setlemeliyiz, bunu yapmadan direk instance_var.value ile
    de çağırabilirsin elbette. Ama biz burada None sevmeyiz.

    Bu yüzden de o fonksiyonun döndürdüğü değer hem field'a set edilecek hem de döndürülecek.
    */
    {
        match self.value {
            Some(v) => v,
            None =>
                {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
        }
    }
}

// run this.
fn runforme() {
    let sample_lazy_function = |num: u32| -> u32 {num};

    let mut c = Cacher::new(sample_lazy_function);
    println!("c value : {:?}", c.value);

    c.value(3);
    println!("c new value : {:?}", c.value);

}

