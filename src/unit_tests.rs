// Bu fonksiyonu test etmek istiyoruz
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("Merhaba {}", name)
}

// 21. Test Modülü Tanımlama
// cfg(test) -> Bu modülün sadece 'cargo test' çalıştırıldığında derlenmesini sağlar.
#[cfg(test)]
mod tests {
    // Üst modüldeki (add, greet) fonksiyonlara erişmek için 'super' kullanırız.
    use super::*;

    // 22. Basit Bir Test Yazma
    #[test]
    fn it_adds_two_numbers() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn it_greets_correctly() {
        let result = greet("Yusuf");
        assert!(result.contains("Yusuf"));
    }

    // 23. Hata Beklenen Testler
    #[test]
    #[should_panic(expected = "sıfıra bölme")]
    fn test_panic() {
        panic!("sıfıra bölme hatası!");
    }

    // Başarısız olacak bir test örneği (Yorum satırında)
    /*
    #[test]
    fn failing_test() {
        assert_eq!(add(2, 2), 5); // Bu test FAIL verecektir.
    }
    */
}

pub fn run_demo() {
    println!("--- PHASE 7: UNIT TESTS (BİRİM TESTLER) ---");
    println!("Testler kodun içinde 'tests' modülünde tanımlanmıştır.");
    println!("Testleri çalıştırmak için terminale şunu yazın: cargo test");
    println!("");
}
