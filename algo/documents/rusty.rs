// rusttaki notlari buraya yazmaya karar verdim.
// umut ucok, 2019.

/*
Rust'ta variable'lar N.S.A immutable'dır. Örneğin
let foo = 5;

Ama istenirse bu variable'lar mutable olabilir. Örneğin
let mut bar = 3;

Burada mut kelimesi o işe yarar.
Ayrıca bir variable tanımlanır tanımlanmaz input olarak da alınabilir. Örneğin
let mut guess = String::new();

Çok iyi bence :)
*/

/*
Rust çalışmaya başladığında bazı kütüphaneleri - bunlar olabildiğinde az ve küçük tutulmuştur -
yükler.
Ayrıca data type'lar da burada yüklenir. Eğer kullandığınız başka bir struct, datatype varsa use
ile yüklemeniz gerekir.
Ayrıca kullanılmayan fonksiyonlar (dead code) bir hata içeriyorsa compile'da gene
hata verir ve program çalışmaz.
*/

/*
kargoda bir çok standart kütüphane henüz built in değil. Örneğin random sayı üreten kütüphane
yazıldı ama built in yapılmadı, crate.
Bir projeyi yaptığınızda ve cargo.toml dosyanızdaki dependencies'leri güncellediğinizde ilk compile'da
cargo bu paketleri halleder sonra bir değişiklik olana kadar bir daha bulaşmaz.

*/

/* bir önemli mesele öğrendim. diyelim guess diye bir var tanımladık. Sonra bunu bir kere daha
tanımlayabiliyoruz çünkü Rust guess'in bir de shadow'unu almamıza izin veriyor..
Bunu kullanacağımız yerler type convertion kısımları. Python'da bunlar built-in func'lar ile sağlanıyor
biliyorsun. Örneğin a = "35" ; b = 55 ; assert int(a) == b, vs.

bu örnekte kullanılan guess.trim().parse() kullanımında trim metodu verilen string'in başındaki kıçındaki
boşlukları siliyor.
*/


/* Data types in Rust:

Length 	Signed 	Unsigned
8-bit 	    i8 	    u8
16-bit 	    i16 	u16
32-bit 	    i32 	u32
64-bit 	    i64 	u64
128-bit 	i128 	u128
arch 	    isize 	usize
* the isize and usize types depend on the kind of computer your program is running on: 64 bits if
 you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

*default data type in Rust is i32. This is generally the fastest even on 64 bit systems.

*In release builds, Rust does not check for overflow. and instead will do something
called “two’s complement wrapping.” In short, 256 becomes 0, 257 becomes 1, etc
Relying on overflow is considered an error, even if this behavior happens.
If you want this behavior explicitly, the standard library has a type, Wrapping, that provides
it explicitly.

*The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable
of more precision.
The f32 type is a single-precision float, and f64 has double precision.

Booleans are one byte in size.


*/

/* arrays and tuples
Arrays in Rust are different from arrays in some other languages because arrays in Rust have a
fixed length, like tuples. . Unlike a tuple, every element of an array must have the same type.

unlike tuple, we need to give lenght of array like:
let a: [i32;5] = [1,2,3,4,5];
 Since an array has a fixed size, this number is always the same, even if the array’s elements are
 modified, it cannot grow or shrink. Not like python !

*/

/* functions
returned için
fn returned_func() -> i32 { // vay amk illa  -> i32 sikini yazacakmışız.
    5

doc.rust-lang'te bahsedilen çok önemli bir mesele var, functions bölümünde geçiyor.

Aynen alıntı yapıyorum:
Function bodies are made up of a series of statements optionally ending in an expression. So far,
we’ve only covered functions without an ending expression, but you have seen an expression as part
of a statement. Because Rust is an expression-based language, this is an important distinction to
understand. Other languages don’t have the same distinctions, so let’s look at what statements and
expressions are and how their differences affect the bodies of functions.

Burada kafalar statement ne expression ne oluyor, açıklama şöyle:
Statements are instructions that perform some action and do not return a value.
 Expressions evaluate to a resulting value.

Devam edelim:
Creating a variable and assigning a value to it with the let keyword is a statement
Statements do not return values
Bu yüzden de let ile bir var'i başka bir var ile C'deki, Ruby'deki veya Python'daki gibi bağlayamazsın.
x=y=6 rustta yok yani.

}
*/

