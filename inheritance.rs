use std::fmt::Display;

trait Animal: Display {
    fn breathe(&self) {
        println!("Taking a breath...");
    }
    fn make_sound(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cat({})", self.name)
    }
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

fn announce<T: Animal>(animal: &T) {
    println!("{}", animal);
    animal.breathe();
    println!("{}", animal.make_sound());
}

fn announce_dyn(animal: &dyn Animal) {
    println!("{}", animal);
    animal.breathe();
    println!("{}", animal.make_sound());
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".into() }),
        Box::new(Cat { name: "Whiskers".into() }),
    ];

    for a in &animals {
        announce_dyn(a.as_ref());
    }

    let dog = Dog { name: "Max".into() };
    let cat = Cat { name: "Luna".into() };

    announce(&dog);
    announce(&cat);
}
