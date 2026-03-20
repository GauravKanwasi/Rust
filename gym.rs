use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Clone)]
struct Member {
    name: String,
    age: u32,
    plan: String,
    email: String,
    phone: String,
}

fn input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn print_separator() {
    println!("{}", "-".repeat(60));
}

fn print_member(id: u32, m: &Member) {
    print_separator();
    println!("  ID     : {}", id);
    println!("  Name   : {}", m.name);
    println!("  Age    : {}", m.age);
    println!("  Plan   : {}", m.plan);
    println!("  Email  : {}", m.email);
    println!("  Phone  : {}", m.phone);
}

fn validate_age(input: &str) -> Option<u32> {
    match input.trim().parse::<u32>() {
        Ok(age) if age > 0 && age <= 120 => Some(age),
        _ => None,
    }
}

fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() >= 5
}

fn validate_phone(phone: &str) -> bool {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.len() >= 7 && digits.len() <= 15
}

fn validate_name(name: &str) -> bool {
    !name.trim().is_empty() && name.chars().all(|c| c.is_alphabetic() || c == ' ')
}

fn add_member(members: &mut HashMap<u32, Member>, next_id: &mut u32) {
    println!("\n--- Add New Member ---");

    let name = loop {
        let n = input("Name: ");
        if validate_name(&n) {
            break n;
        }
        println!("  Invalid name. Only letters and spaces allowed.");
    };

    let age = loop {
        let raw = input("Age: ");
        if let Some(a) = validate_age(&raw) {
            break a;
        }
        println!("  Invalid age. Enter a number between 1 and 120.");
    };

    let plan = loop {
        println!("  Plans: [1] Basic  [2] Standard  [3] Premium");
        let p = input("Select plan (1/2/3): ");
        match p.as_str() {
            "1" => break "Basic".to_string(),
            "2" => break "Standard".to_string(),
            "3" => break "Premium".to_string(),
            _ => println!("  Invalid plan selection."),
        }
    };

    let email = loop {
        let e = input("Email: ");
        if validate_email(&e) {
            break e;
        }
        println!("  Invalid email format.");
    };

    let phone = loop {
        let p = input("Phone: ");
        if validate_phone(&p) {
            break p;
        }
        println!("  Invalid phone number.");
    };

    let member = Member { name, age, plan, email, phone };
    members.insert(*next_id, member);
    println!("\n  Member successfully added with ID {}.", *next_id);
    *next_id += 1;
}

fn view_members(members: &HashMap<u32, Member>) {
    if members.is_empty() {
        println!("\n  No members found.");
        return;
    }

    println!("\n--- All Members ({} total) ---", members.len());
    let mut sorted: Vec<(&u32, &Member)> = members.iter().collect();
    sorted.sort_by_key(|(id, _)| *id);
    for (id, m) in sorted {
        print_member(*id, m);
    }
    print_separator();
}

fn search_members(members: &HashMap<u32, Member>) {
    if members.is_empty() {
        println!("\n  No members to search.");
        return;
    }

    println!("\n--- Search Members ---");
    println!("  [1] Search by ID");
    println!("  [2] Search by Name");
    println!("  [3] Search by Plan");

    let choice = input("Choose search type: ");

    match choice.as_str() {
        "1" => {
            let id: u32 = input("Enter ID: ").parse().unwrap_or(0);
            match members.get(&id) {
                Some(m) => { print_member(id, m); print_separator(); }
                None => println!("  No member found with ID {}.", id),
            }
        }
        "2" => {
            let query = input("Enter name (partial match): ").to_lowercase();
            let results: Vec<_> = members.iter()
                .filter(|(_, m)| m.name.to_lowercase().contains(&query))
                .collect();
            if results.is_empty() {
                println!("  No members found matching '{}'.", query);
            } else {
                for (id, m) in results {
                    print_member(*id, m);
                }
                print_separator();
            }
        }
        "3" => {
            println!("  Plans: [1] Basic  [2] Standard  [3] Premium");
            let p = input("Select plan (1/2/3): ");
            let plan = match p.as_str() {
                "1" => "Basic",
                "2" => "Standard",
                "3" => "Premium",
                _ => { println!("  Invalid selection."); return; }
            };
            let results: Vec<_> = members.iter()
                .filter(|(_, m)| m.plan == plan)
                .collect();
            if results.is_empty() {
                println!("  No members on the {} plan.", plan);
            } else {
                println!("\n  Members on {} plan:", plan);
                for (id, m) in results {
                    print_member(*id, m);
                }
                print_separator();
            }
        }
        _ => println!("  Invalid search option."),
    }
}

