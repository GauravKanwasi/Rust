use std::error::Error;
use std::fmt;

const MAX_AGE: u32 = 150;

/// Everything that can go wrong when creating or mutating a `User`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserError {
    EmptyName,
    InvalidAge(u32),
    InvalidEmail(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name must not be empty"),
            Self::InvalidAge(age) => write!(f, "age {age} is out of range (0..={MAX_AGE})"),
            Self::InvalidEmail(email) => write!(f, "'{email}' is not a valid email address"),
        }
    }
}

impl Error for UserError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    name: String,
    age: u32,
    email: String,
}

impl User {
    /// Creates a validated user. Name and email are trimmed.
    pub fn new(
        name: impl Into<String>,
        age: u32,
        email: impl Into<String>,
    ) -> Result<Self, UserError> {
        let name = name.into().trim().to_owned();
        if name.is_empty() {
            return Err(UserError::EmptyName);
        }
        if age > MAX_AGE {
            return Err(UserError::InvalidAge(age));
        }
        let email = Self::validate_email(email.into())?;

        Ok(Self { name, age, email })
    }

    fn validate_email(email: String) -> Result<String, UserError> {
        let email = email.trim().to_owned();

        // Deliberately lightweight check; use a dedicated crate for strict validation.
        let valid = email
            .split_once('@')
            .is_some_and(|(local, domain)| {
                !local.is_empty()
                    && !domain.contains('@')
                    && domain.contains('.')
                    && !domain.starts_with('.')
                    && !domain.ends_with('.')
            })
            && !email.contains(char::is_whitespace);

        if valid {
            Ok(email)
        } else {
            Err(UserError::InvalidEmail(email))
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn birthday(&mut self) -> Result<(), UserError> {
        if self.age >= MAX_AGE {
            return Err(UserError::InvalidAge(self.age + 1));
        }
        self.age += 1;
        Ok(())
    }

    pub fn update_email(&mut self, new_email: impl Into<String>) -> Result<(), UserError> {
        self.email = Self::validate_email(new_email.into())?;
        Ok(())
    }

    /// Consumes the user and returns the owned name.
    pub fn into_name(self) -> String {
        self.name
    }
}

/// Short form: `User(Gaurav, 21)`. Use `{:#}` for the full form.
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Name: {}, Age: {}, Email: {}", self.name, self.age, self.email)
        } else {
            write!(f, "User({}, {})", self.name, self.age)
        }
    }
}

pub fn greet(user: &User) -> String {
    format!("Hello, {}! You are {} years old.", user.name(), user.age())
}

/// Returns the older user; on a tie, returns `a`.
pub fn oldest<'a>(a: &'a User, b: &'a User) -> &'a User {
    if a.age() >= b.age() { a } else { b }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut user = User::new("Gaurav", 21, "gaurav@example.com")?;

    println!("{user:#}");
    println!("{}", greet(&user));

    user.birthday()?;
    user.update_email("gaurav.new@example.com")?;
    println!("After update: {user:#}");

    let user2 = User::new("Priya", 25, "priya@example.com")?;
    println!("Older user: {}", oldest(&user, &user2));

    let cloned = user.clone();
    assert_eq!(cloned, user);
    println!("Clone is equal to original: {cloned}");

    // Invalid input is now an error instead of silently accepted.
    if let Err(e) = User::new("", 30, "nobody@example.com") {
        println!("Rejected: {e}");
    }

    let name = user.into_name(); // `user` is moved here
    println!("Extracted name (ownership moved): {name}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_name() {
        assert_eq!(User::new("  ", 20, "a@b.com"), Err(UserError::EmptyName));
    }

    #[test]
    fn rejects_bad_email() {
        assert!(matches!(
            User::new("A", 20, "not-an-email"),
            Err(UserError::InvalidEmail(_))
        ));
    }

    #[test]
    fn rejects_unrealistic_age() {
        assert_eq!(
            User::new("A", 200, "a@b.com"),
            Err(UserError::InvalidAge(200))
        );
    }

    #[test]
    fn birthday_increments_age() {
        let mut u = User::new("A", 20, "a@b.com").unwrap();
        u.birthday().unwrap();
        assert_eq!(u.age(), 21);
    }

    #[test]
    fn oldest_prefers_first_on_tie() {
        let a = User::new("A", 30, "a@b.com").unwrap();
        let b = User::new("B", 30, "b@b.com").unwrap();
        assert_eq!(oldest(&a, &b).name(), "A");
    }
}
