use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FuelType {
    Diesel,
    MarineDiesel,
    Petrol,
    JetFuel,
}

impl fmt::Display for FuelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FuelType::Diesel => "Diesel",
            FuelType::MarineDiesel => "Marine Diesel",
            FuelType::Petrol => "Petrol",
            FuelType::JetFuel => "Jet Fuel",
        })
    }
}

#[derive(Debug, PartialEq)]
enum TransportError {
    EmptyName,
    InvalidSpeed(f64),
    InvalidAltitude(f64),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportError::EmptyName => write!(f, "name must not be empty"),
            TransportError::InvalidSpeed(s) => write!(f, "speed must be positive and finite, got {s}"),
            TransportError::InvalidAltitude(a) => write!(f, "altitude must be non-negative and finite, got {a}"),
        }
    }
}

impl Error for TransportError {}

fn check_name(name: &str) -> Result<String, TransportError> {
    let name = name.trim();
    if name.is_empty() {
        Err(TransportError::EmptyName)
    } else {
        Ok(name.to_string())
    }
}

fn check_speed(speed: f64) -> Result<f64, TransportError> {
    if speed.is_finite() && speed > 0.0 {
        Ok(speed)
    } else {
        Err(TransportError::InvalidSpeed(speed))
    }
}

trait Transport {
    fn name(&self) -> &str;
    fn fuel_type(&self) -> FuelType;
    fn max_speed(&self) -> f64;
    fn describe_motion(&self) -> String;
    fn stop(&self) -> String;

    fn emergency_stop(&self) -> String {
        format!("Emergency stop activated! {}", self.stop())
    }

    fn status(&self) -> String {
        format!(
            "{:<14} | {:>7.1} km/h | {}",
            self.name(),
            self.max_speed(),
            self.fuel_type()
        )
    }
}

struct Truck {
    name: String,
    speed: f64,
}

struct Car {
    model: String,
    horsepower: u32,
    speed: f64,
}

struct Boat {
    name: String,
    hull_type: String,
    speed: f64,
}

struct Aircraft {
    airline: String,
    altitude: f64,
    speed: f64,
}

impl Truck {
    fn new(name: &str, speed: f64) -> Result<Self, TransportError> {
        Ok(Self { name: check_name(name)?, speed: check_speed(speed)? })
    }
}

impl Car {
    fn new(model: &str, horsepower: u32, speed: f64) -> Result<Self, TransportError> {
        Ok(Self { model: check_name(model)?, horsepower, speed: check_speed(speed)? })
    }
}

impl Boat {
    fn new(name: &str, hull_type: &str, speed: f64) -> Result<Self, TransportError> {
        Ok(Self {
            name: check_name(name)?,
            hull_type: check_name(hull_type)?,
            speed: check_speed(speed)?,
        })
    }
}

impl Aircraft {
    fn new(airline: &str, altitude: f64, speed: f64) -> Result<Self, TransportError> {
        if !altitude.is_finite() || altitude < 0.0 {
            return Err(TransportError::InvalidAltitude(altitude));
        }
        Ok(Self { airline: check_name(airline)?, altitude, speed: check_speed(speed)? })
    }
}

impl Transport for Truck {
    fn name(&self) -> &str { &self.name }
    fn fuel_type(&self) -> FuelType { FuelType::Diesel }
    fn max_speed(&self) -> f64 { self.speed }

    fn describe_motion(&self) -> String {
        format!("{} moves steadily forward at {:.1} km/h.", self.name, self.speed)
    }

    fn stop(&self) -> String {
        format!("{} applies brakes and halts.", self.name)
    }
}

impl Transport for Car {
    fn name(&self) -> &str { &self.model }
    fn fuel_type(&self) -> FuelType { FuelType::Petrol }
    fn max_speed(&self) -> f64 { self.speed }

    fn describe_motion(&self) -> String {
        format!("{} roars down the road with {}hp at {:.1} km/h.", self.model, self.horsepower, self.speed)
    }

    fn stop(&self) -> String {
        format!("{} engages ABS brakes and stops.", self.model)
    }
}

