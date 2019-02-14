// buradaki notlar Rust ile alakalı ama onunla direk alakalı olmayan, computer science ile
// daha çok alakalı şeylerdir.

// umut ucok, 2019

/*
RAII NEDİR?
*stackexchange : https://stackoverflow.com/questions/2321511/what-is-meant-by-resource-acquisition-is-initialization-raii
C++ 'a özel bir şeymiş. Çok güçlü bir concept diyorlar. Bu yaklaşımı sonrasında Scope-Bound Resource Management
olarak isimlendirmeye yönelik bir harekat olmuşlar ama yine de pek iyi açıklamıyor.
Burada resource denildiğinde sadece memory'i kastetmiyorlar. file handles, network socket,
database handles, GDI object'ler de giriyor bu tanıma.

GDI object dediği de Graphic Device Interface. Windows'ta ekrana çizmekle yükümlü birimin adıymış.
Windows'un sayfasında Windows System Information diye bir bölüm var. Orada Handles and Objects
kategorisinde bu muhabbetler anlatılıyor. Object Catagories altında User Objects, GDI Objects ve Kernel
objects var. Neyse konuya devam edelim hoşuma giderse bunları da ayrı topicte açıklıcam.

Yani kısaca limitli sayıda sahip olduğumuz ve kontrol etmemiz gereken objelerin hepsi.
Scope-bound yaklaşımı şu demek : Bir variable'ın scope'una bağlı olan objenin kullanımı (lifetime)
kastedilir. Dolayısıyla variable scope dışında çıktığında kaynaktan yok edilir ( aha rust'ınki bu lan)

Güzel tarafı bir de şu, exception'a düştüğünüzde dahi bu mekanizma o variable'ları silebilir.
Aslında GC runtime esnasında bu pointer path'lerini 0'a düşürene kadar vırt zırt sorarken
compiler scope bazlı kontrol ediyor.. İyiymiş.

Bir diğer abi de RAII'ye şöyle bir tanım yapmış:
Bir kaynağı class'a encapsulate ediyorsun. Bu class genelde bir constructor oluyor ama şart da değil.
Kaynağa ulaşır ve onu serbest bırakır.

Aslında Garbage Collector veya RAII yaklaşımlarının haricinde kullanılan variable'ların scope'larına
bakacak class'lar oluşturabiliriz. Class'ın constructor'ları eğer variable'ların pointerlarından
oluşursa sürekli bir triggered mekanizması içerisinde bu variable'ları silebiliriz.

RAII bitiş.
*/