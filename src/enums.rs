pub fn run_demo() {
    println!("--- PHASE 4: ENUMS & PATTERN MATCHING ---");

    // 8. Basit Enum Kullanımı
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6; // _ (alt çizgi) Rust'ın kullanılmayan değişken uyarısını engeller

    // 9. Veri Taşıyan Enumlar (Rust'ın süper gücü)
    // Enum varyantları farklı tiplerde ve miktarlarda veri taşıyabilir.
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 10. Pattern Matching (match - Desen Eşleme)
    // Match, seni her ihtimali değerlendirmeye zorlar.
    print_ip(home);
    print_ip(loopback);

    // --- OPTION VE MATCH DERİNLEMESİ ---

    // Bir kutu düşün (Option). İçinde hediye olabilir (Some) ya da boş olabilir (None).
    let hediye_kutusu = Some("Akıllı Telefon");
    let bos_kutu: Option<&str> = None;

    println!("Hediye kutusu kontrol ediliyor...");
    open_box(hediye_kutusu);

    println!("Boş kutu kontrol ediliyor...");
    open_box(bos_kutu);

    println!("");

    // 11. Option Enumu (Hataları güvenli yönetmek için)
    // Rust'ta 'null' yoktur. Bunun yerine Option<T> kullanılır.
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("Option örnekleri: {:?}, {:?}", some_number, absent_number);

    // Option ile match kullanımı
    match some_number {
        Some(i) => println!("Bir sayımız var: {}", i),
        None => println!("Hiçbir şey yok!"),
    }
    println!("");
}

// match yapısı ile 'Option' kutusunu güvenle açıyoruz
fn open_box(kutu: Option<&str>) {
    match kutu {
        Some(hediye) => {
            // Eğer kutu doluysa (Some), içindeki veriyi 'hediye' ismine bağla (bind)
            println!("Yaşasın! Kutudan '{}' çıktı.", hediye);
        }
        None => {
            // Eğer kutu boşsa (None), ne yapacağımızı burada belirliyoruz
            println!("Maalesef kutu boş çıktı...");
        }
    }
}

// Basit bir enum - Bu değerlerden sadece biri olabilir
enum IpAddrKind {
    V4,
    V6,
}

// Doğrudan veri taşıyan enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum varyantlarını işlemek için 'match' kullanan fonksiyon
fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 Adresi: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(s) => {
            println!("IPv6 Adresi: {}", s);
        }
    }
}
