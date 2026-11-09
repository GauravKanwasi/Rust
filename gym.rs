use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const DB_FILE: &str = "gym_members.csv";
const TMP_FILE: &str = "gym_members.csv.tmp";
const BAK_FILE: &str = "gym_members.csv.bak";
const WIDTH: usize = 104;

// ───────────────────────────── Data model ─────────────────────────────

#[derive(Clone)]
struct Member {
    name: String,
    age: u32,
    plan: Plan,
    email: String,
    phone: String,
    joined: String, // YYYY-MM-DD (UTC)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Plan {
    Basic,
    Standard,
    Premium,
}

impl Plan {
    const ALL: [Plan; 3] = [Plan::Basic, Plan::Standard, Plan::Premium];

    fn from_choice(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Basic),
            "2" => Some(Self::Standard),
            "3" => Some(Self::Premium),
            _ => None,
        }
    }

    fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.to_string() == s)
    }

    /// Monthly fee in dollars.
    fn fee(self) -> u32 {
        match self {
            Self::Basic => 20,
            Self::Standard => 35,
            Self::Premium => 55,
        }
    }
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Basic => "Basic",
            Self::Standard => "Standard",
            Self::Premium => "Premium",
        })
    }
}

// ───────────────────────────── UI helpers ─────────────────────────────

/// Reads one trimmed line. Exits cleanly on EOF (Ctrl-D / closed stdin)
/// instead of looping forever or panicking. Data is saved after every change.
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().ok();
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(0) | Err(_) => {
            println!("\n\n  Goodbye!\n");
            std::process::exit(0);
        }
        Ok(_) => s.trim().to_string(),
    }
}

fn divider() {
    println!("{}", "═".repeat(WIDTH));
}

fn header(title: &str) {
    println!();
    divider();
    println!("  {}", title);
    divider();
}

fn banner() {
    let inner = WIDTH - 2;
    println!("╔{}╗", "═".repeat(inner));
    println!("║{:^w$}║", "GYM MEMBERSHIP MANAGEMENT SYSTEM", w = inner);
    println!("╚{}╝", "═".repeat(inner));
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

/// Truncates to `n` characters, adding an ellipsis so columns stay aligned.
fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let t: String = s.chars().take(n - 1).collect();
        format!("{}…", t)
    }
}

fn table_header() {
    println!(
        "  {:<5} │ {:<18} │ {:<3} │ {:<8} │ {:<24} │ {:<16} │ {}",
        "ID", "Name", "Age", "Plan", "Email", "Phone", "Joined"
    );
    divider();
}

fn print_member(id: u32, m: &Member) {
    println!(
        "  #{:<4} │ {:<18} │ {:<3} │ {:<8} │ {:<24} │ {:<16} │ {}",
        id,
        clip(&m.name, 18),
        m.age,
        m.plan.to_string(),
        clip(&m.email, 24),
        clip(&m.phone, 16),
        m.joined
    );
}

/// Sorted table of the given hits.
fn show_list(mut hits: Vec<(u32, &Member)>) {
    hits.sort_by_key(|(id, _)| *id);
    divider();
    table_header();
    for (id, m) in hits {
        print_member(id, m);
    }
    divider();
}

// ───────────────────────────── Validation ─────────────────────────────

fn valid_name(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic())
        && s.chars()
            .all(|c| c.is_alphabetic() || matches!(c, ' ' | '-' | '\'' | '.'))
}

fn valid_age(s: &str) -> Option<u32> {
    s.parse::<u32>().ok().filter(|a| (1..=120).contains(a))
}

fn valid_email(s: &str) -> bool {
    if s.chars().any(|c| c.is_whitespace() || c == ',') {
        return false;
    }
    match s.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty()
                && !domain.contains('@')
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
        }
        None => false,
    }
}

fn valid_phone(s: &str) -> bool {
    let digits = s.chars().filter(|c| c.is_ascii_digit()).count();
    s.chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | '(' | ')' | ' '))
        && (7..=15).contains(&digits)
}

// ───────────────────────────── Input helpers ─────────────────────────────
// Every `ask_*` returns None if the user presses Enter on an empty line,
// which lets them cancel out of any flow.

fn ask<T>(label: &str, hint: &str, parse: impl Fn(&str) -> Option<T>) -> Option<T> {
    loop {
        let v = prompt(&format!("  {:<12}: ", label));
        if v.is_empty() {
            return None;
        }
        if let Some(x) = parse(&v) {
            return Some(x);
        }
        err(hint);
    }
}

fn ask_name(label: &str) -> Option<String> {
    ask(label, "Letters, spaces, hyphens and apostrophes only.", |s| {
        valid_name(s).then(|| s.to_string())
    })
}

fn ask_age(label: &str) -> Option<u32> {
    ask(label, "Enter a number between 1 and 120.", valid_age)
}

fn ask_phone(label: &str) -> Option<String> {
    ask(label, "Phone must contain 7–15 digits.", |s| {
        valid_phone(s).then(|| s.to_string())
    })
}

