// File: Structs, Methods, and Ownership.rs

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    // Associated function: creates a new User
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // Method using immutable reference (borrowing)
    fn info(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }

    // Method using mutable reference
    fn birthday(&mut self) {
        self.age += 1;
    }

    // Method that takes ownership of self
    fn into_name(self) -> String {
        self.name
    }
}

fn main() {
    // Create user (ownership of strings moves into User)
    let mut user = User::new(String::from("Gaurav"), 21);

    println!("Initial info: {}", user.info());

    // Borrowing mutably to modify user
    user.birthday();
    println!("After birthday: {}", user.info());

    // Taking ownership—user can no longer be used after this
    let name = user.into_name();
    println!("Extracted name (ownership moved out): {}", name);

    // Uncommenting the next line will cause a compile error
    // println!("{:?}", user); // user was moved
}
