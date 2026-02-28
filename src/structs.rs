pub fn run_demo() {
    println!("--- PHASE 3: STRUCTS (YAPILAR) ---");

    // 6. Struct Tanımlama ve Kullanma
    let mut user1 = User {
        username: String::from("yusuf123"),
        email: String::from("yusuf@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Kullanıcı adı: {}", user1.username);

    // Değer değiştirme (user1 'mut' olduğu için izin verilir)
    user1.email = String::from("new_email@example.com");

    // 7. Metot Kullanımı (impl)
    user1.say_hello();
    println!("");
}

// Bir Struct (Yapı) tanımlayalım
struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

// Struct'a özel metotlar ekleyelim
impl User {
    fn say_hello(&self) {
        println!(
            "Merhaba, ben {}! Email adresim: {}",
            self.username, self.email,
        );
    }
}
