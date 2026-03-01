pub fn run_demo() {
    println!("--- PHASE 7: MODERN RUST (ITERATORS & CLOSURES) ---");

    // 16. Closures (İsimsiz Fonksiyonlar)
    // Değişkenlere atanabilen veya başka fonksiyonlara parametre olarak geçilebilen fonksiyonlardır.
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    println!("Closure sonucu (5 + 1): {}", result);

    // 17. Iterators (İteratörler)
    // Bir koleksiyon üzerinde tek tek gezmeyi ve veriyi dönüştürmeyi sağlar.
    let numbers = vec![1, 2, 3, 4, 5];

    // İteratör sihrini kullanalım:
    // filter: Belirli şartı sağlayanları seçer.
    // map: Her elemanı değiştirir.
    // collect: Sonucu yeni bir koleksiyona dönüştürür.
    let squared_even_numbers: Vec<i32> = numbers
        .iter() // 1. İteratör oluştur
        .filter(|&x| x % 2 == 0) // 2. Sadece çiftleri al (Closure kullandık!)
        .map(|&x| x * x) // 3. Karelerini al
        .collect(); // 4. Yeni bir Vector yap

    println!("Orijinal sayılar: {:?}", numbers);
    println!("Çiftlerin kareleri: {:?}", squared_even_numbers);

    // 18. Fonksiyonel Zincirleme (Chaining)
    // Rust'ta döngü yazmak yerine bu yöntemi kullanmak genellikle daha hızlı ve güvenlidir.
    let sum: i32 = numbers.iter().sum();
    println!("Sayıların toplamı: {}", sum);

    println!("");
}
