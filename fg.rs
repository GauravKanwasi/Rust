use eframe::egui;

fn factorial(n: u64) -> u128 {
    (1..=n as u128).product()
}

struct FactorialApp {
    input: String,
    result: String,
}

impl Default for FactorialApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: String::from("Result will appear here"),
        }
    }
}

impl eframe::App for FactorialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Factorial Calculator (Rust GUI)");
            ui.separator();

            ui.label("Enter a number:");
            ui.text_edit_singleline(&mut self.input);

            if ui.button("Calculate").clicked() {
                match self.input.trim().parse::<u64>() {
                    Ok(n) => {
                        self.result = format!("Factorial of {} is {}", n, factorial(n));
                    }
                    Err(_) => {
                        self.result = String::from("Invalid input! Please enter a number.");
                    }
                }
            }

            ui.separator();
            ui.label(&self.result);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Factorial GUI",
        options,
        Box::new(|_cc| Box::new(FactorialApp::default())),
    )
}
