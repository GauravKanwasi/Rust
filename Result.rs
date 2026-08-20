use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::{self, Write as IoWrite};

// ---------- ANSI colors (no external crate needed) ----------
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";

struct Subject {
    name: String,
    marks: f32,
}

impl Subject {
    fn grade(&self) -> &'static str {
        match self.marks {
            90.0..=100.0 => "A+",
            80.0..=89.99 => "A",
            70.0..=79.99 => "B",
            60.0..=69.99 => "C",
            50.0..=59.99 => "D",
            _ => "F",
        }
    }

    fn grade_point(&self) -> f32 {
        match self.marks {
            90.0..=100.0 => 4.0,
            80.0..=89.99 => 3.7,
            70.0..=79.99 => 3.3,
            60.0..=69.99 => 3.0,
            50.0..=59.99 => 2.0,
            _ => 0.0,
        }
    }

    fn feedback(&self) -> &'static str {
        match self.marks {
            90.0..=100.0 => "Excellent! Keep it up!",
            75.0..=89.99 => "Good performance. Aim for excellence!",
            50.0..=74.99 => "Needs improvement. Focus more on this subject.",
            _ => "Needs serious improvement. Consider seeking help.",
        }
    }

    fn is_passed(&self) -> bool {
        self.marks >= 50.0
    }

    fn status(&self) -> &'static str {
        if self.is_passed() {
            "PASS"
        } else {
            "FAIL"
        }
    }

    fn status_colored(&self) -> String {
        if self.is_passed() {
            format!("{GREEN}PASS{RESET}")
        } else {
            format!("{RED}FAIL{RESET}")
        }
    }

    fn grade_colored(&self) -> String {
        let color = match self.grade() {
            "A+" | "A" => GREEN,
            "B" | "C" => YELLOW,
            "D" => MAGENTA,
            _ => RED,
        };
        format!("{color}{}{RESET}", self.grade())
    }
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
            _ => println!("{RED}Please enter a valid positive number.{RESET}"),
        }
    }
}

fn read_marks(prompt: &str) -> f32 {
    loop {
        let input = read_line(prompt);

        match input.parse::<f32>() {
            Ok(marks) if (0.0..=100.0).contains(&marks) => return marks,
            _ => println!("{RED}Please enter marks between 0 and 100.{RESET}"),
        }
    }
}

fn read_yes_no(prompt: &str) -> bool {
    loop {
        let input = read_line(prompt).to_lowercase();
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("{RED}Please answer y/n.{RESET}"),
        }
    }
}

fn division(average: f32) -> (&'static str, &'static str) {
    match average {
        x if x >= 75.0 => ("Distinction", GREEN),
        x if x >= 60.0 => ("First Division", GREEN),
        x if x >= 50.0 => ("Second Division", YELLOW),
        _ => ("Fail", RED),
    }
}

fn separator(width: usize) -> String {
    "-".repeat(width)
}

fn header(title: &str, width: usize) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "\n{BOLD}{}{RESET}", "=".repeat(width));
    let _ = writeln!(s, "{BOLD}{:^width$}{RESET}", title, width = width);
    let _ = writeln!(s, "{BOLD}{}{RESET}", "=".repeat(width));
    s
}

