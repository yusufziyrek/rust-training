/*
    RUST VERİ TİPLERİ (DATA TYPES)
    Rust tamamen "statik tipli" bir dildir. Derleme sırasında tüm tipler bilinmelidir.
*/

pub fn run_demo() {
    println!("--- PHASE 0: DATA TYPES (VERİ TİPLERİ) ---");

    // 1. SCALAR TİPLER (Tek bir değer tutanlar)

    // Tam Sayılar (Integers)
    // i8, i16, i32, i64, i128 (İşaretli - negatif olabilir)
    // u8, u16, u32, u64, u128 (İşaretsiz - sadece pozitif)
    let tam_sayi: i32 = -42;
    let pozitif_sayi: u32 = 100;
    println!("Sayılar: {} ve {}", tam_sayi, pozitif_sayi);

    // Ondalıklı Sayılar (Floating-Point)
    // f32 (tek hassasiyet), f64 (çift hassasiyet - varsayılan)
    let ondalikli: f64 = 3.14159;
    println!("Pi: {}", ondalikli);

    // Mantıksal Değer (Boolean)
    let dogru_mu: bool = true;
    println!("Rust harika mı? {}", dogru_mu);

    // Karakter (Char)
    // Unicode destekler, tek tırnak kullanılır.
    let harf: char = 'R';
    let emoji: char = '🦀';
    println!("Harf: {}, Emoji: {}", harf, emoji);

    // 2. COMPOUND TİPLER (Birden fazla değeri gruplayanlar)

    // Tuple (Demet)
    // Farklı tipleri bir araya getirebilir. Sabit boyuttur.
    let demet: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = demet; // Deconstruction (Parçalama)
    println!("Tuple değerleri: {}, {}, {}", x, y, z);
    println!("Tuple'dan bir eleman: {}", demet.0); // Nokta notasyonu ile erişim

    // Array (Dizi)
    // Aynı tipten veriler. Sabit boyuttur, Stack'te tutulur.
    let dizi: [i32; 3] = [10, 20, 30];
    println!("Dizinin ilk elemanı: {}", dizi[0]);

    println!("");
}
