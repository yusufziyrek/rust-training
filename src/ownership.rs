pub fn run_demo() {
    println!("--- PHASE 1: OWNERSHIP ---");

    // 1. Variable Definition (Immutable)
    let x = 5;
    println!("Value of x: {}", x);

    // 2. Mutable Variable
    let mut y = 10;
    println!("Old value of y: {}", y);
    y = 15;
    println!("New value of y: {}", y);

    // 3. Ownership - Introduction
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moved from s1 to s2

    // println!("{}", s1); // This would THROW ERROR.
    println!("s2: {}", s2);
    println!("");
}