fn ask_email(label: &str, members: &HashMap<u32, Member>, exclude: Option<u32>) -> Option<String> {
    loop {
        let v = ask(label, "Invalid email (e.g. name@example.com).", |s| {
            valid_email(s).then(|| s.to_string())
        })?;
        let taken = members
            .iter()
            .any(|(id, m)| Some(*id) != exclude && m.email.eq_ignore_ascii_case(&v));
        if !taken {
            return Some(v);
        }
        err("That email is already registered.");
    }
}

fn ask_plan(label: &str) -> Option<Plan> {
    loop {
        println!(
            "  {}: [1] Basic ${}/mo  [2] Standard ${}/mo  [3] Premium ${}/mo",
            label,
            Plan::Basic.fee(),
            Plan::Standard.fee(),
            Plan::Premium.fee()
        );
        let v = prompt("  Choose (1/2/3): ");
        if v.is_empty() {
            return None;
        }
        if let Some(p) = Plan::from_choice(&v) {
            return Some(p);
        }
        err("Enter 1, 2 or 3.");
    }
}

/// Prompts for an existing member ID. Returns None on cancel or unknown ID.
fn ask_id(members: &HashMap<u32, Member>) -> Option<u32> {
    let raw = prompt("  Member ID   : ");
    if raw.is_empty() {
        return None;
    }
    let id = raw.trim_start_matches('#').parse::<u32>().ok();
    match id {
        Some(id) if members.contains_key(&id) => Some(id),
        _ => {
            err(&format!("No member with ID '{}'.", raw));
            None
        }
    }
}

// ───────────────────────────── Dates ─────────────────────────────

/// Today's date (UTC) as YYYY-MM-DD, without external crates.
fn today() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    // Howard Hinnant's civil-from-days algorithm.
    let z = secs / 86_400 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{:04}-{:02}-{:02}", y, m, d)
}

// ───────────────────────────── Persistence ─────────────────────────────
// Format: id,name,age,plan,email,phone,joined   (6-column legacy files still load)
// Validation forbids commas in every field, so no CSV quoting is needed.

fn parse_line(line: &str) -> Option<(u32, Member)> {
    let p: Vec<&str> = line.split(',').collect();
    if p.len() != 6 && p.len() != 7 {
        return None;
    }
    Some((
        p[0].parse().ok()?,
        Member {
            name: p[1].to_string(),
            age: p[2].parse().ok()?,
            plan: Plan::parse(p[3])?,
            email: p[4].to_string(),
            phone: p[5].to_string(),
            joined: p.get(6).map(|s| s.to_string()).unwrap_or_else(today),
        },
    ))
}

fn load_data() -> (HashMap<u32, Member>, u32) {
    let mut members = HashMap::new();
    let mut max_id = 0;
    let mut skipped = 0;

    if let Ok(file) = File::open(DB_FILE) {
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }
            match parse_line(&line) {
                Some((id, m)) => {
                    max_id = max_id.max(id);
                    members.insert(id, m);
                }
                None => skipped += 1,
            }
        }
    }

    if skipped > 0 {
        err(&format!(
            "Skipped {} unreadable line(s). Original file backed up to {}.",
            skipped, BAK_FILE
        ));
        let _ = fs::copy(DB_FILE, BAK_FILE);
    }
    (members, max_id + 1)
}

/// Writes to a temp file then renames, so a crash mid-write can't corrupt the database.
fn save_data(members: &HashMap<u32, Member>) -> io::Result<()> {
    let mut ids: Vec<u32> = members.keys().copied().collect();
    ids.sort_unstable();

    let mut f = File::create(TMP_FILE)?;
    for id in ids {
        let m = &members[&id];
        writeln!(
            f,
            "{},{},{},{},{},{},{}",
            id, m.name, m.age, m.plan, m.email, m.phone, m.joined
        )?;
    }
    f.sync_all()?;
    fs::rename(TMP_FILE, DB_FILE)
}

fn persist(members: &HashMap<u32, Member>) {
    if let Err(e) = save_data(members) {
        err(&format!("Could not save data: {}", e));
    }
}

// ───────────────────────────── Features ─────────────────────────────

fn read_member(members: &HashMap<u32, Member>) -> Option<Member> {
    Some(Member {
        name: ask_name("Name")?,
        age: ask_age("Age")?,
        email: ask_email("Email", members, None)?,
        phone: ask_phone("Phone")?,
        plan: ask_plan("Plan")?,
        joined: today(),
    })
}

fn add_member(members: &mut HashMap<u32, Member>, next_id: &mut u32) {
    header("ADD MEMBER");
    info("Press Enter on an empty field to cancel.");
    let Some(member) = read_member(members) else {
        info("Cancelled.");
        return;
    };
    members.insert(*next_id, member);
    ok(&format!("Member added — ID #{}", *next_id));
    *next_id += 1;
    persist(members);
}