fn main() {
    const WIDTH: usize = 70;

    println!("{}", header("STUDENT RESULT CHECKER", WIDTH));

    let name = loop {
        let name = read_line("Enter your name: ");

        if !name.is_empty() {
            break name;
        }

        println!("{RED}Name cannot be empty.{RESET}");
    };

    let count = read_usize("How many subjects? ");
    let mut subjects: Vec<Subject> = Vec::with_capacity(count);

    for i in 1..=count {
        println!("\n{CYAN}--- Subject {} ---{RESET}", i);

        let subject_name = loop {
            let candidate = read_line("Subject name : ");

            if candidate.is_empty() {
                println!("{RED}Subject name cannot be empty.{RESET}");
                continue;
            }

            if subjects
                .iter()
                .any(|s: &Subject| s.name.eq_ignore_ascii_case(&candidate))
            {
                println!("{RED}That subject was already entered. Choose a different one.{RESET}");
                continue;
            }

            break candidate;
        };

        let marks = read_marks("Marks (0-100): ");

        subjects.push(Subject {
            name: subject_name,
            marks,
        });
    }

    // ---------- Calculations ----------
    let total: f32 = subjects.iter().map(|s| s.marks).sum();
    let average = total / subjects.len() as f32;
    let all_passed = subjects.iter().all(|s| s.is_passed());
    let passed_count = subjects.iter().filter(|s| s.is_passed()).count();
    let failed_count = subjects.len() - passed_count;

    let gpa: f32 =
        subjects.iter().map(|s| s.grade_point()).sum::<f32>() / subjects.len() as f32;

    let highest = subjects
        .iter()
        .max_by(|a, b| a.marks.partial_cmp(&b.marks).unwrap())
        .unwrap();
    let lowest = subjects
        .iter()
        .min_by(|a, b| a.marks.partial_cmp(&b.marks).unwrap())
        .unwrap();

    let overall_grade = Subject {
        name: String::new(),
        marks: average,
    }
    .grade();

    let (division_label, division_color) = division(average);

    // ---------- Build report into a buffer (printed + optionally saved) ----------
    let mut report = String::new();

    let _ = write!(
        report,
        "{}",
        header(&format!("RESULT CARD FOR {}", name.to_uppercase()), WIDTH)
    );

    let _ = writeln!(report, "{}", separator(WIDTH));
    let _ = writeln!(
        report,
        "{:<25} {:>8} {:>8} {:>10}",
        "Subject", "Marks", "Grade", "Status"
    );
    let _ = writeln!(report, "{}", separator(WIDTH));

    for subject in &subjects {
        let _ = writeln!(
            report,
            "{:<25} {:>8.1} {:>8} {:>10}",
            subject.name,
            subject.marks,
            subject.grade(),
            subject.status()
        );
    }

    let _ = writeln!(report, "{}", separator(WIDTH));
    let _ = writeln!(report, "{:<25} {:>8.1}", "Total", total);
    let _ = writeln!(report, "{:<25} {:>8.1}", "Average / Percentage", average);
    let _ = writeln!(report, "{:<25} {:>8}", "Overall Grade", overall_grade);
    let _ = writeln!(report, "{:<25} {:>8.2}", "GPA (4.0 scale)", gpa);
    let _ = writeln!(report, "{:<25} {:>8}", "Division", division_label);
    let _ = writeln!(
        report,
        "{:<25} {:>8}/{}",
        "Subjects Passed", passed_count, subjects.len()
    );

    let _ = writeln!(report, "{}", separator(WIDTH));

    let _ = writeln!(report, "\nHIGHLIGHTS");
    let _ = writeln!(report, "{}", separator(WIDTH));
    let _ = writeln!(
        report,
        "Best subject : {} ({:.1} marks, {})",
        highest.name,
        highest.marks,
        highest.grade()
    );
    let _ = writeln!(
        report,
        "Weakest subject : {} ({:.1} marks, {})",
        lowest.name,
        lowest.marks,
        lowest.grade()
    );

    let _ = writeln!(report, "\nSUBJECT-WISE FEEDBACK");
    let _ = writeln!(report, "{}", separator(WIDTH));

    for subject in &subjects {
        let _ = writeln!(report, "{:<25} {}", subject.name, subject.feedback());
    }

    let _ = writeln!(report, "\nOVERALL VERDICT");
    let _ = writeln!(report, "{}", separator(WIDTH));

    if all_passed && average >= 75.0 {
        let _ = writeln!(report, "{} passed with distinction!", name);
    } else if all_passed {
        let _ = writeln!(report, "{} has passed.", name);
        let _ = writeln!(report, "Focus on weaker subjects to improve the overall score.");
    } else {
        let _ = writeln!(report, "{} has failed ({} subject(s) below 50).", name, failed_count);
        let _ = writeln!(report, "Focus on subjects below 50 marks and seek additional help.");
    }

    let _ = writeln!(report, "\n{}", "=".repeat(WIDTH));
    let _ = writeln!(report, "{:^width$}", "END OF RESULT", width = WIDTH);
    let _ = writeln!(report, "{}", "=".repeat(WIDTH));

    // ---------- Print with color where it matters ----------
    println!("{}", report);

    println!("{CYAN}Quick colored view:{RESET}");
    println!("{}", separator(WIDTH));
    for subject in &subjects {
        println!(
            "{:<25} {:>8.1}   {}   {}",
            subject.name,
            subject.marks,
            subject.grade_colored(),
            subject.status_colored()
        );
    }
    println!("{}", separator(WIDTH));

    println!(
        "{BOLD}Overall status: {}{}{RESET}   {BOLD}Division: {}{}{RESET}",
        if all_passed { GREEN } else { RED },
        if all_passed { "PASS" } else { "FAIL" },
        division_color,
        division_label
    );

    // ---------- Optional: save plain (non-colored) report to file ----------
    if read_yes_no("\nSave this result card to a file? (y/n): ") {
        let default_filename = format!(
            "{}_result.txt",
            name.trim().to_lowercase().replace(' ', "_")
        );
        let filename_input = read_line(&format!(
            "Enter filename [default: {}]: ",
            default_filename
        ));
        let filename = if filename_input.is_empty() {
            default_filename
        } else {
            filename_input
        };

        match fs::write(&filename, &report) {
            Ok(_) => println!("{GREEN}Saved to {}{RESET}", filename),
            Err(e) => println!("{RED}Failed to save file: {}{RESET}", e),
        }
    }
}
