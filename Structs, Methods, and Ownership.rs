#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    email: String,
}

impl User {
    fn new(name: impl Into<String>, age: u32, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            age,
            email: email.into(),
        }
    }

    fn info(&self) -> String {
        format!("Name: {}, Age: {}, Email: {}", self.name, self.age, self.email)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn update_email(&mut self, new_email: impl Into<String>) {
        self.email = new_email.into();
    }

    fn into_name(self) -> String {
        self.name
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({}, {})", self.name, self.age)
    }
}

fn greet(user: &User) -> String {
    format!("Hello, {}! You are {} years old.", user.name(), user.age())
}

fn oldest<'a>(a: &'a User, b: &'a User) -> &'a User {
    if a.age() >= b.age() { a } else { b }
}

fn main() {
    let mut user = User::new("Gaurav", 21, "gaurav@example.com");

    println!("{}", user.info());
    println!("{}", greet(&user));

    user.birthday();
    user.update_email("gaurav.new@example.com");
    println!("After update: {}", user.info());

    let user2 = User::new("Priya", 25, "priya@example.com");
    let elder = oldest(&user, &user2);
    println!("Older user: {}", elder);

    let cloned = user.clone();
    println!("Clone still accessible: {}", cloned.info());

    let name = user.into_name();
    println!("Extracted name (ownership moved): {}", name);
}
