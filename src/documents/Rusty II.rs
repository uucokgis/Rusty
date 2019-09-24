// rusty gittikçe çok uzadı bazı şeyler bulmakta zorlaşıyor.
// bu yüzden de references ve borrowing bölümünden sonra ele alınacak şeyleri buraya yazdım.

/*
borrowing - I
But mutable references have one big restriction: you can have only one mutable reference to a
particular piece of data in a particular scope.
Ornek
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

*/

/*
data race and race condition
Öncelikle data race ve race condition ne demektir onu bi halledelim.
Paylaşılan bir variable'a aynı anda iki thread'in gitmesi sonucunda oluşan düzensizliğe
race condition denilir. Skolastik bir durum olduğu için de bazen olur bazen olmaz.

Özellikle data'yı okumak değil de aynı anda değiştirmelerde bu durum gözlenir. Bunun için değiştirilme
esnasında bir lock konulması gerekir.

ekşide birisinin başına böyle bir iş gelmiş ve şunları yazmış:
("...") problemin tespitinden sonra, function pointer'larını bir queue'ya atıp,
her şeyi sırasıyla graphics thread'inde execute eden bir scheduler yazdığımı hatırlıyorum ("...")

Bu tarz durumlar debug release'de değil normal release'de daha çok gerçekleşir bir de, çünkü
debug yavaştır.

Oracle'ın sayfasında data race şöyle anlatılır:

    two or more threads in a single process access the same memory location concurrently, and

    at least one of the accesses is for writing, and

    the threads are not using any exclusive locks to control their accesses to that memory.


Rust'ta bu durumlar baştan önlenir. Bu da borrow alınan bir objenin ikinci kez borrow edilememesi
ile çözülür. Rust dökümanında d.r. ve r.c. durumları hakkında şöyle yazar:

A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to
track them down at runtime; Rust prevents this problem from happening because it won’t even
compile code with data races!

Acaba bu bir yetki kısıtlandırması mıdır yoksa nihai bir çözüm mü.. Henüz bunu bilemiyorum.
*/

/*
borrowing - II

İlla bunu yapacağım derseniz rust size yeni scope açtırtıyor:
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

Yani sonuçta bir variable'ın actual value'sı sonsuz kere borrowed edilebilirken mutable olarak
sadece bir kere borrow edilebilir. Onda da eğer öncesinde actual value alınmış olsa bile yapamıyorsunuz.
Bir tanım daha yaparsak: Bir immutable referansımız varsa orada bir de mutable referansımız olamaz


*/

/* Slice
Ownership'i olmayan diğer bir veri tipi slice.

Internally, the slice data structure stores the starting position and the length of the slice,
which corresponds to ending_index minus starting_index

Şöyle bir örnek olsun:
let s = String::from("hello world");

hello olanını almak için:
let hello = &s[0..5];

sonuncu indekstekini de almak isterseniz:
let hello = &s[0..=4];

baştan yapacaksanız başını pas geçebilirsin
let slice = &s[..2];

sonuna kadar gideceksin onu da pas geçebilirsin
let slice = &s[3..];
veya
let len = s.len();
let slice = &s[3..len];

tamamı için ise:
let slice = &s[..];

String slice'larda ü,ş,ç gibi harfler kabul edilmez. Orjinal not şurada:
String slice range indices must occur at valid UTF-8 character boundaries
peki nerede anlatıyon lan diğerlerin diye sorarsan:
a more thorough discussion of UTF-8 handling is in the “Storing UTF-8 Encoded Text with Strings”
section of Chapter 8.
*/

/*
String literals are slice
If we have a string slice, we can pass that directly. If we have a String,
we can pass a slice of the entire String

Bir fonksiyonu String yapmaktansa string slice ( &str) yapmak fonksiyonumuzu daha genel, faydalı
yapar ve herhangi bir fonksiyon da kaybettirmez.

*/

/* Struct

Structlar classın property'si gibidir. instance'i alınabilir ve değerleri mutable'dır.
Ama hepsi mutable'dır seçtiğimiz bazı fieldlar değil.

başka bir struct'tan instance yapıldıysa o değerler yeni instance'a borrow edilir,
return etmediğin de kullanamazsın.

*/

/*
Tuple structs
kitapta:
Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a
different type than other tuples, and naming each field as in a regular struct would be verbose
or redundant.
denilmiş. Sikko bir açıklama, tuple struct'ların hangi derde deva olduğu ne siki çözdüğü bir bok
belli değil. Sırf kullanacağım tuple'a isim vermek için niye böyle bir şey uydurayım. Zaten tuple
bir variable'a eşitlenmeyecek mi amk:
let (x, y, z) = tup;  // unzipping (destructuring)
al işte tup diye bir var'a eşitledim. Geç.

*/
/*
Struct'larda çoğu zaman &str gibi referenced data kullanılmaz. Bunun sebebi struct'ların genellikle
instance'ları alınıp kullanılır. Dolayısıyla borrowed edilen bir nesnenin başka instance'larda
kullanılması sakıncalı bir şeydir.
Her neyse ama bunu yapabiliriz. Eğer lifetime parameter verilirse yapılabilir. İlerde anlatılacak (Chapter 10)
*/

/*
Bazen kendi kodunu anlamlandırmak veya debug etmek için struct'ların içerisindeki elementleri görmek
isteyebilirsin. Struct'lar print yemez :)
Tabii Rust süper bir dil olduğu için bizleri düşünerek şöyle bir şey getirmiş:
rect1 'in bir struct instance'ı olduğu düşünülürse,
println!("rect1 is {:?}", rect1);

Buradaki :? ile debug trait'i çalışıyor ve struct'ı kullanışlı bir şekilde dışarıda gösterebiliyoruz
Aksi takdirde println! makrosu display kullandığı için ve struct'a bu yemediği için yapamayacaktık.

yazıyı buraya kadar okuyup yaparsan olmadığını göreceksin. Çünkü debug moduna geçirmedin, tepeye:
#[derive(Debug)]
*/

