use std::io::{self, Write};

struct Subject {
    name: String,
    marks: f32,
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn grade(marks: f32) -> &'static str {
    match marks as u32 {
        90..=100 => "A+",
        80..=89  => "A",
        70..=79  => "B",
        60..=69  => "C",
        50..=59  => "D",
        _        => "F",
    }
}

fn feedback(marks: f32) -> &'static str {
    if marks >= 90.0 {
        "Excellent! Keep it up!"
    } else if marks >= 75.0 {
        "Good performance. A little more effort for excellence!"
    } else if marks >= 50.0 {
        "Needs improvement. Focus more on this subject."
    } else {
        "Poor performance. Seriously needs to improve!"
    }
}

fn print_separator(len: usize) {
    println!("{}", "-".repeat(len));
}

fn main() {
    println!("\n========================================");
    println!("       STUDENT RESULT CHECKER           ");
    println!("========================================\n");

    let name = read_line("Enter your name: ");

    let count_str = read_line("How many subjects? ");
    let count: usize = match count_str.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("Invalid number of subjects. Exiting.");
            return;
        }
    };

    let mut subjects: Vec<Subject> = Vec::new();

    for i in 1..=count {
        println!("\n--- Subject {} ---", i);
        let subject_name = read_line("  Subject name : ");
        let marks_str = read_line("  Marks (0-100): ");
        let marks: f32 = match marks_str.trim().parse() {
            Ok(m) if (0.0..=100.0).contains(&m) => m,
            _ => {
                println!("  Invalid marks. Setting to 0.");
                0.0
            }
        };
        subjects.push(Subject { name: subject_name, marks });
    }

    let total: f32 = subjects.iter().map(|s| s.marks).sum();
    let average = total / subjects.len() as f32;

    println!("\n\n========================================");
    println!("         RESULT CARD FOR {}          ", name.to_uppercase());
    println!("========================================");

    let col_width = 50;
    print_separator(col_width);
    println!("{:<20} {:>8} {:>6} {:>12}", "Subject", "Marks", "Grade", "Status");
    print_separator(col_width);

    for s in &subjects {
        let status = if s.marks >= 50.0 { "PASS" } else { "FAIL" };
        println!(
            "{:<20} {:>8.1} {:>6} {:>12}",
            s.name,
            s.marks,
            grade(s.marks),
            status
        );
    }

    print_separator(col_width);
    println!("{:<20} {:>8.1}", "Total", total);
    println!("{:<20} {:>8.1}", "Average", average);
    println!("{:<20} {:>8}", "Overall Grade", grade(average));
    print_separator(col_width);

    println!("\n📋 SUBJECT-WISE FEEDBACK:");
    println!("{}", "-".repeat(col_width));
    for s in &subjects {
        println!("  {:<18} → {}", s.name, feedback(s.marks));
    }

    println!("\n📌 OVERALL VERDICT:");
    if average >= 75.0 {
        println!("  ✅ {} has passed with distinction!", name);
    } else if average >= 50.0 {
        println!("  ✅ {} has passed.", name);
        println!("  📈 Room for improvement in weaker subjects.");
    } else {
        println!("  ❌ {} needs to improve significantly.", name);
        println!("  📚 Consider revising all subjects and seeking help.");
    }

    println!("\n========================================\n");
}
