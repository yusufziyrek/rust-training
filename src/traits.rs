pub fn run_demo() {
    println!("--- PHASE 6: TRAITS & GENERICS ---");

    // 14. Trait Uygulaması
    let article = NewsArticle {
        headline: String::from("Rust 1.90 Yayınlandı!"),
        location: String::from("İstanbul"),
        author: String::from("yusuf123"),
        content: String::from("Rust dili her geçen gün daha da güçleniyor..."),
    };

    let tweet = Tweet {
        username: String::from("rust_fan"),
        content: String::from("Traits ve Generics öğrenmek harika!"),
        reply: false,
        retweet: false,
    };

    println!("Haber özeti: {}", article.summarize());
    println!("Tweet özeti: {}", tweet.summarize());

    // 15. Generics (Genel Tipler)
    // Farklı tiplerle (i32, f64 vb.) çalışabilen ortak bir fonksiyon.
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("En büyük sayı: {}", result);

    let char_list = vec!['a', 'y', 'z', 'b'];
    let result = largest(&char_list);
    println!("En büyük karakter: {}", result);

    println!("");
}

// 1. Trait Tanımlama (Bir 'özellik' arayüzü)
pub trait Summary {
    fn summarize(&self) -> String;
}

// 2. Trait'i bir Struct'a Uygulama
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, yazan: {} ({})",
            self.headline, self.author, self.location
        )
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 3. Generics ve Trait Bounds
// 'T' herhangi bir tip olabilir, ama 'PartialOrd' (karşılaştırılabilir) olmak zorunda.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
