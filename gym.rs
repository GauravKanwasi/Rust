use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Clone)]
struct Member {
    name: String,
    age: u32,
    plan: String,
}

fn input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut members: HashMap<u32, Member> = HashMap::new();
    let mut next_id: u32 = 1;

    loop {
        println!("\n1. Add Member\n2. View Members\n3. Remove Member\n4. Exit");
        let choice = input("Choose: ");

        match choice.as_str() {
            "1" => {
                let name = input("Name: ");
                let age: u32 = input("Age: ").parse().unwrap_or(0);
                let plan = input("Plan: ");
                let member = Member { name, age, plan };
                members.insert(next_id, member);
                println!("Member added with ID {}", next_id);
                next_id += 1;
            }
            "2" => {
                if members.is_empty() {
                    println!("No members found");
                } else {
                    for (id, m) in &members {
                        println!("ID: {} | Name: {} | Age: {} | Plan: {}", id, m.name, m.age, m.plan);
                    }
                }
            }
            "3" => {
                let id: u32 = input("Enter ID to remove: ").parse().unwrap_or(0);
                if members.remove(&id).is_some() {
                    println!("Member removed");
                } else {
                    println!("Member not found");
                }
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
