pub fn run_demo() {
    println!("--- PHASE 7: LIFETIMES (YAŞAM SÜRELERİ) ---");

    // 19. Lifetimes Giriş
    // Rust'ta her referansın bir 'ömrü' vardır.
    // Çoğu zaman derleyici bunu kendisi anlar (Lifetime Elision).
    // Ama bazen hangi referansın ne kadar yaşayacağını bizim söylememiz gerekir.

    let string1 = String::from("uzun bir metin");
    let string2 = "kısa";

    // longest fonksiyonu iki referans alır ve bir referans döner.
    // Dönen referansın hangisi olacağını derleyici çalışma anında bilemez.
    // Bu yüzden bu referansların ömürlerinin ortak olduğunu belirtmemiz gerekir.
    let result = longest(string1.as_str(), string2);
    println!("Daha uzun olan: {}", result);

    // 20. Lifetime Hatası Örneği (Yorumda)
    /*
    let result;
    {
        let string3 = String::from("geçici metin");
        result = longest(string1.as_str(), string3.as_str());
    } // string3 burada ölür!
    println!("{}", result); // HATA! result artık geçersiz bir yeri gösteriyor olabilir.
    */

    println!("Lifetimes mantığı: Referansların birbirine olan ömür bağlılığını tanımlar.");
    println!("");
}

// '<'a>' işareti bir Lifetime tanımıdır.
// "Gelen x ve y referansları en az 'a kadar yaşamalı, dönen sonuç da bu kadar süre geçerli kalmalı" demektir.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
