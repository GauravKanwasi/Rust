use std::fmt::{self, Display};

trait Animal: Display {
    fn breathe(&self) {
        println!("{} takes a breath...", self.name());
    }

    fn name(&self) -> &str;
    fn legs(&self) -> u8 {
        4
    }
    fn make_sound(&self) -> String;

    fn announce(&self) {
        println!("{}", self);
        self.breathe();
        println!("{} ({} legs)", self.make_sound(), self.legs());
    }
}

#[derive(Debug, Clone)]
struct Dog {
    name: String,
}

#[derive(Debug, Clone)]
struct Cat {
    name: String,
}

#[derive(Debug, Clone)]
struct Bird {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Dog name cannot be empty".into());
        }
        Ok(Self { name: name.to_string() })
    }
}

impl Cat {
    fn new(name: &str) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Cat name cannot be empty".into());
        }
        Ok(Self { name: name.to_string() })
    }
}

impl Bird {
    fn new(name: &str) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Bird name cannot be empty".into());
        }
        Ok(Self { name: name.to_string() })
    }
}

impl Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cat({})", self.name)
    }
}

impl Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bird({})", self.name)
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_sound(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_sound(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

impl Animal for Bird {
    fn name(&self) -> &str {
        &self.name
    }
    fn legs(&self) -> u8 {
        2
    }
    fn make_sound(&self) -> String {
        format!("{} says: Tweet!", self.name)
    }
}

struct Zoo {
    animals: Vec<Box<dyn Animal>>,
}

impl Zoo {
    fn new() -> Self {
        Self { animals: Vec::new() }
    }

    fn add(&mut self, animal: Box<dyn Animal>) -> &mut Self {
        self.animals.push(animal);
        self
    }

    fn announce_all(&self) {
        for animal in &self.animals {
            animal.announce();
            println!("---");
        }
    }

    fn total_legs(&self) -> u32 {
        self.animals.iter().map(|a| a.legs() as u32).sum()
    }

    fn len(&self) -> usize {
        self.animals.len()
    }
}

fn main() -> Result<(), String> {
    let mut zoo = Zoo::new();
    zoo.add(Box::new(Dog::new("Buddy")?))
        .add(Box::new(Cat::new("Whiskers")?))
        .add(Box::new(Bird::new("Tweety")?));

    zoo.announce_all();

    println!(
        "Zoo has {} animals with {} legs total.",
        zoo.len(),
        zoo.total_legs()
    );

    let max = Dog::new("Max")?;
    max.announce();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dog_sound_includes_name() {
        let dog = Dog::new("Rex").unwrap();
        assert!(dog.make_sound().contains("Rex"));
    }

    #[test]
    fn bird_has_two_legs() {
        let bird = Bird::new("Polly").unwrap();
        assert_eq!(bird.legs(), 2);
    }

    #[test]
    fn empty_name_rejected() {
        assert!(Dog::new("").is_err());
        assert!(Cat::new("   ").is_err());
    }

    #[test]
    fn zoo_totals_legs_correctly() {
        let mut zoo = Zoo::new();
        zoo.add(Box::new(Dog::new("D").unwrap()))
            .add(Box::new(Bird::new("B").unwrap()));
        assert_eq!(zoo.total_legs(), 6);
    }
}
