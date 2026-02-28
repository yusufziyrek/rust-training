pub fn run_demo() {
    println!("--- PHASE 1: OWNERSHIP (SAHİPLİK) ---");

    // 1. Değişken Tanımlama (Immutable - Değiştirilemez)
    let x = 5;
    println!("x'in değeri: {}", x);

    // 2. Değiştirilebilir Değişken (Mutable)
    let mut y = 10;
    println!("y'nin eski değeri: {}", y);
    y = 15;
    println!("y'nin yeni değeri: {}", y);

    // 3. Ownership (Sahiplik) - Giriş
    let s1 = String::from("Hello");
    let s2 = s1; // Sahiplik s1'den s2'ye geçti (Move)

    // println!("{}", s1); // Bu satır HATA verir, çünkü s1 artık geçerli değil.
    println!("s2: {}", s2);
    println!("");
}
