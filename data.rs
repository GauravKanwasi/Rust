use std::collections::{BTreeMap, HashMap};
use std::fmt;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    name_lower: String,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u32, name: impl Into<String>, email: impl Into<String>, active: bool) -> Self {
        let name = name.into();
        let name_lower = name.to_lowercase();
        User { id, name, name_lower, email: email.into(), active }
    }
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
    users: BTreeMap<u32, User>,
    emails: HashMap<String, u32>,
}

impl Database {
    fn new() -> Self {
        Database {
            users: BTreeMap::new(),
            emails: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: User) -> Result<(), DbError> {
        if self.users.contains_key(&user.id) {
            return Err(DbError::DuplicateId(user.id));
        }
        if self.emails.contains_key(&user.email) {
            return Err(DbError::DuplicateEmail(user.email));
        }
        self.emails.insert(user.email.clone(), user.id);
        self.users.insert(user.id, user);
        Ok(())
    }

    fn get_user(&self, id: u32) -> Result<&User, DbError> {
        self.users.get(&id).ok_or(DbError::UserNotFound(id))
    }

    fn update_user(
        &mut self,
        id: u32,
        name: Option<String>,
        email: Option<String>,
        active: Option<bool>,
    ) -> Result<(), DbError> {
        let user = self.users.get_mut(&id).ok_or(DbError::UserNotFound(id))?;

        if let Some(e) = email {
            if e != user.email {
                if self.emails.contains_key(&e) {
                    return Err(DbError::DuplicateEmail(e));
                }
                self.emails.remove(&user.email);
                self.emails.insert(e.clone(), id);
                user.email = e;
            }
        }
        if let Some(n) = name {
            user.name_lower = n.to_lowercase();
            user.name = n;
        }
        if let Some(a) = active {
            user.active = a;
        }
        Ok(())
    }

    fn delete_user(&mut self, id: u32) -> Result<User, DbError> {
        let user = self.users.remove(&id).ok_or(DbError::UserNotFound(id))?;
        self.emails.remove(&user.email);
        Ok(user)
    }

    fn search_by_name<'a>(&'a self, query: &str) -> impl Iterator<Item = &'a User> + 'a {
        let q = query.to_lowercase();
        self.users.values().filter(move |u| u.name_lower.contains(&q))
    }

    fn list_active(&self) -> impl Iterator<Item = &User> {
        self.users.values().filter(|u| u.active)
    }

    fn list_users(&self) {
        if self.users.is_empty() {
            println!("No users in database.");
            return;
        }
        println!("{:-<55}", "");
        println!("{:<6} {:<12} {:<25} {}", "ID", "Name", "Email", "Active");
        println!("{:-<55}", "");
        for u in self.users.values() {
            println!("{:<6} {:<12} {:<25} {}", u.id, u.name, u.email, u.active);
        }
        println!("{:-<55}", "");
        println!("Total: {} user(s)", self.users.len());
    }
}

fn print_user_list<'a>(label: &str, users: impl Iterator<Item = &'a User>) {
    println!("\n{}", label);
    let mut any = false;
    for u in users {
        any = true;
        println!("  [{:?}]", u);
    }
    if !any {
        println!("  (none)");
    }
}

fn main() {
    let mut db = Database::new();

    let seed = [
        User::new(1, "Alice", "alice@example.com", true),
        User::new(2, "Bob", "bob@example.com", false),
        User::new(3, "Charlie", "charlie@example.com", true),
    ];

    for user in seed {
        if let Err(e) = db.add_user(user) {
            eprintln!("Add error: {}", e);
        }
    }

    println!("=== Initial State ===");
    db.list_users();

    println!("\n=== Lookup ID 2 ===");
    match db.get_user(2) {
        Ok(u) => println!("Found: {:?}", u),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Update Bob (ID 2) ===");
    match db.update_user(2, None, Some("bob.new@example.com".into()), Some(true)) {
        Ok(_) => println!("Updated successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Duplicate ID Test ===");
    if let Err(e) = db.add_user(User::new(1, "Dup", "dup@example.com", true)) {
        eprintln!("Expected error: {}", e);
    }

    println!("\n=== Search 'al' ===");
    print_user_list("Results:", db.search_by_name("al"));

    println!("\n=== Active Users ===");
    print_user_list("Active:", db.list_active());

    println!("\n=== Delete ID 3 ===");
    match db.delete_user(3) {
        Ok(u) => println!("Deleted: {:?}", u),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Final State ===");
    db.list_users();

    println!("\n=== Lookup Deleted ID 3 ===");
    match db.get_user(3) {
        Ok(u) => println!("Found: {:?}", u),
        Err(e) => eprintln!("Expected error: {}", e),
    }
}
