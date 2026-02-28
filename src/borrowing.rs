pub fn run_demo() {
    println!("--- PHASE 2: BORROWING (ÖDÜNÇ ALMA) ---");

    // 4. Referans ile Ödünç Alma (&)
    let m1 = String::from("Rust");
    let len = calculate_length(&m1);
    println!("'{}' kelimesinin uzunluğu: {}", m1, len);

    // 5. Değiştirilebilir Referans (&mut)
    let mut m2 = String::from("High");
    change(&mut m2);
    println!("Değişmiş hali: {}", m2);
    println!("");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s kapsam dışına çıkar ama veriyi silmez (çünkü sahibi değil)

fn change(s: &mut String) {
    s.push_str(" Performance");
}
