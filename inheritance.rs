// 1. Define a Trait (similar to an Interface or Abstract Class)
trait Animal {
    // A method with a default implementation
    fn breathe(&self) {
        println!("Taking a breath...");
    }

    // A method that must be implemented by the struct
    fn make_sound(&self) -> String;
}

// 2. Define your specific types (Structs)
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// 3. Implement the Trait for each Struct
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

// 4. Using Polymorphism via Generics or Dynamic Dispatch
fn announce_animal(animal: &dyn Animal) {
    animal.breathe();
    println!("{}", animal.make_sound());
}

fn main() {
    let my_dog = Dog { name: String::from("Buddy") };
    let my_cat = Cat { name: String::from("Whiskers") };

    println!("--- Testing Dog ---");
    announce_animal(&my_dog);

    println!("\n--- Testing Cat ---");
    announce_animal(&my_cat);
}
