use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Item {
    name: String,
    price: f64,
    category: String,
}

#[derive(Debug)]
struct CartItem {
    item: Item,
    quantity: u32,
}

struct Cart {
    items: Vec<CartItem>,
    coupon: Option<String>,
}

struct Store {
    inventory: Vec<Item>,
    coupons: HashMap<String, f64>,
    tax_rate: f64,
}

impl Store {
    fn new() -> Self {
        let mut coupons = HashMap::new();
        coupons.insert("SAVE10".to_string(), 0.10);
        coupons.insert("SAVE20".to_string(), 0.20);
        coupons.insert("HALFOFF".to_string(), 0.50);

        Store {
            inventory: vec![
                Item { name: "Apple".to_string(), price: 1.50, category: "Fruit".to_string() },
                Item { name: "Banana".to_string(), price: 0.80, category: "Fruit".to_string() },
                Item { name: "Orange".to_string(), price: 1.20, category: "Fruit".to_string() },
                Item { name: "Grapes".to_string(), price: 2.50, category: "Fruit".to_string() },
                Item { name: "Milk".to_string(), price: 2.30, category: "Dairy".to_string() },
                Item { name: "Cheese".to_string(), price: 4.50, category: "Dairy".to_string() },
                Item { name: "Yogurt".to_string(), price: 1.80, category: "Dairy".to_string() },
                Item { name: "Bread".to_string(), price: 1.90, category: "Bakery".to_string() },
                Item { name: "Bagel".to_string(), price: 1.20, category: "Bakery".to_string() },
                Item { name: "Eggs".to_string(), price: 3.20, category: "Protein".to_string() },
                Item { name: "Chicken".to_string(), price: 6.50, category: "Protein".to_string() },
                Item { name: "Salmon".to_string(), price: 8.99, category: "Protein".to_string() },
                Item { name: "Spinach".to_string(), price: 2.10, category: "Vegetables".to_string() },
                Item { name: "Carrot".to_string(), price: 0.90, category: "Vegetables".to_string() },
                Item { name: "Tomato".to_string(), price: 1.30, category: "Vegetables".to_string() },
            ],
            coupons,
            tax_rate: 0.08,
        }
    }

    fn search(&self, query: &str) -> Vec<(usize, &Item)> {
        let q = query.to_lowercase();
        self.inventory
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.name.to_lowercase().contains(&q) || item.category.to_lowercase().contains(&q)
            })
            .collect()
    }
}

impl Cart {
    fn new() -> Self {
        Cart { items: Vec::new(), coupon: None }
    }

    fn add(&mut self, item: Item, qty: u32) {
        if let Some(existing) = self.items.iter_mut().find(|c| c.item.name == item.name) {
            existing.quantity += qty;
            println!("  ✓ Updated {} (now x{})", item.name, existing.quantity);
        } else {
            println!("  ✓ Added {} x{} to cart", item.name, qty);
            self.items.push(CartItem { item, quantity: qty });
        }
    }

