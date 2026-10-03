use eframe::egui;
use std::time::{Duration, Instant};

const MAX_N: u32 = 10_000;
const BASE: u64 = 1_000_000_000;
const HISTORY_LEN: usize = 8;

fn factorial(n: u32) -> Vec<u32> {
    let mut limbs = vec![1u32];
    for k in 2..=n as u64 {
        let mut carry = 0u64;
        for limb in limbs.iter_mut() {
            let v = *limb as u64 * k + carry;
            *limb = (v % BASE) as u32;
            carry = v / BASE;
        }
        while carry > 0 {
            limbs.push((carry % BASE) as u32);
            carry /= BASE;
        }
    }
    limbs
}

fn to_digits(limbs: &[u32]) -> String {
    let mut s = String::with_capacity(limbs.len() * 9);
    let mut iter = limbs.iter().rev();
    if let Some(first) = iter.next() {
        s.push_str(&first.to_string());
    }
    for limb in iter {
        s.push_str(&format!("{limb:09}"));
    }
    s
}

fn group_digits(digits: &str) -> String {
    let len = digits.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
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
    let frac: String = digits.chars().skip(1).take(5).collect();
    format!("{}.{}e+{}", &digits[..1], frac, digits.len() - 1)
}

struct Output {
    n: u32,
    digits: String,
    grouped: String,
    elapsed: Duration,
}

#[derive(Default)]
struct App {
    input: String,
    output: Option<Output>,
    error: Option<String>,
    history: Vec<u32>,
    group: bool,
}

impl App {
    fn submit(&mut self) {
        let raw: String = self
            .input
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '_' && *c != ',')
            .collect();

        if raw.is_empty() {
            self.fail("Enter a number first.");
            return;
        }

        match raw.parse::<u32>() {
            Ok(n) if n <= MAX_N => self.compute(n),
            Ok(_) => self.fail(format!("Please enter a value no larger than {MAX_N}.")),
            Err(_) => self.fail("Enter a whole number of 0 or greater."),
        }
    }

    fn compute(&mut self, n: u32) {
        let start = Instant::now();
        let digits = to_digits(&factorial(n));
        let elapsed = start.elapsed();
        let grouped = group_digits(&digits);

        self.input = n.to_string();
        self.output = Some(Output { n, digits, grouped, elapsed });
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
                let digit_count = out.digits.len();
                ui.label(egui::RichText::new(format!("{}!", out.n)).strong().size(20.0));
                ui.label(format!(
                    "{} digits  |  {} trailing zeros  |  ≈ {}  |  {:.2?}",
                    group_digits(&digit_count.to_string()),
                    trailing_zeros(out.n),
                    scientific(&out.digits),
                    out.elapsed,
                ));

                ui.horizontal(|ui| {
                    if ui.button("Copy").clicked() {
                        ui.ctx().copy_text(out.digits.clone());
                    }
                    ui.checkbox(&mut self.group, "Group digits");
                });

                let mut shown: &str = if self.group { &out.grouped } else { &out.digits };
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
