pub fn run_demo() {
    println!("--- PHASE 5: COLLECTIONS & ERROR HANDLING ---");

    // 12. Vectors (Vec<T>) - Dinamik Listeler
    // Vector'ler aynı tipten birden fazla veriyi bir arada tutar ve boyutları değişebilir.
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Başka bir tanımlama yöntemi (vec! makrosu)
    let mut v2 = vec![10, 20, 30];
    v2.push(40);

    println!("Vector 1: {:?}", v);
    println!("Vector 2: {:?}", v2);

    // Bir elemana erişme (Güvenli yöntem: get)
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(value) => println!("Üçüncü eleman: {}", value),
        None => println!("Böyle bir eleman yok!"),
    }

    // 13. Result Enumu (Hata Yönetimi)
    // Bir işlem başarılıysa Ok(veri), başarısızsa Err(hata) döner.
    let result = divide(10.0, 2.0);
    handle_result(result);

    let error_result = divide(10.0, 0.0);
    handle_result(error_result);

    println!("");
}

// Result dönen bir basit bölme fonksiyonu
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Sıfıra bölme hatası!"))
    } else {
        Ok(numerator / denominator)
    }
}

// Result'ı match ile işleme
fn handle_result(res: Result<f64, String>) {
    match res {
        Ok(value) => println!("İşlem başarılı, sonuç: {}", value),
        Err(e) => println!("HATA OLUŞTU: {}", e),
    }
}
