/*
    RUST SYNTAX (SÖZDİZİMİ) SÖZLÜĞÜ VE PRATİKLERİ
    Bu dosya, Rust'ın yazım kurallarını "parçalara ayırarak" öğretir.
*/

fn run_demo() {
    println!("--- PHASE 7.5: SYNTAX PRACTICE (SÖZDİZİMİ PRATİĞİ) ---");

    // 1. Noktalı Virgül (Expression vs Statement)
    // Rust'ta bir bloğun son satırında ';' yoksa, o değer geri döndürülür (return).
    let x = {
        let y = 5;
        y + 1 // ';' yok! x = 6 olur.
    };
    println!("x'in değeri: {}", x);

    // 2. Makrolar (!)
    // Sonunda '!' olan her şey bir MAKRODUR. Derleme aşamasında kod üretirler.
    // Fonksiyonlardan farkı: Değişken sayıda argüman alabilirler.
    println!("Ben bir makroyum!");
    let _v = vec![1, 2, 3]; // vec! bir makrodur.

    // 3. Path Seperator (::)
    // Modüllere, Enum varyantlarına veya Struct metotlarına erişmek için kullanılır.
    let _option = std::option::Option::Some(5);

    // 4. Turbo Fish ::<>
    // Bazen derleyici bir tipin ne olduğunu anlayamaz. O zaman biz söyleriz.
    let numbers = "1,2,3,4";
    let _v: Vec<i32> = numbers.split(',').map(|s| s.parse().unwrap()).collect();
    // Yukarıdakinin aynısı Turbo Fish ile:
    let _v2 = numbers
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Syntax pratikleri kodun içindeki yorumlarda detaylandırılmıştır.");
    println!("");
}

// 5. Karmaşık Bir Fonksiyon İmzasını Parçalayalım:
/*
   fn   fonksiyon_adi  <'a, T>   (parametre: &'a T)   ->   String
   --   -------------  -------   ------------------   --   ------
   (1)      (2)          (3)            (4)          (5)    (6)

   (1) Fonksiyon tanımı başlangıcı.
   (2) Fonksiyonun adı.
   (3) Genel tip (Generic) ve Yaşam Süresi (Lifetime) tanımları.
   (4) Parametre: 'a yaşam süresine sahip bir T tipi referansı.
   (5) Dönüş tipi belirteci.
   (6) Geriye dönecek verinin tipi.
*/
fn complex_syntax_example<'a, T: std::fmt::Display>(item: &'a T) -> String {
    format!("İşlenen öğe: {}", item)
}
