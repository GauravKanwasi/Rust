use eframe::egui;
use std::cell::OnceCell;
use std::fmt::Write;
use std::time::{Duration, Instant};

const MAX_N: u32 = 10_000;
const BASE: u64 = 1_000_000_000;
const BATCH_LIMIT: u64 = 10_000_000_000;
const HISTORY_LEN: usize = 8;

fn mul_small(limbs: &mut Vec<u32>, m: u64) {
    let mut carry = 0u64;
    for limb in limbs.iter_mut() {
        let v = *limb as u64 * m + carry;
        *limb = (v % BASE) as u32;
        carry = v / BASE;
    }
    while carry > 0 {
        limbs.push((carry % BASE) as u32);
        carry /= BASE;
    }
}

fn extend_factorial(limbs: &mut Vec<u32>, from: u32, to: u32) {
    let end = to as u64;
    let mut k = from as u64 + 1;
    while k <= end {
        let mut m = 1u64;
        while k <= end && m * k <= BATCH_LIMIT {
            m *= k;
            k += 1;
        }
        mul_small(limbs, m);
    }
}

struct Cache {
    n: u32,
    limbs: Vec<u32>,
}

impl Default for Cache {
    fn default() -> Self {
        let mut limbs = Vec::with_capacity(4096);
        limbs.push(1);
        Self { n: 0, limbs }
    }
}

impl Cache {
    fn advance_to(&mut self, n: u32) {
        if n < self.n {
            self.n = 0;
            self.limbs.clear();
            self.limbs.push(1);
        }
        extend_factorial(&mut self.limbs, self.n, n);
        self.n = n;
    }
}

fn to_digits(limbs: &[u32]) -> String {
    let mut s = String::with_capacity(limbs.len() * 9);
    if let Some((last, rest)) = limbs.split_last() {
        write!(s, "{last}").unwrap();
        for limb in rest.iter().rev() {
            write!(s, "{limb:09}").unwrap();
        }
    }
    s
}

fn group_digits(digits: &str) -> String {
    let len = digits.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, &b) in digits.as_bytes().iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(b as char);
    }
    out
}

fn trailing_zeros(n: u32) -> u32 {
    let mut count = 0;
    let mut p = 5u64;
    while p <= n as u64 {
        count += n as u64 / p;
        p *= 5;
    }
    count as u32
}

fn scientific(digits: &str) -> String {
    if digits.len() <= 1 {
        return digits.to_string();
    }
    format!("{}.{}e+{}", &digits[..1], &digits[1..digits.len().min(6)], digits.len() - 1)
}

struct Output {
    n: u32,
    digits: String,
    grouped: OnceCell<String>,
    summary: String,
}

#[derive(Default)]
struct App {
    input: String,
    output: Option<Output>,
    error: Option<String>,
    history: Vec<u32>,
    group: bool,
    cache: Cache,
}

impl App {
    fn submit(&mut self) {
        let raw: String = self
            .input
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '_' && *c != ',')
            .collect();

        if raw.is_empty() {
            return self.fail("Enter a number first.");
        }
        if !raw.bytes().all(|b| b.is_ascii_digit()) {
            return self.fail("Enter a whole number of 0 or greater.");
        }
        match raw.parse::<u32>() {
            Ok(n) if n <= MAX_N => self.compute(n),
            _ => self.fail(format!("Please enter a value no larger than {MAX_N}.")),
        }
    }

    fn compute(&mut self, n: u32) {
        let start = Instant::now();
        self.cache.advance_to(n);
        let digits = to_digits(&self.cache.limbs);
        let elapsed: Duration = start.elapsed();

        let summary = format!(
            "{} digits  |  {} trailing zeros  |  ≈ {}  |  {:.2?}",
            group_digits(&digits.len().to_string()),
            trailing_zeros(n),
            scientific(&digits),
            elapsed,
        );

        self.input = n.to_string();
        self.output = Some(Output { n, digits, grouped: OnceCell::new(), summary });
        self.error = None;
        self.history.retain(|&h| h != n);
        self.history.insert(0, n);
        self.history.truncate(HISTORY_LEN);
    }

    fn fail(&mut self, msg: impl Into<String>) {
        self.error = Some(msg.into());
        self.output = None;
    }

    fn clear(&mut self) {
        self.input.clear();
        self.output = None;
        self.error = None;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Factorial Calculator");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("n =");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.input)
                        .hint_text(format!("0 – {MAX_N}"))
                        .desired_width(120.0),
                );
                let submitted =
                    resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Calculate").clicked() || submitted {
                    self.submit();
                    resp.request_focus();
                }
                if ui.button("Clear").clicked() {
                    self.clear();
                }
            });

            if !self.history.is_empty() {
                let mut picked = None;
                ui.horizontal_wrapped(|ui| {
                    ui.weak("Recent:");
                    for &n in &self.history {
                        if ui.small_button(n.to_string()).clicked() {
                            picked = Some(n);
                        }
                    }
                });
                if let Some(n) = picked {
                    self.compute(n);
                }
            }

            ui.separator();

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::LIGHT_RED, err);
            }

            if let Some(out) = &self.output {
                ui.label(egui::RichText::new(format!("{}!", out.n)).strong().size(20.0));
                ui.label(&out.summary);

                ui.horizontal(|ui| {
                    if ui.button("Copy digits").clicked() {
                        ui.ctx().copy_text(out.digits.clone());
                    }
                    ui.checkbox(&mut self.group, "Group digits");
                });

                let mut shown: &str = if self.group {
                    out.grouped.get_or_init(|| group_digits(&out.digits))
                } else {
                    &out.digits
                };
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut shown)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([560.0, 460.0])
            .with_min_inner_size([360.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Factorial",
        options,
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}