/*
expressions, loops, statemants
Bir loop var bir de while, for, gibi döngüler var.
Loop döngüsünde koşul yoktur, sonsuza kadar döner:
fn main() {
    loop {
        println!("again!");
    }
}

bir liste içerisinde de while döngüsü ile gezebiliriz:
fn list_loop() {
    let sample_list = [1, 2, 3, 4, 5];
    let mut cnt = 0;

    while cnt < sample_list.len() {
        println!("sample list degeri : {}", sample_list[cnt]);
        cnt += 1;
    }
}

ancak rust'ın dökümanına göre, bu şekilde liste içerisinde döngü yapmak hem hataya açık (lenght'ini
bilemediğimiz için diyor ama ben sample_list.len() diyerek onu öğreniştim) hem de hızı yavaşlatan
(çünkü compiler her döngüde durumu kontorl edecekmiş, ondan da ne olacak amk) bir şey olduğunu iddia ediyor.

Neyse bu yüzden de for döngüsünü kullan diyor. Yani biz for'u daha çok severiz elbette ama o sıraladığı nedenler sikko.

for number in (1..5) { // eger (1..5).rev() yaparsam tersine çevirir.
*/


//OWNERSHIP

// ownership -- genel -- HEAP ve STACK
// None of the ownership features slow down your program while it’s running.

/*
Meseleye heap ve stack üzerinden başlayalım.
Stack ve heap verinizi RAM üzerinde tutan iki farklı yapıdır. Kısaca böyle.
Stack'ler hızlıdır, çünkü ilk giren son çıkar. Son giren de ilk çıkar. Ayrıca yeni veriler eklenmez
yani immutable'dır diyebilirsin, fixed size'dır. Bunlar onu hızlı yapan ana etkenler.
Pek de büyük değildir dolayısıyla iyi organize edilmişlerdir.

*/

/*
Heap:
Compile anında eğer bir datanın boyutu bilinmiyorsa veya değişecekse, stack yerine heap üzerinde
saklanır. Heap'lar stack'lere daha az organize edilmiş yapılardır. Eğer bu denli büyük bir veriyi RAM'e
atacaksanız, OS sizin datanız için yeterince büyük olacak bir spot bulur ve POINTER döndürür.
Bu pointer o konumun
adresidir. Bu işleme allocating on the heap denilir. Stack'e değer yüklemek allocating olarak değerlendirilmez.

Stack üzerine veri atmak allocating olarak değerlendirilmez. Çünkü pointer bilinen bir şeydir ve uzunluğu sabittir.
Pointer'ınızı stack üzerinde depolayabilirsiniz, ama gerçek dataya ulaşmak istediğinizde pointer'ı takip etmeniz gerekir.

*/

/*
Bu iki kavramı anlatırken çok güzel bir örnek verilmiş:
Diyelim ki bir restorana gittiniz. Kapıdan girdiğiniz an kaç kişi olduğunuzu garsona söylersiniz
ve o da sizler için bir yer ayarlar. Ancak Sonradan bir başka birisi geldiğinde sizin nerede oturduğunuzu
bilmesi gerekir size katılabilmesi için.

Heap'teki dataya erişmek stack'tekinden daha yavaştır, çünkü oraya erişebilmeniz için pointer'a ihtiyacınız var.

Günümüzdeki işlemciler hafızada gidecekleri yeri çabukca bulurlarsa çok hızlıdırlar.

Restoran örneğinden ilerlersek, bir restoranda her masadan sipariş isteği geliyor diyelim.
Masaların arası ne kadar birbirine yakın olursa siparişleri kümeleyerek toplamamız o kadar yakın olur.
Yani önce A masasına gidip sonra B ye sonra tekrar A'ya gitmek çok daha yavaş olacaktır.

Bu yüzden de bir işlemci eğer datalar (stack'ler yani) birbirlerine ne kadar yakınsa işini de o denli
hızlı yaparlar.

Dolayısıyla eşşek gibi datayı heap'ta ayırmak da zaman alır.
*/

/*
Kodunuz bir fonksiyon çağırdığında, argümanlar fonksiyonun içerisine geçer. Eğer heap'taki datanın adresini
tutan bir pointer'ınız varsa o da geçer. Fonksiyonun local parametreleri stack içerisine ittirilir.
Fonksiyon bittiğinde ise bu değerler stack'ten çekilir.

*/

/*
Dolayısıyla heap ve stack'larınızı sürekli takip etmeniz gerekir. Hangi data heap'ta kullanılıyor,
heap'te duplicate dataları nasıl azaltabilirim, kullanılmamışları nasıl silebilirim vs.

Bütün bu dertler ownership ile çözülüyor. Ownership mantığını anladıktan sonra heap ve stack'leri
pek düşünmek zorunda kalmayacaksın, ama ownership'i anlaman için bunları bilmen gerekliydi diyor
rust'ın kendi klavuzunda. Her neyse bakalım bu işler doğru muymuş ilerde anlayacağız.
*/

// Ownership Kuralları - I (buldukça II - III diye devam edeceğim.)
/*
Do not talk about ownership..

Rust'ta her değerin (value) bir değişkeni (variable) vardır ve bu değişkene owner denilir.
Bir anda sadece tek bir owner olabilir.
Owner scope'dan gittiği an değer yok edilir ( dropped )

*/

