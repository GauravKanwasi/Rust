// data.rs
// Sample in-memory database in Rust

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn list_users(&self) {
        println!("--- User Database ---");
        for user in &self.users {
            println!(
                "ID: {}, Name: {}, Email: {}, Active: {}",
                user.id, user.name, user.email, user.active
            );
        }
    }
}

fn main() {
    let mut db = Database::new();

    // Sample data
    db.add_user(User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
    });

    db.add_user(User {
        id: 2,
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        active: false,
    });

    db.add_user(User {
        id: 3,
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        active: true,
    });

    // Display database contents
    db.list_users();
}
