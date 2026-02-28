// Modülleri tanımlıyoruz (Dosya isimleriyle aynı olmalı)
mod borrowing;
mod enums;
mod ownership;
mod structs;

fn main() {
    println!("=== RUST EĞİTİMİNE HOŞ GELDİN ===\n");

    // Phase 1: Ownership (Sahiplik)
    ownership::run_demo();

    // Phase 2: Borrowing (Ödünç Alma)
    borrowing::run_demo();

    // Phase 3: Structs (Yapılar)
    structs::run_demo();

    // Phase 4: Enums & Pattern Matching (Enumlar ve Desen Eşleme)
    enums::run_demo();

    println!("=== DERS SONU ===");
}