/*
String - Literal

Örnekler hep string üzerinden veriliyor kitapta. Ama diğer data type'lar da aynı örnekler için
kullanılabilir elbette.
Bu arada sanırım Rust'ta string meselesine biraz daha derinden bakıyorlar ama bilemiyorum Chapter 8'de
anlatılıyor.
String literal'lar esnektir, ama metin olarak kullanmak istediğimiz her şeye uygun olmayabilir. Bunun ilk sebebi
değişken olmamalarıdır. Bir diğer sebep, kodumuzu yazarken her string bilinmeyebilir. Örneğin kullanıcıdan
bir değer alacğaız.. Bunu bilemeyiz. Bu tarz durumlar için Rust'ın ikinci bir string type'ı varmış:
String .. Ne diyor lan bu diyorsan şöyle açıklayayım, bir literal var bir de String var.
String heap'ta tutuluyor, mutable olabiliyor. Literal ise hızlı, immutable bir bok.


Bu type heap üzerinde ayrılmıştır, dolayısıyla compile anında büyüklüğü bilinmeyen metinleri depolayabilir.
string literal'dan String type oluşturulabilir, şöyle:
let kelime = String::from("naber");

Buradaki :: meselesi ileriki konularda ( Bölüm 5 - Method Syntax ve Bölüm 7 -
Paths for Referring to an Item in the Module Tree bölümlerinde ) anlatılıyor. Ama biz kısaca açıklayalım
:: bir operatör. String type'ının altındaki from fonksiyonuna namespace getirmemize yarıyor.
Dolayısıyla bunu böyle string_from gibi kullanmıyoruz.

Bu tarz string'ler mutatable (değiştirilebilir) de yapılabilir:
let mut kelime = String::from("naber");
kelime.push_str(", lan");
println!("{}", kelime);

String'ler mutable olabilirken literal'lar olamaz. Aradaki fark iki data type'ın memory'i nasıl
kullandıkları ile ilintilidir.
String literal'lar hızlıdır ve hardcoded final executable'a gömülebilirler. Bunun sebebi literal'ların
immutable olmasıdır.
Maalesef program çalışırken veya compile anında boyutu değişen eşşek gibi textleri ram'e gömemeyiz.

*/

/*
Memory Allocation - I - String - Literal

String ile mutable, büyüyebilen, kocaman text'leri heap'a gömebilmemiz için allocate etmemiz gerekir.
Bu şu demektir:
- Runtime anında, memory OS tarafından talep edilmelidir.
- String'lerle işimiz bittiğinde bunu OS'a geri göndermenin bir yolunu bulmalıyız.

İlk iş zaten her dilde var. String::from dediğinde Rust'ta bunu OS'dan String için memory isteyerek
yapıyorsun.
İkinci iş ise biraz farklı.
Bir çok dilde Garbage Collector (GC) var. GC basitçe memory'de kullanılmayan objeleri takip eder bulur ve temizler.
GC olmadan kullanılmayan RAM bloklarını bulmak ve geri çağırmak (OS'a geri döndürmeden bahsediyoruz)
bizim işimizdir.
Bunu yapmak da bir mesele. Evvelden çağırırsak invalid variable oluşur unutursak da memory'i heba ederiz.
İki kere yaparsak da bug olur. Bir allocate ile bir free işlemi birbirlerini tutmalı yani günün sonunda
allocate ettiğin her objeyi free etmelisin. Bu arada ben object diyorum ama Python'da her şey objedir o
alışkanlıkla öyle diyorum, yanlış anlaşılmasın.

Şimdi bu laflar C dilinde geliştirme yapanlar için geçerli. Yani memory management için bunlara dikkat
etmek zorundalar. Rust biraz farklı bir yol izliyor.

*/

/*
Memory Allocation - II

Rust'ta memory free allocate meselesi ownership üzerinden ilerliyor demiştik. Scope'dan çıkan bir
variable otomatik olarak dropped edilir.
Bir variable scope dışına çıktığında Rust bunu düşürmek için özel bir fonksiyon çağırır adı drop.

İlginç bir not, kitaptan:
C++ 'da - hiç bilmiyorum ama kitapta yazıyor - bir objenin kullanımı bittiğinde bu deallocate işine
bazen Resource Acquisition Is Initialization (RAII) deniliyor. Drop fonksiyonu bu RAII pattern'ına aşine
olanlara tanıdık gelebilir diyolla.

RAII 'nin ne olduğunu bilmediğim için araştırdım ve onu da burada ayrı bir topic olarak
marty.rs içine yazdım: RAII NEDIR?

Bu yaklaşım Rust'ın içerisinde çok derinlemesine yayılmıştır, basit gibi görünebilir ama kod
karmaşıklaştıkça davranış da karmaşıklaşabilir, diyor kitap.

*/