fn view_members(members: &HashMap<u32, Member>) {
    header(&format!("ALL MEMBERS ({})", members.len()));
    if members.is_empty() {
        info("No members yet.");
        return;
    }
    let sort = prompt("  Sort by [1] ID  [2] Name  [3] Newest  (Enter = ID): ");
    let mut list: Vec<(u32, &Member)> = members.iter().map(|(id, m)| (*id, m)).collect();
    match sort.as_str() {
        "2" => list.sort_by_key(|(id, m)| (m.name.to_lowercase(), *id)),
        "3" => list.sort_by(|a, b| b.1.joined.cmp(&a.1.joined).then(b.0.cmp(&a.0))),
        _ => list.sort_by_key(|(id, _)| *id),
    }
    divider();
    table_header();
    for (id, m) in list {
        print_member(id, m);
    }
    divider();
}

fn search_members(members: &HashMap<u32, Member>) {
    if members.is_empty() {
        err("No members to search.");
        return;
    }
    header("SEARCH");
    println!("  [1] By ID   [2] By Name / Email / Phone   [3] By Plan");
    let choice = prompt("  Choose: ");

    match choice.as_str() {
        "1" => {
            if let Some(id) = ask_id(members) {
                show_list(vec![(id, &members[&id])]);
            }
        }
        "2" => {
            let q = prompt("  Search text : ").to_lowercase();
            if q.is_empty() {
                return;
            }
            let hits: Vec<(u32, &Member)> = members
                .iter()
                .filter(|(_, m)| {
                    m.name.to_lowercase().contains(&q)
                        || m.email.to_lowercase().contains(&q)
                        || m.phone.contains(&q)
                })
                .map(|(id, m)| (*id, m))
                .collect();
            if hits.is_empty() {
                err(&format!("No match for '{}'.", q));
            } else {
                show_list(hits);
            }
        }
        "3" => {
            let Some(plan) = ask_plan("Filter by") else { return };
            let hits: Vec<(u32, &Member)> = members
                .iter()
                .filter(|(_, m)| m.plan == plan)
                .map(|(id, m)| (*id, m))
                .collect();
            if hits.is_empty() {
                info(&format!("No members on the {} plan.", plan));
            } else {
                show_list(hits);
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
    info("Press Enter on an empty field to cancel.");

    let Some(id) = ask_id(members) else { return };
    show_list(vec![(id, &members[&id])]);

    println!("  [1] Name  [2] Age  [3] Plan  [4] Email  [5] Phone");
    let field = prompt("  Field to update: ");

    let mut m = members[&id].clone();
    let changed = match field.as_str() {
        "1" => ask_name("New name").map(|v| m.name = v).is_some(),
        "2" => ask_age("New age").map(|v| m.age = v).is_some(),
        "3" => ask_plan("New plan").map(|v| m.plan = v).is_some(),
        "4" => ask_email("New email", members, Some(id)).map(|v| m.email = v).is_some(),
        "5" => ask_phone("New phone").map(|v| m.phone = v).is_some(),
        _ => {
            err("Invalid field.");
            return;
        }
    };

    if changed {
        members.insert(id, m);
        ok("Member updated.");
        persist(members);
    } else {
        info("No changes made.");
    }
}

fn remove_member(members: &mut HashMap<u32, Member>) {
    if members.is_empty() {
        err("No members to remove.");
        return;
    }
    header("REMOVE MEMBER");

    let Some(id) = ask_id(members) else { return };
    let m = &members[&id];
    info(&format!("About to remove: {} ({}, {})", m.name, m.plan, m.email));

    let confirm = prompt("  Confirm? (yes/no): ");
    if confirm.eq_ignore_ascii_case("yes") || confirm.eq_ignore_ascii_case("y") {
        members.remove(&id);
        ok("Member removed.");
        persist(members);
    } else {
        info("Cancelled.");
    }
}

fn show_stats(members: &HashMap<u32, Member>) {
    header("STATISTICS");
    if members.is_empty() {
        info("No data yet.");
        return;
    }

    let total = members.len();
    let avg = members.values().map(|m| m.age as f64).sum::<f64>() / total as f64;
    let youngest = members.values().min_by_key(|m| m.age).unwrap();
    let oldest = members.values().max_by_key(|m| m.age).unwrap();
    let revenue: u32 = members.values().map(|m| m.plan.fee()).sum();

    println!("  Total Members   : {}", total);
    println!("  Average Age     : {:.1}", avg);
    println!("  Youngest        : {} ({})", youngest.name, youngest.age);
    println!("  Oldest          : {} ({})", oldest.name, oldest.age);
    println!("  Monthly Revenue : ${}", revenue);
    divider();

    const BAR_MAX: usize = 25;
    for plan in Plan::ALL {
        let count = members.values().filter(|m| m.plan == plan).count();
        let pct = count as f64 * 100.0 / total as f64;
        println!(
            "  {:<8} │ {:>3} │ {:>5.1}% │ ${:<5} │ {}",
            plan.to_string(),
            count,
            pct,
            count as u32 * plan.fee(),
            "█".repeat(count * BAR_MAX / total)
        );
    }
    divider();
}

// ───────────────────────────── Main ─────────────────────────────

fn main() {
    let (mut members, mut next_id) = load_data();

    banner();

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
