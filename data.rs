use std::fmt;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
enum DbError {
    UserNotFound(u32),
    DuplicateId(u32),
    DuplicateEmail(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::UserNotFound(id) => write!(f, "User with ID {} not found", id),
            DbError::DuplicateId(id) => write!(f, "User with ID {} already exists", id),
            DbError::DuplicateEmail(email) => write!(f, "Email '{}' already in use", email),
        }
    }
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }

    fn add_user(&mut self, user: User) -> Result<(), DbError> {
        if self.users.iter().any(|u| u.id == user.id) {
            return Err(DbError::DuplicateId(user.id));
        }
        if self.users.iter().any(|u| u.email == user.email) {
            return Err(DbError::DuplicateEmail(user.email.clone()));
        }
        self.users.push(user);
        Ok(())
    }

    fn get_user(&self, id: u32) -> Result<&User, DbError> {
        self.users
            .iter()
            .find(|u| u.id == id)
            .ok_or(DbError::UserNotFound(id))
    }

    fn update_user(
        &mut self,
        id: u32,
        name: Option<String>,
        email: Option<String>,
        active: Option<bool>,
    ) -> Result<(), DbError> {
        let user = self
            .users
            .iter_mut()
            .find(|u| u.id == id)
            .ok_or(DbError::UserNotFound(id))?;

        if let Some(n) = name   { user.name = n; }
        if let Some(e) = email  { user.email = e; }
        if let Some(a) = active { user.active = a; }
        Ok(())
    }

    fn delete_user(&mut self, id: u32) -> Result<User, DbError> {
        let pos = self
            .users
            .iter()
            .position(|u| u.id == id)
            .ok_or(DbError::UserNotFound(id))?;
        Ok(self.users.remove(pos))
    }

    fn search_by_name(&self, query: &str) -> Vec<&User> {
        let q = query.to_lowercase();
        self.users
            .iter()
            .filter(|u| u.name.to_lowercase().contains(&q))
            .collect()
    }

    fn list_active(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.active).collect()
    }

    fn list_users(&self) {
        if self.users.is_empty() {
            println!("No users in database.");
            return;
        }
        println!("{:-<55}", "");
        println!("{:<6} {:<12} {:<25} {}", "ID", "Name", "Email", "Active");
        println!("{:-<55}", "");
        for u in &self.users {
            println!("{:<6} {:<12} {:<25} {}", u.id, u.name, u.email, u.active);
        }
        println!("{:-<55}", "");
        println!("Total: {} user(s)", self.users.len());
    }
}

fn print_user_list(label: &str, users: &[&User]) {
    println!("\n{}", label);
    if users.is_empty() {
        println!("  (none)");
    } else {
        for u in users {
            println!("  [{:?}]", u);
        }
    }
}

fn main() {
    let mut db = Database::new();

    let seed = vec![
        User { id: 1, name: "Alice".into(),   email: "alice@example.com".into(),   active: true  },
        User { id: 2, name: "Bob".into(),     email: "bob@example.com".into(),     active: false },
        User { id: 3, name: "Charlie".into(), email: "charlie@example.com".into(), active: true  },
    ];

    for user in seed {
        match db.add_user(user) {
            Ok(_)    => {}
            Err(e)   => eprintln!("Add error: {}", e),
        }
    }

    println!("=== Initial State ===");
    db.list_users();

    println!("\n=== Lookup ID 2 ===");
    match db.get_user(2) {
        Ok(u)  => println!("Found: {:?}", u),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Update Bob (ID 2) ===");
    match db.update_user(2, None, Some("bob.new@example.com".into()), Some(true)) {
        Ok(_)  => println!("Updated successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Duplicate ID Test ===");
    match db.add_user(User { id: 1, name: "Dup".into(), email: "dup@example.com".into(), active: true }) {
        Ok(_)  => {}
        Err(e) => eprintln!("Expected error: {}", e),
    }

    println!("\n=== Search 'al' ===");
    print_user_list("Results:", &db.search_by_name("al"));

    println!("\n=== Active Users ===");
    print_user_list("Active:", &db.list_active());

    println!("\n=== Delete ID 3 ===");
    match db.delete_user(3) {
        Ok(u)  => println!("Deleted: {:?}", u),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Final State ===");
    db.list_users();

    println!("\n=== Lookup Deleted ID 3 ===");
    match db.get_user(3) {
        Ok(u)  => println!("Found: {:?}", u),
        Err(e) => eprintln!("Expected error: {}", e),
    }
}
