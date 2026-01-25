use std::fs::{self, OpenOptions};
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
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
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

fn load_bucket_list() -> Vec<String> {
    match fs::read_to_string(FILE_NAME) {
        Ok(contents) => contents.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    }
}

fn save_bucket_list(list: &Vec<String>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_NAME)
        .unwrap();

    for item in list {
        writeln!(file, "{}", item).unwrap();
    }
}

fn add_item(list: &mut Vec<String>) {
    print!("Enter a new bucket list item: ");
    io::stdout().flush().unwrap();

    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap();

    let item = item.trim().to_string();
    if !item.is_empty() {
        list.push(item);
        println!("Item added.");
    } else {
        println!("Empty item not added.");
    }
}

fn view_items(list: &Vec<String>) {
    if list.is_empty() {
        println!("Bucket list is empty.");
    } else {
        println!("\nYour Bucket List:");
        for (i, item) in list.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }
    }
}

fn remove_item(list: &mut Vec<String>) {
    view_items(list);

    if list.is_empty() {
        return;
    }

    print!("Enter item number to remove: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
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
