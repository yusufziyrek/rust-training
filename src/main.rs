// Modülleri tanımlıyoruz (Dosya isimleriyle aynı olmalı)
mod borrowing;
mod collections;
mod enums;
mod modern_rust;
mod traits;

fn main() {
    // Phase 1-5: Önceki dersler (Yorum satırında)
    // ownership::run_demo();
    // borrowing::run_demo();
    // structs::run_demo();
    // enums::run_demo();
    // collections::run_demo();

    // Phase 6: Traits & Generics (Özellikler ve Genel Tipler)
    // traits::run_demo();

    // Phase 7: Modern Rust (İteratörler ve Closure'lar)
    modern_rust::run_demo();
}
