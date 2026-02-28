pub fn run_demo() {
    println!("--- PHASE 2: BORROWING ---");

    // 4. Borrowing with Reference (&)
    let m1 = String::from("Rust");
    let len = calculate_length(&m1);
    println!("Length of '{}': {}", m1, len);

    // 5. Mutable Reference (&mut)
    let mut m2 = String::from("High");
    change(&mut m2);
    println!("Changed state: {}", m2);
    println!("");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" Performance");
}
