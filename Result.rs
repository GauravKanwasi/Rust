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

fn read_usize(prompt: &str) -> usize {
    loop {
        let input = read_line(prompt);

        match input.parse::<usize>() {
            Ok(value) if value > 0 => return value,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

fn read_marks(prompt: &str) -> f32 {
    loop {
        let input = read_line(prompt);

        match input.parse::<f32>() {
            Ok(marks) if (0.0..=100.0).contains(&marks) => return marks,
            _ => println!("Please enter marks between 0 and 100."),
        }
    }
}

fn grade(marks: f32) -> &'static str {
    match marks {
        90.0..=100.0 => "A+",
        80.0..=89.99 => "A",
        70.0..=79.99 => "B",
        60.0..=69.99 => "C",
        50.0..=59.99 => "D",
        _ => "F",
    }
}

fn feedback(marks: f32) -> &'static str {
    match marks {
        90.0..=100.0 => "Excellent! Keep it up!",
        75.0..=89.99 => "Good performance. Aim for excellence!",
        50.0..=74.99 => "Needs improvement. Focus more on this subject.",
        _ => "Needs serious improvement. Consider seeking help.",
    }
}

fn is_passed(marks: f32) -> bool {
    marks >= 50.0
}

fn print_separator(length: usize) {
    println!("{}", "-".repeat(length));
}

fn print_header(title: &str, width: usize) {
    println!("\n{}", "=".repeat(width));
    println!("{:^width$}", title, width = width);
    println!("{}", "=".repeat(width));
}

fn main() {
    const WIDTH: usize = 65;

    print_header("STUDENT RESULT CHECKER", WIDTH);

    let name = loop {
        let name = read_line("Enter your name: ");

        if !name.is_empty() {
            break name;
        }

        println!("Name cannot be empty.");
    };

    let count = read_usize("How many subjects? ");
    let mut subjects = Vec::with_capacity(count);

    for i in 1..=count {
        println!("\n--- Subject {} ---", i);

        let subject_name = loop {
            let name = read_line("Subject name : ");

            if !name.is_empty() {
                break name;
            }

            println!("Subject name cannot be empty.");
        };

        let marks = read_marks("Marks (0-100): ");

        subjects.push(Subject {
            name: subject_name,
            marks,
        });
    }

    let total: f32 = subjects.iter().map(|s| s.marks).sum();
    let average = total / subjects.len() as f32;
    let passed = subjects.iter().all(|s| is_passed(s.marks));

    print_header(
        &format!("RESULT CARD FOR {}", name.to_uppercase()),
        WIDTH,
    );

    print_separator(WIDTH);

    println!(
        "{:<25} {:>8} {:>8} {:>10}",
        "Subject", "Marks", "Grade", "Status"
    );

    print_separator(WIDTH);

    for subject in &subjects {
        let status = if is_passed(subject.marks) {
            "PASS"
        } else {
            "FAIL"
        };

        println!(
            "{:<25} {:>8.1} {:>8} {:>10}",
            subject.name,
            subject.marks,
            grade(subject.marks),
            status
        );
    }

    print_separator(WIDTH);

    println!("{:<25} {:>8.1}", "Total", total);
    println!("{:<25} {:>8.1}", "Average", average);
    println!("{:<25} {:>8}", "Overall Grade", grade(average));

    print_separator(WIDTH);

    println!("\nSUBJECT-WISE FEEDBACK");
    print_separator(WIDTH);

    for subject in &subjects {
        println!(
            "{:<25} {}",
            subject.name,
            feedback(subject.marks)
        );
    }

    println!("\nOVERALL VERDICT");
    print_separator(WIDTH);

    if passed && average >= 75.0 {
        println!("{} passed with distinction!", name);
    } else if passed {
        println!("{} has passed.", name);
        println!("Focus on weaker subjects to improve the overall score.");
    } else {
        println!("{} has failed.", name);
        println!("Focus on subjects below 50 marks and seek additional help.");
    }

    println!("\n{}", "=".repeat(WIDTH));
    println!("{:^width$}", "END OF RESULT", width = WIDTH);
    println!("{}", "=".repeat(WIDTH));
}
