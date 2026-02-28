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
