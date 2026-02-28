pub fn run_demo() {
    println!("--- PHASE 3: STRUCTS ---");

    // 6. Defining and Using Structs
    let mut user1 = User {
        username: String::from("yusuf123"),
        email: String::from("yusuf@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Username: {}", user1.username);

    // Changing value (allowed because user1 is 'mut')
    user1.email = String::from("new_email@example.com");

    // 7. Using Methods (impl)
    user1.say_hello();
    println!("");
}

// Define a Struct
struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

// Add methods to the struct
impl User {
    fn say_hello(&self) {
        println!("Hello, I am {}! My email: {}", self.username, self.email);
    }
}
