// moverwriting.rs

// Define a trait with a method
trait Mover {
    fn move_object(&self);
}

// Base struct
struct Vehicle;

// Implement the trait for Vehicle
impl Mover for Vehicle {
    fn move_object(&self) {
        println!("The vehicle moves forward.");
    }
}

// Derived-like struct
struct Car;

// Override the method for Car
impl Mover for Car {
    fn move_object(&self) {
        println!("The car drives on the road.");
    }
}

// Another derived-like struct
struct Boat;

// Override the method for Boat
impl Mover for Boat {
    fn move_object(&self) {
        println!("The boat sails on water.");
    }
}

fn main() {
    let vehicle = Vehicle;
    let car = Car;
    let boat = Boat;

    // Dynamic dispatch enables method overriding behavior
    let movers: Vec<&dyn Mover> = vec![&vehicle, &car, &boat];

    for mover in movers {
        mover.move_object();
    }
}
