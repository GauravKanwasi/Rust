use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const DB_FILE: &str = "gym_members.csv";

#[derive(Clone)]
struct Member {
    name: String,
    age: u32,
    plan: Plan,
    email: String,
    phone: String,
}

#[derive(Clone, PartialEq)]
enum Plan {
    Basic,
    Standard,
    Premium,
}

impl Plan {
    fn from_choice(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Basic),
            "2" => Some(Self::Standard),
            "3" => Some(Self::Premium),
            _ => None,
        }
    }

    fn from_str_exact(s: &str) -> Self {
        match s {
            "Standard" => Self::Standard,
            "Premium" => Self::Premium,
            _ => Self::Basic,
        }
    }
}

impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::Basic => "Basic",
            Self::Standard => "Standard",
            Self::Premium => "Premium",
        };
        write!(f, "{}", s)
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn divider() {
    println!("{}", "═".repeat(85));
}

fn header(title: &str) {
    println!();
    divider();
    println!("  {}", title);
    divider();
}

fn ok(msg: &str) {
    println!("  ✔  {}", msg);
}
fn err(msg: &str) {
    println!("  ✖  {}", msg);
}
fn info(msg: &str) {
    println!("  ·  {}", msg);
}

fn valid_name(s: &str) -> bool {
    !s.trim().is_empty() && s.chars().all(|c| c.is_alphabetic() || c == ' ')
}

fn valid_age(s: &str) -> Option<u32> {
    s.trim().parse::<u32>().ok().filter(|&a| (1..=120).contains(&a))
}

fn valid_email(s: &str) -> bool {
    s.contains('@') && s.contains('.') && s.len() >= 5
}

fn valid_phone(s: &str) -> bool {
    let d: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    (7..=15).contains(&d.len())
}

fn ask_name(label: &str) -> String {
    loop {
        let v = prompt(&format!("  {} Name : ", label));
        if valid_name(&v) {
            return v.trim().to_string();
        }
        err("Only letters and spaces allowed.");
    }
}

fn ask_age(label: &str) -> u32 {
    loop {
        let v = prompt(&format!("  {} Age  : ", label));
        if let Some(a) = valid_age(&v) {
            return a;
        }
        err("Enter a number between 1 and 120.");
    }
}

fn ask_email(label: &str) -> String {
    loop {
        let v = prompt(&format!("  {} Email: ", label));
        if valid_email(&v) {
            return v;
        }
        err("Invalid email format.");
    }
}

fn ask_phone(label: &str) -> String {
    loop {
        let v = prompt(&format!("  {} Phone: ", label));
        if valid_phone(&v) {
            return v;
        }
        err("Phone must have 7–15 digits.");
    }
}

fn ask_plan(label: &str) -> Plan {
    loop {
        println!("  {} Plan  : [1] Basic  [2] Standard  [3] Premium", label);
        let v = prompt("  Choose (1/2/3): ");
        if let Some(p) = Plan::from_choice(&v) {
            return p;
        }
        err("Enter 1, 2 or 3.");
    }
}

fn print_member(id: u32, m: &Member) {
    println!(
        "  #{:<4} │ {:<18} │ Age: {:<3} │ {:<9} │ {:<20} │ {}",
        id, m.name, m.age, m.plan.to_string(), m.email, m.phone
    );
}

fn save_data(members: &HashMap<u32, Member>) {
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(DB_FILE) {
        for (id, m) in members {
            writeln!(file, "{},{},{},{},{},{}", id, m.name, m.age, m.plan, m.email, m.phone).unwrap_or(());
        }
    }
}

fn load_data(members: &mut HashMap<u32, Member>, next_id: &mut u32) {
    if Path::new(DB_FILE).exists() {
        if let Ok(file) = File::open(DB_FILE) {
            let reader = BufReader::new(file);
            let mut max_id = 0;
            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 6 {
                    if let Ok(id) = parts[0].parse::<u32>() {
                        members.insert(
                            id,
                            Member {
                                name: parts[1].to_string(),
                                age: parts[2].parse().unwrap_or(0),
                                plan: Plan::from_str_exact(parts[3]),
                                email: parts[4].to_string(),
                                phone: parts[5].to_string(),
                            },
                        );
                        if id > max_id {
                            max_id = id;
                        }
                    }
                }
            }
            if max_id > 0 {
                *next_id = max_id + 1;
            }
        }
    }
}

fn add_member(members: &mut HashMap<u32, Member>, next_id: &mut u32) {
    header("ADD MEMBER");
    let member = Member {
        name: ask_name(""),
        age: ask_age(""),
        email: ask_email(""),
        phone: ask_phone(""),
        plan: ask_plan(""),
    };
    members.insert(*next_id, member);
    ok(&format!("Member added — ID #{}", *next_id));
    *next_id += 1;
    save_data(members);
}

fn view_members(members: &HashMap<u32, Member>) {
    header(&format!("ALL MEMBERS ({})", members.len()));
    if members.is_empty() {
        info("No members yet.");
        return;
    }
    let mut list: Vec<(&u32, &Member)> = members.iter().collect();
    list.sort_by_key(|(id, _)| *id);
    for (id, m) in list {
        print_member(**id, m);
    }
    divider();
}