    fn remove(&mut self, name: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|c| c.item.name.to_lowercase() == name.to_lowercase()) {
            let removed = self.items.remove(pos);
            println!("  ✓ Removed {} from cart", removed.item.name);
            true
        } else {
            false
        }
    }

    fn update_quantity(&mut self, name: &str, qty: u32) -> bool {
        if let Some(item) = self.items.iter_mut().find(|c| c.item.name.to_lowercase() == name.to_lowercase()) {
            if qty == 0 {
                return false;
            }
            item.quantity = qty;
            println!("  ✓ Updated {} to x{}", item.item.name, qty);
            true
        } else {
            false
        }
    }

    fn subtotal(&self) -> f64 {
        self.items.iter().map(|c| c.item.price * c.quantity as f64).sum()
    }

    fn total_items(&self) -> u32 {
        self.items.iter().map(|c| c.quantity).sum()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn view(&self) {
        if self.is_empty() {
            println!("  (cart is empty)");
            return;
        }
        println!("  {:<15} {:>6} {:>8} {:>10}", "Item", "Price", "Qty", "Total");
        println!("  {}", "-".repeat(44));
        for c in &self.items {
            println!(
                "  {:<15} {:>6.2} {:>8} {:>10.2}",
                c.item.name,
                c.item.price,
                format!("x{}", c.quantity),
                c.item.price * c.quantity as f64
            );
        }
    }

    fn apply_coupon(&mut self, code: &str, store: &Store) -> bool {
        let upper = code.to_uppercase();
        if store.coupons.contains_key(&upper) {
            self.coupon = Some(upper.clone());
            let pct = store.coupons[&upper] * 100.0;
            println!("  ✓ Coupon applied: {:.0}% discount", pct);
            true
        } else {
            false
        }
    }

    fn print_bill(&self, store: &Store) {
        let subtotal = self.subtotal();
        let discount = self.coupon.as_ref().map_or(0.0, |c| store.coupons.get(c).copied().unwrap_or(0.0));
        let discount_amount = subtotal * discount;
        let after_discount = subtotal - discount_amount;
        let tax = after_discount * store.tax_rate;
        let grand_total = after_discount + tax;

        println!("\n╔══════════════════════════════════════════╗");
        println!("║              RECEIPT                     ║");
        println!("╠══════════════════════════════════════════╣");

        self.view();

        println!("  {}", "─".repeat(44));
        println!("  {:<30} {:>10.2}", "Subtotal:", subtotal);

        if discount > 0.0 {
            println!(
                "  {:<30} {:>10}",
                format!("Discount ({:.0}%):", discount * 100.0),
                format!("-${:.2}", discount_amount)
            );
        }

        println!("  {:<30} {:>10.2}", format!("Tax ({:.0}%):", store.tax_rate * 100.0), tax);
        println!("  {}", "─".repeat(44));
        println!("  {:<30} {:>10.2}", "GRAND TOTAL:", grand_total);
        println!("  {:<30} {:>10}", "Total Items:", self.total_items());

        if let Some(c) = &self.coupon {
            println!("  {:<30} {:>10}", "Coupon Used:", c);
        }

        println!("╚══════════════════════════════════════════╝");
        println!("         Thank you for shopping!");
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_menu() {
    println!("\n┌─────────────────────────────────┐");
    println!("│         GROCERY STORE           │");
    println!("├─────────────────────────────────┤");
    println!("│  [1] Browse all items           │");
    println!("│  [2] Search items               │");
    println!("│  [3] View cart                  │");
    println!("│  [4] Remove item from cart      │");
    println!("│  [5] Update item quantity       │");
    println!("│  [6] Apply coupon               │");
    println!("│  [7] Checkout & print bill      │");
    println!("│  [0] Exit                       │");
    println!("└─────────────────────────────────┘");
}

fn browse_and_add(store: &Store, cart: &mut Cart) {
    let categories: Vec<&str> = {
        let mut seen = Vec::new();
        for item in &store.inventory {
            if !seen.contains(&item.category.as_str()) {
                seen.push(item.category.as_str());
            }
        }
        seen
    };

    println!("\n  Available Items:");
    println!("  {:<4} {:<15} {:<12} {:>8}", "#", "Name", "Category", "Price");
    println!("  {}", "─".repeat(42));

    for (i, item) in store.inventory.iter().enumerate() {
        println!("  {:<4} {:<15} {:<12} {:>8.2}", i + 1, item.name, item.category, item.price);
    }

    println!("\n  Categories: {}", categories.join(", "));

    let choice_str = read_input("\n  Enter item # to add (or 0 to cancel): ");
    let choice: usize = choice_str.parse().unwrap_or(0);

    if choice == 0 || choice > store.inventory.len() {
        if choice != 0 {
            println!("  Invalid selection.");
        }
        return;
    }

    let qty_str = read_input("  Enter quantity: ");
    let qty: u32 = qty_str.parse().unwrap_or(0);

    if qty == 0 {
        println!("  Invalid quantity.");
        return;
    }

    cart.add(store.inventory[choice - 1].clone(), qty);
}

fn search_and_add(store: &Store, cart: &mut Cart) {
    let query = read_input("\n  Search for item or category: ");
    if query.is_empty() {
        return;
    }

    let results = store.search(&query);

    if results.is_empty() {
        println!("  No items found matching \"{}\".", query);
        return;
    }

    println!("\n  Search results for \"{}\":", query);
    println!("  {:<4} {:<15} {:<12} {:>8}", "#", "Name", "Category", "Price");
    println!("  {}", "─".repeat(42));

    for (orig_idx, item) in &results {
        println!("  {:<4} {:<15} {:<12} {:>8.2}", orig_idx + 1, item.name, item.category, item.price);
    }

    let choice_str = read_input("\n  Enter item # to add (or 0 to cancel): ");
    let choice: usize = choice_str.parse().unwrap_or(0);

    if choice == 0 {
        return;
    }

    let selected = results.iter().find(|(i, _)| i + 1 == choice);
    match selected {
        None => println!("  Invalid selection."),
        Some((_, item)) => {
            let qty_str = read_input("  Enter quantity: ");
            let qty: u32 = qty_str.parse().unwrap_or(0);
            if qty == 0 {
                println!("  Invalid quantity.");
            } else {
                cart.add((*item).clone(), qty);
            }
        }
    }
}

fn main() {
    let store = Store::new();
    let mut cart = Cart::new();

    println!("╔══════════════════════════════════════════╗");
    println!("║       Welcome to Grocery Store!          ║");
    println!("║   Available coupons: SAVE10, SAVE20      ║");
    println!("╚══════════════════════════════════════════╝");

    loop {
        print_menu();
        let choice = read_input("  Choose option: ");

        match choice.as_str() {
            "1" => browse_and_add(&store, &mut cart),
            "2" => search_and_add(&store, &mut cart),
            "3" => {
                println!("\n  --- Your Cart ---");
                cart.view();
                if !cart.is_empty() {
                    println!("  Subtotal: ${:.2} ({} items)", cart.subtotal(), cart.total_items());
                }
            }
            "4" => {
                if cart.is_empty() {
                    println!("  Cart is empty.");
                    continue;
                }
                cart.view();
                let name = read_input("\n  Enter item name to remove: ");
                if !cart.remove(&name) {
                    println!("  Item \"{}\" not found in cart.", name);
                }
            }
            "5" => {
                if cart.is_empty() {
                    println!("  Cart is empty.");
                    continue;
                }
                cart.view();
                let name = read_input("\n  Enter item name to update: ");
                let qty_str = read_input("  Enter new quantity (0 to remove): ");
                let qty: u32 = qty_str.parse().unwrap_or(0);
                if qty == 0 {
                    if !cart.remove(&name) {
                        println!("  Item \"{}\" not found in cart.", name);
                    }
                } else if !cart.update_quantity(&name, qty) {
                    println!("  Item \"{}\" not found in cart.", name);
                }
            }
            "6" => {
                let code = read_input("\n  Enter coupon code: ");
                if !cart.apply_coupon(&code, &store) {
                    println!("  Invalid coupon code.");
                }
            }
            "7" => {
                if cart.is_empty() {
                    println!("  Your cart is empty. Add some items first!");
                    continue;
                }
                cart.print_bill(&store);
                break;
            }
            "0" => {
                println!("\n  Goodbye!");
                break;
            }
            _ => println!("  Invalid option. Please choose 0-7."),
        }
    }
}