/*
Multi vars

Bir string üç parçadan oluşur. ptr, len ve capacity. Bu datalar stack'te saklanır.
ptr kısmında bizim metnimiz yer alıyor ve index,value şeklinde tutulur.
len kısmında ise String'in kullandığı metnin byte olarak ne kadar kullandığı yazar.
capacity, String'in OS'dan aldığı toplam hafızadır.
Yani len / capacity verimlilik diyebilir miyiz : ) galiba.

Şimdi ilginç bir yere geliyoruz.
s1 değerini s2'ye atadığımızda String datası kopyalanıyor. Yani stack üzerindeki
pointer, lenght ve capacity kopyalanıyor. Heap'taki data kopyalanmıyor. E haliyle amk

Daha önce söylediğimiz gibi bir var scope dışına çıkınca Rust onu free eder.
Ama s1 ve s2 aynı yere eşitleniyor ve iki pointer da aynı konumu gösteriyorsa iki kere silmek işine
double free diyorlar ve bu bir bug'tır.

Hatta şöyle bir şey diyor:
Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

Rust bu problemi şöyle çözüyor:
Allocated memory'i kopyalamaya çalışmaktansa s1'in valid olmadığını düşünüyor. Böylelikle s1 scope
dışına çıktığında free etmek zorunda kalmayacak.

*/

/*
Copy - Heap Data
Sadece pointer, lenght ve capacity'i kopyalama işine shallow copy diyorlar. Bu kavramlar sadece Rust'a
özgü değil diğer dillerde de var.
Şimdi double free ile alakalı bir örnek vereceğiz burası önemli.
malloc fonksiyonu C'de bir datanın pointer'ını geri getirmekle görevli bir fonksiyon. Bu fonksiyonu
kullanarak kullanılmayan verileri RAM'den silebiliyoruz.

şöyle bir örnek verelim:
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

Bu durumda Rust ağlayacaktır. Çünkü s1'i bir String'e eşitledikten sonra s2'yi de s1'e eşitledin.
Rust shallow copy yapmaktansa ilk variable'ı invalidate yapıyor. bu harekete de move diyorlar.
Sen bu kodu çalıştırdığında hatada --value moved here  ve  ^^value borrowed here after move diyor.

String objesine implement edilen bir copy yeteneği yok çünkü !
Bu yüzden de taşıyor. Her neyse özet olarak aynı değere sahip variable'lardan sadece en son kullanılanı
(atananı) validdir, gerisi boş.

not : Az önce assembly'e kadar gittim. Tırstım.

Neyse şimdi burada Rust bir mimari kararı veriyor : Rust'ta hiçbir zaman deep copy yapılmaz.
Bundan ötürü de herhangi bir otomatik işlemi runtime performansında gereksiz yük olarak görülüyor.

*/

/*
Clone

Örnek:
fn clone_sample() {
    let s1= String::from("Hellüü");
    let s2 = s1.clone();
}

Clone'lamalarda heap data tamamen kopyalanır.
*/

/*
Copy - Stack Data
Şimdi ilginç bir durumdan bahsedelim.

let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);

Bu kod tamamen valid. Halbuki yukarıda öğrendiğimize göre x'in invalid olması gerekiyordu çünkü y
x'e eşitlendi.
Durum şu : Integer gibi size'ı bilinen type'lar derlenirken stack'te tutulur. Dolayısıyla gerçek datayı
kopyalamak çok hızlı yapılabilir.
Deep veya shallow copy'nin hiçbir farkı yoktur burada.
* Dolayısıyla clone metodunu çağırmak hiçbir şeyi farkettirmeyecektir. *

Integer gibi stack'te tutulan variable'ların Copy diye bir trait'i vardır. Bu trait built-in func gibi
bir şey anladığım. Neyse eğer bir type'ın copy trait'i varsa atandıktan sonrahala eski değeri kullanılabilir
, diyor kitap. Eğer bir type'a drop trait'i implement edilmişse Rust Copy'i kullanmamıza izin vermez.

Dolayısıyla string'ler clone'lanır integer'lara bişi yapılmaz çünkü anında stack'e kopyalanabilir diyebiliriz.

Copy trait'ine sahip bazı objeler:

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.

Tuples, if they only contain types that are also Copy.
For example, (i32, i32) is Copy, but (i32, String) is not.

*/

/*
Ownership - Functions
The semantics for passing a value to a function are similar to those for assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.

Returning values can also transfer ownership

Heap'taki bir datayı içeren variable scope'tan çıkarsa, o datanın ownership'liği başka bir variable'a
geçmemişse silinir.

Bu duruma bir örnek verelim:

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

Kodu siktir et . Önemli olan diğer işlerde mutlaka tanım yapıyorduk let ile. Burada öyle bir mesele yok,
yani heap'ta bir bok ayırtılmıyor pointer döndürülmüyor.
