use eframe::egui;

fn factorial(n: u64) -> u128 {
    (1..=n as u128).product()
}

#[derive(Default)]
struct App {
    input: String,
    result: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Factorial Calculator");
            ui.separator();
            ui.label("Enter a number:");
            let enter = ui.text_edit_singleline(&mut self.input).lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Calculate").clicked() || enter {
                self.result = match self.input.trim().parse::<u64>() {
                    Ok(n) => format!("{}! = {}", n, factorial(n)),
                    Err(_) => "Invalid input.".into(),
                };
            }
            ui.separator();
            if !self.result.is_empty() {
                ui.label(&self.result);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Factorial",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(App::default())),
    )
}
