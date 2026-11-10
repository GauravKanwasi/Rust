use std::fs;
use std::io::{self, Write};

const FILE: &str = "gym_members.csv";
const PLANS: [(&str, u32); 3] = [("Basic", 20), ("Standard", 35), ("Premium", 55)];

struct Member {
    id: u32,
    name: String,
    age: u32,
    plan: usize, // index into PLANS
}

fn ask(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).unwrap_or(0) == 0 {
        std::process::exit(0); // EOF
    }
    s.trim().to_string()
}

fn ask_name() -> String {
    loop {
        let name = ask("Name: ");
        if !name.is_empty() && !name.contains(',') {
            return name;
        }
        println!("Name can't be empty or contain commas.");
    }
}

fn ask_age() -> u32 {
    loop {
        match ask("Age: ").parse::<u32>() {
            Ok(a) if (1..=120).contains(&a) => return a,
            _ => println!("Enter an age between 1 and 120."),
        }
    }
}

fn ask_plan() -> usize {
    for (i, (name, fee)) in PLANS.iter().enumerate() {
        println!("  [{}] {} (${}/mo)", i + 1, name, fee);
    }
    loop {
        match ask("Plan: ").parse::<usize>() {
            Ok(p) if (1..=PLANS.len()).contains(&p) => return p - 1,
            _ => println!("Enter 1, 2 or 3."),
        }
    }
}

fn load() -> Vec<Member> {
    fs::read_to_string(FILE)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let p: Vec<&str> = line.split(',').collect();
            if p.len() != 4 {
                return None;
            }
            Some(Member {
                id: p[0].parse().ok()?,
                name: p[1].to_string(),
                age: p[2].parse().ok()?,
                plan: p[3].parse().ok().filter(|&x| x < PLANS.len())?,
            })
        })
        .collect()
}

fn save(members: &[Member]) {
    let text: String = members
        .iter()
        .map(|m| format!("{},{},{},{}\n", m.id, m.name, m.age, m.plan))
        .collect();
    if let Err(e) = fs::write(FILE, text) {
        println!("Could not save: {e}");
    }
}

fn show(members: &[&Member]) {
    if members.is_empty() {
        println!("No members found.");
        return;
    }
    println!("{:<4} {:<20} {:<4} {}", "ID", "Name", "Age", "Plan");
    for m in members {
        println!("{:<4} {:<20} {:<4} {}", m.id, m.name, m.age, PLANS[m.plan].0);
    }
}

fn find<'a>(members: &'a mut [Member], id_text: &str) -> Option<&'a mut Member> {
    let id = id_text.parse::<u32>().ok()?;
    members.iter_mut().find(|m| m.id == id)
}

fn main() {
    let mut members = load();

    loop {
        println!("\n[1] Add  [2] View  [3] Search  [4] Change plan  [5] Remove  [6] Exit");
        match ask("> ").as_str() {
            "1" => {
                let id = members.iter().map(|m| m.id).max().unwrap_or(0) + 1;
                let (name, age, plan) = (ask_name(), ask_age(), ask_plan());
                members.push(Member { id, name, age, plan });
                save(&members);
                println!("Added member #{id}.");
            }
            "2" => {
                show(&members.iter().collect::<Vec<_>>());
                let revenue: u32 = members.iter().map(|m| PLANS[m.plan].1).sum();
                println!("\n{} members, ${} per month", members.len(), revenue);
            }
            "3" => {
                let q = ask("Search name: ").to_lowercase();
                let hits: Vec<&Member> = members
                    .iter()
                    .filter(|m| m.name.to_lowercase().contains(&q))
                    .collect();
                show(&hits);
            }
            "4" => match find(&mut members, &ask("Member ID: ")) {
                Some(m) => {
                    m.plan = ask_plan();
                    save(&members);
                    println!("Plan updated.");
                }
                None => println!("No such member."),
            },
            "5" => match find(&mut members, &ask("Member ID: ")) {
                Some(m) => {
                    let id = m.id;
                    members.retain(|m| m.id != id);
                    save(&members);
                    println!("Removed.");
                }
                None => println!("No such member."),
            },
            "6" => break,
            _ => println!("Enter 1-6."),
        }
    }
}
