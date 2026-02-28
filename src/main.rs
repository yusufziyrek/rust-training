// Defining modules (must match file names)
mod borrowing;
mod ownership;
mod structs;

fn main() {
    println!("=== WELCOME TO RUST TRAINING ===\n");

    // Phase 1: Ownership
    ownership::run_demo();

    // Phase 2: Borrowing
    borrowing::run_demo();

    // Phase 3: Structs
    structs::run_demo();

    println!("=== END OF LESSON ===");
}