impl Transport for Boat {
    fn name(&self) -> &str { &self.name }
    fn fuel_type(&self) -> FuelType { FuelType::MarineDiesel }
    fn max_speed(&self) -> f64 { self.speed }

    fn describe_motion(&self) -> String {
        format!("{} ({} hull) glides across the water at {:.1} km/h.", self.name, self.hull_type, self.speed)
    }

    fn stop(&self) -> String {
        format!("{} drops anchor and drifts to a stop.", self.name)
    }

    fn emergency_stop(&self) -> String {
        format!("Mayday! {} initiating emergency halt! {}", self.name, self.stop())
    }
}

impl Transport for Aircraft {
    fn name(&self) -> &str { &self.airline }
    fn fuel_type(&self) -> FuelType { FuelType::JetFuel }
    fn max_speed(&self) -> f64 { self.speed }

    fn describe_motion(&self) -> String {
        format!("{} soars at {:.0} m altitude, cruising at {:.1} km/h.", self.airline, self.altitude, self.speed)
    }

    fn stop(&self) -> String {
        format!("{} begins descent and landing sequence.", self.airline)
    }

    fn emergency_stop(&self) -> String {
        format!("{} declaring emergency, initiating rapid descent! {}", self.airline, self.stop())
    }
}

#[derive(Default)]
struct Fleet {
    units: Vec<Box<dyn Transport>>,
}

impl Fleet {
    const WIDTH: usize = 62;

    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, unit: impl Transport + 'static) -> &mut Self {
        self.units.push(Box::new(unit));
        self
    }

    fn fastest(&self) -> Option<&dyn Transport> {
        self.units
            .iter()
            .max_by(|a, b| a.max_speed().total_cmp(&b.max_speed()))
            .map(|t| t.as_ref())
    }

    fn dispatch(&self) {
        let line = "=".repeat(Self::WIDTH);
        println!("\n{line}\n  FLEET DISPATCH\n{line}");
        for unit in &self.units {
            println!("{}", unit.status());
            println!("   >> {}", unit.describe_motion());
        }
        println!("{line}");
    }

    fn stop_all(&self, emergency: bool) {
        for unit in &self.units {
            let message = if emergency { unit.emergency_stop() } else { unit.stop() };
            println!("   {message}");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut fleet = Fleet::new();
    fleet
        .add(Truck::new("Heavy Hauler", 90.0)?)
        .add(Car::new("Ferrari Roma", 612, 320.0)?)
        .add(Boat::new("Sea Phantom", "Catamaran", 75.0)?)
        .add(Aircraft::new("Airbus A380", 10_500.0, 903.0)?);

    fleet.dispatch();

    if let Some(unit) = fleet.fastest() {
        println!("\n  Fastest in fleet: {} ({:.1} km/h)", unit.name(), unit.max_speed());
    }

    println!("\n--- Normal Stop ---");
    fleet.stop_all(false);

    println!("\n--- Emergency Stop ---");
    fleet.stop_all(true);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_input() {
        assert_eq!(Truck::new("  ", 50.0).err(), Some(TransportError::EmptyName));
        assert_eq!(Car::new("Mini", 100, -1.0).err(), Some(TransportError::InvalidSpeed(-1.0)));
        assert!(Car::new("Mini", 100, f64::NAN).is_err());
        assert_eq!(
            Aircraft::new("Jet", -5.0, 800.0).err(),
            Some(TransportError::InvalidAltitude(-5.0))
        );
    }

    #[test]
    fn finds_fastest() {
        let mut fleet = Fleet::new();
        fleet
            .add(Truck::new("Slow", 50.0).unwrap())
            .add(Aircraft::new("Fast", 9000.0, 900.0).unwrap());
        assert_eq!(fleet.fastest().unwrap().name(), "Fast");
        assert!(Fleet::new().fastest().is_none());
    }

    #[test]
    fn emergency_stop_overrides() {
        let boat = Boat::new("Sea Phantom", "Catamaran", 75.0).unwrap();
        let truck = Truck::new("Hauler", 90.0).unwrap();
        assert!(boat.emergency_stop().starts_with("Mayday!"));
        assert!(truck.emergency_stop().starts_with("Emergency stop activated!"));
    }
}
