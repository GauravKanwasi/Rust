use std::collections::BTreeMap;
use std::io::{self, Write};

const TAX_RATE: f64 = 0.08;

const ITEMS: [(&str, f64, &str); 15] = [
    ("Apple", 1.50, "Fruit"),
    ("Banana", 0.80, "Fruit"),
    ("Orange", 1.20, "Fruit"),
    ("Grapes", 2.50, "Fruit"),
    ("Milk", 2.30, "Dairy"),
    ("Cheese", 4.50, "Dairy"),
    ("Yogurt", 1.80, "Dairy"),
    ("Bread", 1.90, "Bakery"),
    ("Bagel", 1.20, "Bakery"),
    ("Eggs", 3.20, "Protein"),
    ("Chicken", 6.50, "Protein"),
    ("Salmon", 8.99, "Protein"),
    ("Spinach", 2.10, "Vegetables"),
    ("Carrot", 0.90, "Vegetables"),
    ("Tomato", 1.30, "Vegetables"),
];

type Cart = BTreeMap<usize, u32>;

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn number(prompt: &str) -> usize {
    input(prompt).parse().unwrap_or(0)
}

fn discount(code: &str) -> Option<f64> {
    match code.to_uppercase().as_str() {
        "SAVE10" => Some(0.10),
        "SAVE20" => Some(0.20),
        "HALFOFF" => Some(0.50),
        _ => None,
    }
}

fn show_cart(cart: &Cart) -> f64 {
    let mut subtotal = 0.0;
    for (&i, &qty) in cart {
        let line = ITEMS[i].1 * qty as f64;
        subtotal += line;
        println!("{:>2}. {:<10} x{:<3} ${:.2}", i + 1, ITEMS[i].0, qty, line);
    }
    subtotal
}

fn shop(cart: &mut Cart) {
    let query = input("Search by name or category (blank for all): ").to_lowercase();
    let matches: Vec<usize> = (0..ITEMS.len())
        .filter(|&i| {
            ITEMS[i].0.to_lowercase().contains(&query) || ITEMS[i].2.to_lowercase().contains(&query)
        })
        .collect();

    if matches.is_empty() {
        println!("No matches.");
        return;
    }

    for &i in &matches {
        let (name, price, category) = ITEMS[i];
        println!("{:>2}. {:<10} {:<11} ${:.2}", i + 1, name, category, price);
    }

    let id = number("Item # to add (0 to cancel): ");
    if id == 0 {
        return;
    }
    if !matches.contains(&(id - 1)) {
        println!("Invalid selection.");
        return;
    }

    let qty = number("Quantity: ") as u32;
    if qty == 0 {
        println!("Invalid quantity.");
        return;
    }

    *cart.entry(id - 1).or_insert(0) += qty;
    println!("Added {} x{}.", ITEMS[id - 1].0, qty);
}

fn edit(cart: &mut Cart) {
    if cart.is_empty() {
        println!("Cart is empty.");
        return;
    }
    show_cart(cart);

    let id = number("Item # to change: ");
    if id == 0 || !cart.contains_key(&(id - 1)) {
        println!("Not in cart.");
        return;
    }

    let qty = number("New quantity (0 to remove): ") as u32;
    if qty == 0 {
        cart.remove(&(id - 1));
        println!("Removed.");
    } else {
        cart.insert(id - 1, qty);
        println!("Updated.");
    }
}

fn checkout(cart: &Cart, coupon: &Option<(String, f64)>) {
    println!("\n----------- RECEIPT -----------");
    let subtotal = show_cart(cart);
    let rate = coupon.as_ref().map_or(0.0, |(_, r)| *r);
    let discounted = subtotal * (1.0 - rate);
    let tax = discounted * TAX_RATE;

    println!("-------------------------------");
    println!("Subtotal:  ${:.2}", subtotal);
    if let Some((code, r)) = coupon {
        println!("Discount ({code}, {:.0}%): -${:.2}", r * 100.0, subtotal * r);
    }
    println!("Tax:       ${:.2}", tax);
    println!("TOTAL:     ${:.2}", discounted + tax);
    println!("\nThank you for shopping!");
}

fn main() {
    let mut cart = Cart::new();
    let mut coupon: Option<(String, f64)> = None;

    println!("Welcome to the Grocery Store! Coupons: SAVE10, SAVE20");

    loop {
        println!("\n1) Shop  2) View cart  3) Edit cart  4) Coupon  5) Checkout  0) Exit");
        match input("> ").as_str() {
            "1" => shop(&mut cart),
            "2" => {
                if cart.is_empty() {
                    println!("Cart is empty.");
                } else {
                    println!("Subtotal: ${:.2}", show_cart(&cart));
                }
            }
            "3" => edit(&mut cart),
            "4" => {
                let code = input("Coupon code: ");
                match discount(&code) {
                    Some(rate) => {
                        println!("Applied {:.0}% off.", rate * 100.0);
                        coupon = Some((code.to_uppercase(), rate));
                    }
                    None => println!("Invalid coupon."),
                }
            }
            "5" => {
                if cart.is_empty() {
                    println!("Add some items first.");
                } else {
                    checkout(&cart, &coupon);
                    break;
                }
            }
            "0" => break,
            _ => println!("Please choose 0-5."),
        }
    }
}
