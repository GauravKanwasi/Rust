use std::fs;
use std::io::{self, Write};

const FILE_NAME: &str = "bucketlist.txt";

fn main() {
    let mut bucket_list = load_bucket_list();

    loop {
        println!("\n--- Bucket List Menu ---");
        println!("1. Add item");
        println!("2. View items");
        println!("3. Remove item");
        println!("4. Save and exit");

        print!("Enter your choice: ");
        io::stdout().flush().ok();

        let choice = read_input();

        match choice.as_str() {
            "1" => add_item(&mut bucket_list),
            "2" => view_items(&bucket_list),
            "3" => remove_item(&mut bucket_list),
            "4" => {
                save_bucket_list(&bucket_list);
                println!("Bucket list saved. Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

fn load_bucket_list() -> Vec<String> {
    fs::read_to_string(FILE_NAME)
        .map(|s| s.lines().map(str::to_owned).collect())
        .unwrap_or_default()
}

fn save_bucket_list(list: &[String]) {
    let data = list.join("\n");
    let _ = fs::write(FILE_NAME, data);
}

fn add_item(list: &mut Vec<String>) {
    print!("Enter a new bucket list item: ");
    io::stdout().flush().ok();

    let item = read_input();
    if !item.is_empty() {
        list.push(item);
        println!("Item added.");
    } else {
        println!("Empty item not added.");
    }
}

fn view_items(list: &[String]) {
    if list.is_empty() {
        println!("Bucket list is empty.");
        return;
    }

    println!("\nYour Bucket List:");
    for (i, item) in list.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }
}

fn remove_item(list: &mut Vec<String>) {
    if list.is_empty() {
        println!("Bucket list is empty.");
        return;
    }

    view_items(list);

    print!("Enter item number to remove: ");
    io::stdout().flush().ok();

    if let Ok(index) = read_input().parse::<usize>() {
        if index > 0 && index <= list.len() {
            list.remove(index - 1);
            println!("Item removed.");
        } else {
            println!("Invalid item number.");
        }
    } else {
        println!("Invalid input.");
    }
}