/* Methods - I
Metotlar en iyi şöyle anlatılabilir : Class içerisinde oluşturulan fonksiyonlar örneğin Python'da
ilk parametresi self olmak üzere (şart değil ama gelenek) yazılır ya. İşte metotlar da struct içerisinde
tanımlanan ve ilk pm'i self olan fonksiyonlardır.

Metotlar tanımlanırken bağlı bulunduğu struct instance'ının ownershipliği almak istemediği ve sadece
veriyi okumak istediği, yazmak istemediği için fn samplemethod(&self) kullanırız. Buradaki ownership
methods örneğinde implement edilen Rectangle'dır.

Eğer instance'ı değiştirip içine veri yazmak isteseydik, o zaman &mut self olarak yazardık.
sadece ilk parametre olarak self'i kullanan nadirdir. Bu tekniği genelde metodun selfi başka bir şeye
dönüştürmesini istediğimizde ve orjinal instance tarafından dönüşüm sonrası çağırılmasını engellemek
için yaparız ?

Bir de şunu diyolla:
The main benefit of using methods instead of functions, in addition to using method syntax and not
having to repeat the type of self in every method’s signature, is for organization. We’ve put all
the things we can do with an instance of a type in one impl block rather than making future users
of our code search for capabilities of Rectangle in various places in the library we provide.

*/

/* Methods - II
C ve C++'da metotları çağırmak için iki yöntem var:
Eğer obje üzerindeki metodu direk çağırmak istiyorsanız : "."
Yok eğer bir pointer üzerindeki metodu obje için (to object) çağırmak istiyorsanız : " -> "
Pointer üzerindeki metodu çağırdığınızda önce onu dereference etmeniz gerekir.

Her neyse, Rust'ta -> meselesi yok, çünkü automatic referencing and dereferencing denilen bir
özelliği var ve böylelikle bu işleri tek kalemde yapıyor. Mesela şunlar aynıdır:
p1.distance(&p2);                  ile                    (&p1).distance(&p2);
Tabii Rust'ta -> işareti var ama fonksiyonun döndüreceği tipi belirtirken kullanıyoruz.

*/
/* Methods - III / Associated Functions

Impl yaparak metodları struct'a bastığımızı biliyorsun. Impl'in bir diğer özelliği de fonksiyonları (metotları)
impl bloğunun içerisinde tanımlarsak self parametresi almak zorunda değiller. Bun associated functions
diyorlar çünkü struct ile associated'lar.

*/

/*
Error Handling. For examples, see : src/error_handling
For example, if you have a type rather than an Option, your program expects to have something
rather than nothing. Your code then doesn’t have to handle two cases for the Some and None variants:
it will only have one case for definitely having a value. Code trying to pass nothing to your
function won’t even compile, so your function doesn’t have to check for that case at runtime.
Another example is using an unsigned integer type such as u32, which ensures the parameter is never
negative.

*/

/*
GENERICS
Generic'ler performansta bir maliyete sebep olmuyorlar. Rust bunu koda monomorphization islemi
uygulayarak yapıyor (derleme zamanında)

*/

/*
TRAITS
Interface'lere benzerler.

*/


/*
TESTS
. The attribute cfg stands for configuration and tells Rust that the following item should only be
included given a certain configuration option
*/

/*
Closures
Closure'lar lazy function'lardır. Çok karmaşık olabilirler özellikle traitle beraber
generic olarak kullanıldıklarında.
örnekler için:
see: src/closures

When a closure captures a value from its environment, it uses memory to store the values for use in
the closure body. This use of memory is overhead that we don’t want to pay in more common cases where
we want to execute code that doesn’t capture its environment. Because functions are never allowed to
capture their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which directly map to the three ways
a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably.
These are encoded in the three Fn traits as follows:

    FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment.
    To consume the captured variables, the closure must take ownership of these variables and move them
    into the closure when it is defined. The Once part of the name represents the fact that the closure
    can’t take ownership of the same variables more than once, so it can be called only once.

    FnMut can change the environment because it mutably borrows values.

    Fn borrows values from the environment immutably.


Concurrency note:
If you want to force the closure to take ownership of the values it uses in the environment, you can
use the move keyword before the parameter list. This technique is mostly useful when passing a closure
to a new thread to move the data so it’s owned by the new thread.

*/

/* Crates:
- Rust'ta crate comment'ler example'lar panicler falan /// ile yazılır.
Crate hakkında bilgi veren yazılar //! ile yazılır

    crates.io

cargo publish ile crates.io'ya gönderilir.

cargo yank ile belirli bir versiyon kullanıma yasaklanabilir. Bu senin yazdığın crate bozuk
olduğunda kullanışlı bir şey:
cargo yank --vers 1.0.1

undo ile de geri alınabilir.
cargo yank --vers 1.0.1 --undo

Yank ile kod silinmez. crates.io'daki hiçbir kod silinmez aslında.
Yank ile yeni versiyonu atılan crate etkilenmez.

our token : cargo login s0kFyCAeFG1UI3xXD7GKz2x4ynTS02hi
token name : kuttamuwa




    Crates -p
Workspace'lerle çalışırken bir binary package'i çalıştırmak istediğinde -p kullan:
cargo run -p adder