fn update_member(members: &mut HashMap<u32, Member>) {
    if members.is_empty() {
        println!("\n  No members to update.");
        return;
    }

    println!("\n--- Update Member ---");
    let id: u32 = input("Enter member ID to update: ").parse().unwrap_or(0);

    if !members.contains_key(&id) {
        println!("  Member with ID {} not found.", id);
        return;
    }

    println!("  What would you like to update?");
    println!("  [1] Name  [2] Age  [3] Plan  [4] Email  [5] Phone");
    let field = input("Choose field: ");

    let member = members.get_mut(&id).unwrap();

    match field.as_str() {
        "1" => {
            let name = loop {
                let n = input("New name: ");
                if validate_name(&n) { break n; }
                println!("  Invalid name.");
            };
            member.name = name;
            println!("  Name updated.");
        }
        "2" => {
            let age = loop {
                let raw = input("New age: ");
                if let Some(a) = validate_age(&raw) { break a; }
                println!("  Invalid age.");
            };
            member.age = age;
            println!("  Age updated.");
        }
        "3" => {
            let plan = loop {
                println!("  Plans: [1] Basic  [2] Standard  [3] Premium");
                let p = input("Select plan: ");
                match p.as_str() {
                    "1" => break "Basic".to_string(),
                    "2" => break "Standard".to_string(),
                    "3" => break "Premium".to_string(),
                    _ => println!("  Invalid selection."),
                }
            };
            member.plan = plan;
            println!("  Plan updated.");
        }
        "4" => {
            let email = loop {
                let e = input("New email: ");
                if validate_email(&e) { break e; }
                println!("  Invalid email.");
            };
            member.email = email;
            println!("  Email updated.");
        }
        "5" => {
            let phone = loop {
                let p = input("New phone: ");
                if validate_phone(&p) { break p; }
                println!("  Invalid phone.");
            };
            member.phone = phone;
            println!("  Phone updated.");
        }
        _ => println!("  Invalid field selection."),
    }
}

fn remove_member(members: &mut HashMap<u32, Member>) {
    if members.is_empty() {
        println!("\n  No members to remove.");
        return;
    }

    println!("\n--- Remove Member ---");
    let id: u32 = input("Enter ID to remove: ").parse().unwrap_or(0);

    if let Some(m) = members.get(&id) {
        println!("  Removing: {} (Plan: {})", m.name, m.plan);
        let confirm = input("  Confirm removal? (yes/no): ");
        if confirm.eq_ignore_ascii_case("yes") {
            members.remove(&id);
            println!("  Member removed successfully.");
        } else {
            println!("  Removal cancelled.");
        }
    } else {
        println!("  Member with ID {} not found.", id);
    }
}

fn show_stats(members: &HashMap<u32, Member>) {
    if members.is_empty() {
        println!("\n  No members to show stats for.");
        return;
    }

    let total = members.len();
    let avg_age: f64 = members.values().map(|m| m.age as f64).sum::<f64>() / total as f64;
    let basic = members.values().filter(|m| m.plan == "Basic").count();
    let standard = members.values().filter(|m| m.plan == "Standard").count();
    let premium = members.values().filter(|m| m.plan == "Premium").count();
    let youngest = members.values().min_by_key(|m| m.age).unwrap();
    let oldest = members.values().max_by_key(|m| m.age).unwrap();

    println!("\n--- Member Statistics ---");
    print_separator();
    println!("  Total Members  : {}", total);
    println!("  Average Age    : {:.1}", avg_age);
    println!("  Youngest       : {} ({})", youngest.name, youngest.age);
    println!("  Oldest         : {} ({})", oldest.name, oldest.age);
    print_separator();
    println!("  Basic          : {} member(s)", basic);
    println!("  Standard       : {} member(s)", standard);
    println!("  Premium        : {} member(s)", premium);
    print_separator();
}

fn main() {
    let mut members: HashMap<u32, Member> = HashMap::new();
    let mut next_id: u32 = 1;

    println!("========================================");
    println!("     Gym Membership Management System   ");
    println!("========================================");

    loop {
        println!("\n--- Main Menu ---");
        println!("  [1] Add Member");
        println!("  [2] View All Members");
        println!("  [3] Search Members");
        println!("  [4] Update Member");
        println!("  [5] Remove Member");
        println!("  [6] Statistics");
        println!("  [7] Exit");

        let choice = input("\nChoose: ");

        match choice.as_str() {
            "1" => add_member(&mut members, &mut next_id),
            "2" => view_members(&members),
            "3" => search_members(&members),
            "4" => update_member(&mut members),
            "5" => remove_member(&mut members),
            "6" => show_stats(&members),
            "7" => {
                println!("\n  Goodbye!");
                break;
            }
            _ => println!("\n  Invalid choice. Please select 1-7."),
        }
    }
}
