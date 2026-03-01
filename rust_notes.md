# Rust Öğrenme Notları 🦀

## 1. Ownership (Sahiplik) - Rust'ın Kalbi
Rust'ta her verinin bir **sahibi** vardır. Bu sistem, Garbage Collector olmadan hafıza güvenliğini sağlar.

- **Move (Taşıma):** Bir değişken başka bir değişkene atandığında (String gibi Heap verileri için), sahiplik yeni değişkene geçer. Eski değişken artık kullanılamaz.
- **Copy (Kopyalama):** Sayılar gibi basit tipler (Stack'te tutulanlar) kopyalanır, eski değişken hala geçerlidir.

---

## 2. Borrowing (Ödünç Alma) ve Referanslar
Veriyi tamamen devretmek yerine "ödünç" veririz.

- **`&T` (Immutable Borrow):** Veriyi sadece okumak için ödünç alma. İstediğin kadar kişi aynı anda okuyabilir.
- **`&mut T` (Mutable Borrow):** Veriyi değiştirmek için ödünç alma. SADEÇE BİR kişi aynı anda değiştirebilir.
- **Altın Kural:** Aynı anda ya birçok okuyucu (`&`) olabilir ya da tek bir yazıcı (`&mut`). İkisi aynı anda olamaz!

---

## 3. Structs (Yapılar) ve Metotlar
İlişkili verileri bir araya getirme biçimimizdir.

- **Struct:** `struct User { username: String, ... }` şeklinde tanımlanır.
- **Metotlar (impl):** Bir yapıya fonksiyon eklemek için `impl` bloğu kullanılır. İçeride veriye ulaşmak için `&self` kullanılır.

---

## 4. Modül Sistemi (mod & pub)
Kodumuz büyüdükçe parçalara ayırmamızı sağlar.

- **`mod`:** Yeni bir dosyayı ana projeye (`main.rs`) dahil eder.
- **`pub`:** Bir fonksiyonu veya yapıyı "halka açık" (public) hale getirerek diğer dosyalardan erişilmesini sağlar.

---

## 5. Enumlar ve Pattern Matching (Desen Eşleme)
Bir verinin alabileceği farklı halleri tanımlarız.

- **Enum:** `enum IpAddr { V4, V6 }` gibi. Rust'ta enumlar içlerinde veri de taşıyabilir.
- **Match:** Bir enumun tüm ihtimallerini kontrol etmemizi zorunlu kılan ve güvenli kontrol sağlayan yapıdır.
- **Option<T>:** Rust'ta `null` yoktur! Onun yerine `Some(değer)` veya `None` (boş) hallerine sahip `Option` kutusu vardır. Bu sayede "boş veri" hatalarını derleme aşamasında önleriz.

---

## 6. Koleksiyonlar (Vectors)
Dinamik listeler oluşturmamızı sağlar.

- **`Vec<T>`:** Aynı tipten verileri tutan, boyutu değişebilen listeler.
- **Erişim:** `v.get(index)` yöntemi bir `Option` döner. Bu sayede dizinin sınırları dışına çıkma hataları (`Index Out of Bounds`) engellenir.

---

## 7. Result Enumu ve Hata Yönetimi
İşlemlerin başarılı olup olmadığını yönetme biçimimizdir.

- **`Result<T, E>`:** İşlem başarılıysa `Ok(T)`, başarısızsa `Err(E)` döner.
- **Güvenlik:** Tıpkı `Option` gibi, `Result` da bizi hatayı görmezden gelmemeye zorlar. `match` ile hatayı yakalamak zorundayız.

---

## 8. Traits (Özellikler) - "Ne Yapabilir?"
Farklı veri tiplerinin ortak davranışlar sergilemesini sağlar. (Interface yapısına benzer).

- **Trait:** `pub trait Summary { fn summarize(&self) -> String; }` şeklinde davranış tanımlanır.
- **Implementation:** `impl Summary for NewsArticle { ... }` diyerek bu özelliği belirli bir yapıya kazandırırız.
- **Polimorfizm:** Farklı tiplerin aynı metodu (`summarize`) farklı şekilde çalıştırmasını sağlar.

---

## 9. Generics (Genel Tipler) - "Esnek Tipler"
Kod tekrarını önlemek için tip bağımsız fonksiyonlar yazmamızı sağlar.

- **`<T>`:** Tip parametresi. Fonksiyonun herhangi bir `T` tipi için çalışabileceğini belirtir.
- **Trait Bounds:** `fn largest<T: PartialOrd>(list: &[T])` dendiğinde, "T her şey olabilir ama mutlaka karşılaştırılabilir olmalı" kısıtlaması getirilir.

---

## 10. Closures (Kapanışlar / İsimsiz Fonksiyonlar)
Değişken gibi davranan fonksiyonlardır. `|parametre| { gövde }` sözdizimiyle yazılırlar. Dışındaki değişkenlere erişebilirler.

---

## 11. Iterators (İteratörler)
Döngü yazmak yerine veriyi "borulardan" geçirir gibi işlememizi sağlar.

- **`.iter()`**: Bir koleksiyonu işleme sırasına sokar.
- **`.map()`**: Her elemanı bir işlemden geçirip dönüştürür.
- **`.filter()`**: Şartı uymayanları eler.
- **`.collect()`**: İşlem bitmiş verileri tekrar bir listeye (Vector vb.) toplar.
- **Performans:** Rust'ta bu yapı (`Zero-cost abstractions`), düz `for` döngüsü kadar hızlıdır.
