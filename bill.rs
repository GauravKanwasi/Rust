use std::io;

#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
}

struct CartItem {
    item: Item,
    quantity: u32,
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let items = vec![
        Item { name: "Apple".to_string(), price: 1.5 },
        Item { name: "Banana".to_string(), price: 0.8 },
        Item { name: "Milk".to_string(), price: 2.3 },
        Item { name: "Bread".to_string(), price: 1.9 },
        Item { name: "Eggs".to_string(), price: 3.2 },
    ];

    let mut cart: Vec<CartItem> = Vec::new();

    loop {
        println!("\nAvailable Items:");
        for (i, item) in items.iter().enumerate() {
            println!("{}: {} - ${:.2}", i + 1, item.name, item.price);
        }
        println!("0: Finish and Print Bill");
        println!("Select item number:");

        let choice: usize = read_input().parse().unwrap_or(999);

        if choice == 0 {
            break;
        }

        if choice < 1 || choice > items.len() {
            println!("Invalid selection");
            continue;
        }

        println!("Enter quantity:");
        let qty: u32 = read_input().parse().unwrap_or(0);

        if qty == 0 {
            println!("Invalid quantity");
            continue;
        }

        let selected_item = items[choice - 1].clone();

        if let Some(existing) = cart.iter_mut().find(|c| c.item.name == selected_item.name) {
            existing.quantity += qty;
        } else {
            cart.push(CartItem {
                item: selected_item,
                quantity: qty,
            });
        }
    }

    println!("\n------ BILL ------");
    let mut grand_total = 0.0;

    for cart_item in &cart {
        let total = cart_item.item.price * cart_item.quantity as f64;
        grand_total += total;
        println!(
            "{} x{} = ${:.2}",
            cart_item.item.name,
            cart_item.quantity,
            total
        );
    }

    println!("------------------");
    println!("Total Items: {}", cart.iter().map(|c| c.quantity).sum::<u32>());
    println!("Grand Total: ${:.2}", grand_total);
}
