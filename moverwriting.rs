use std::fmt;

trait Mover {
    fn move_object(&self);
    fn fuel_type(&self) -> &str;
    fn max_speed(&self) -> f64;
    fn name(&self) -> &str;

    fn status(&self) -> String {
        format!(
            "[{}] Max Speed: {:.1} km/h | Fuel: {}",
            self.name(),
            self.max_speed(),
            self.fuel_type()
        )
    }
}

trait Stoppable {
    fn stop(&self);
    fn emergency_stop(&self) {
        println!("Emergency stop activated!");
        self.stop();
    }
}

struct Vehicle {
    name: String,
    speed: f64,
}

struct Car {
    model: String,
    horsepower: u32,
    speed: f64,
}

struct Boat {
    vessel_name: String,
    hull_type: String,
    speed: f64,
}

struct Aircraft {
    airline: String,
    altitude: f64,
    speed: f64,
}

impl Vehicle {
    fn new(name: &str, speed: f64) -> Self {
        Self { name: name.to_string(), speed }
    }
}

impl Car {
    fn new(model: &str, horsepower: u32, speed: f64) -> Self {
        Self { model: model.to_string(), horsepower, speed }
    }
}

impl Boat {
    fn new(vessel_name: &str, hull_type: &str, speed: f64) -> Self {
        Self { vessel_name: vessel_name.to_string(), hull_type: hull_type.to_string(), speed }
    }
}

impl Aircraft {
    fn new(airline: &str, altitude: f64, speed: f64) -> Self {
        Self { airline: airline.to_string(), altitude, speed }
    }
}

impl Mover for Vehicle {
    fn move_object(&self) {
        println!(">> {} moves steadily forward at {:.1} km/h.", self.name, self.speed);
    }

    fn fuel_type(&self) -> &str { "Diesel" }
    fn max_speed(&self) -> f64 { self.speed }
    fn name(&self) -> &str { &self.name }
}

impl Mover for Car {
    fn move_object(&self) {
        println!(">> {} roars down the road with {}hp at {:.1} km/h.", self.model, self.horsepower, self.speed);
    }

    fn fuel_type(&self) -> &str { "Petrol" }
    fn max_speed(&self) -> f64 { self.speed }
    fn name(&self) -> &str { &self.model }
}

impl Mover for Boat {
    fn move_object(&self) {
        println!(">> {} ({} hull) glides across the water at {:.1} km/h.", self.vessel_name, self.hull_type, self.speed);
    }

    fn fuel_type(&self) -> &str { "Marine Diesel" }
    fn max_speed(&self) -> f64 { self.speed }
    fn name(&self) -> &str { &self.vessel_name }
}

impl Mover for Aircraft {
    fn move_object(&self) {
        println!(">> {} soars at {:.0}m altitude, cruising at {:.1} km/h.", self.airline, self.altitude, self.speed);
    }

    fn fuel_type(&self) -> &str { "Jet Fuel" }
    fn max_speed(&self) -> f64 { self.speed }
    fn name(&self) -> &str { &self.airline }
}

impl Stoppable for Vehicle {
    fn stop(&self) { println!("   {} applies brakes and halts.", self.name); }
}

impl Stoppable for Car {
    fn stop(&self) { println!("   {} engages ABS brakes and stops.", self.model); }
}

impl Stoppable for Boat {
    fn stop(&self) { println!("   {} drops anchor and drifts to a stop.", self.vessel_name); }
    fn emergency_stop(&self) {
        println!("   Mayday! {} initiating emergency halt!", self.vessel_name);
        self.stop();
    }
}

impl Stoppable for Aircraft {
    fn stop(&self) { println!("   {} begins descent and landing sequence.", self.airline); }
    fn emergency_stop(&self) {
        println!("   {} declaring emergency — initiating rapid descent!", self.airline);
        self.stop();
    }
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vehicle({})", self.name)
    }
}

fn dispatch_fleet(movers: &[&dyn Mover]) {
    println!("\n{}", "=".repeat(55));
    println!("  FLEET DISPATCH");
    println!("{}", "=".repeat(55));
    for mover in movers {
        println!("{}", mover.status());
        mover.move_object();
    }
    println!("{}", "=".repeat(55));
}

fn fastest(movers: &[&dyn Mover]) -> Option<&str> {
    movers.iter().max_by(|a, b| {
        a.max_speed().partial_cmp(&b.max_speed()).unwrap()
    }).map(|m| m.name())
}

fn main() {
    let vehicle  = Vehicle::new("Heavy Hauler", 90.0);
    let car      = Car::new("Ferrari Roma", 612, 320.0);
    let boat     = Boat::new("Sea Phantom", "Catamaran", 75.0);
    let aircraft = Aircraft::new("Airbus A380", 10_500.0, 903.0);

    let movers: Vec<&dyn Mover> = vec![&vehicle, &car, &boat, &aircraft];

    dispatch_fleet(&movers);

    if let Some(name) = fastest(&movers) {
        println!("\n  Fastest in fleet: {}", name);
    }

    println!("\n--- Stop Sequence ---");
    vehicle.stop();
    car.stop();
    boat.emergency_stop();
    aircraft.emergency_stop();
}