fn search_members(members: &HashMap<u32, Member>) {
    if members.is_empty() {
        err("No members to search.");
        return;
    }
    header("SEARCH");
    println!("  [1] By ID   [2] By Name   [3] By Plan");
    let choice = prompt("  Choose: ");

    match choice.as_str() {
        "1" => {
            let raw = prompt("  Member ID: ");
            let id: u32 = raw.trim().parse().unwrap_or(0);
            match members.get(&id) {
                Some(m) => {
                    divider();
                    print_member(id, m);
                    divider();
                }
                None => err(&format!("No member with ID #{}.", id)),
            }
        }
        "2" => {
            let q = prompt("  Name (partial): ").to_lowercase();
            let hits: Vec<_> = members
                .iter()
                .filter(|(_, m)| m.name.to_lowercase().contains(&q))
                .collect();
            if hits.is_empty() {
                err(&format!("No match for '{}'.", q));
            } else {
                divider();
                let mut sorted = hits;
                sorted.sort_by_key(|(id, _)| *id);
                for (id, m) in sorted {
                    print_member(**id, m);
                }
                divider();
            }
        }
        "3" => {
            let plan = ask_plan("Filter by");
            let hits: Vec<_> = members.iter().filter(|(_, m)| m.plan == plan).collect();
            if hits.is_empty() {
                info(&format!("No members on {} plan.", plan));
            } else {
                divider();
                let mut sorted = hits;
                sorted.sort_by_key(|(id, _)| *id);
                for (id, m) in sorted {
                    print_member(**id, m);
                }
                divider();
            }
        }
        _ => err("Invalid option."),
    }
}

fn update_member(members: &mut HashMap<u32, Member>) {
    if members.is_empty() {
        err("No members to update.");
        return;
    }
    header("UPDATE MEMBER");

    let raw = prompt("  Member ID: ");
    let id: u32 = raw.trim().parse().unwrap_or(0);
    if !members.contains_key(&id) {
        err(&format!("No member with ID #{}.", id));
        return;
    }

    println!("  [1] Name  [2] Age  [3] Plan  [4] Email  [5] Phone");
    let field = prompt("  Field to update: ");
    let m = members.get_mut(&id).unwrap();

    match field.as_str() {
        "1" => {
            m.name = ask_name("New");
            ok("Name updated.");
        }
        "2" => {
            m.age = ask_age("New");
            ok("Age updated.");
        }
        "3" => {
            m.plan = ask_plan("New");
            ok("Plan updated.");
        }
        "4" => {
            m.email = ask_email("New");
            ok("Email updated.");
        }
        "5" => {
            m.phone = ask_phone("New");
            ok("Phone updated.");
        }
        _ => {
            err("Invalid field.");
            return;
        }
    }
    save_data(members);
}

fn remove_member(members: &mut HashMap<u32, Member>) {
    if members.is_empty() {
        err("No members to remove.");
        return;
    }
    header("REMOVE MEMBER");

    let raw = prompt("  Member ID: ");
    let id: u32 = raw.trim().parse().unwrap_or(0);

    match members.get(&id) {
        None => err(&format!("No member with ID #{}.", id)),
        Some(m) => {
            info(&format!("About to remove: {} ({})", m.name, m.plan));
            let confirm = prompt("  Confirm? (yes/no): ");
            if confirm.eq_ignore_ascii_case("yes") || confirm.eq_ignore_ascii_case("y") {
                members.remove(&id);
                ok("Member removed.");
                save_data(members);
            } else {
                info("Cancelled.");
            }
        }
    }
}

fn show_stats(members: &HashMap<u32, Member>) {
    header("STATISTICS");
    if members.is_empty() {
        info("No data yet.");
        return;
    }

    let vals: Vec<&Member> = members.values().collect();
    let total = vals.len();
    let avg = vals.iter().map(|m| m.age as f64).sum::<f64>() / total as f64;

    let basic = vals.iter().filter(|m| m.plan == Plan::Basic).count();
    let standard = vals.iter().filter(|m| m.plan == Plan::Standard).count();
    let premium = vals.iter().filter(|m| m.plan == Plan::Premium).count();

    let youngest = vals.iter().min_by_key(|m| m.age).unwrap();
    let oldest = vals.iter().max_by_key(|m| m.age).unwrap();

    println!("  Total Members : {}", total);
    println!("  Average Age   : {:.1}", avg);
    println!("  Youngest      : {} ({})", youngest.name, youngest.age);
    println!("  Oldest        : {} ({})", oldest.name, oldest.age);
    divider();

    let max_len = 25;
    let bar = |count: usize| "█".repeat(count * max_len / total.max(1));
    
    println!("  Basic    │ {:>3} │ {}", basic, bar(basic));
    println!("  Standard │ {:>3} │ {}", standard, bar(standard));
    println!("  Premium  │ {:>3} │ {}", premium, bar(premium));
    divider();
}

fn main() {
    let mut members: HashMap<u32, Member> = HashMap::new();
    let mut next_id: u32 = 1;

    load_data(&mut members, &mut next_id);

    println!("╔═════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           GYM MEMBERSHIP MANAGEMENT SYSTEM                          ║");
    println!("╚═════════════════════════════════════════════════════════════════════════════════════╝");

    loop {
        println!();
        println!("  [1] Add     [2] View    [3] Search");
        println!("  [4] Update  [5] Remove  [6] Stats    [7] Exit");
        let choice = prompt("\n  > ");

        match choice.as_str() {
            "1" => add_member(&mut members, &mut next_id),
            "2" => view_members(&members),
            "3" => search_members(&members),
            "4" => update_member(&mut members),
            "5" => remove_member(&mut members),
            "6" => show_stats(&members),
            "7" => {
                println!("\n  Goodbye!\n");
                break;
            }
            _ => err("Enter 1–7."),
        }
    }
}
