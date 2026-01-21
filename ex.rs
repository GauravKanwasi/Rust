use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Egui App (ex.rs)",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    counter: i32,
    name: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui Example");
            ui.separator();

            ui.label("Your name:");
            ui.text_edit_singleline(&mut self.name);

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.counter += 1;
                }
                if ui.button("Reset").clicked() {
                    self.counter = 0;
                }
            });

            ui.add_space(10.0);
            ui.label(format!("Counter: {}", self.counter));

            if !self.name.is_empty() {
                ui.label(format!("Hello, {} 👋", self.name));
            }

            ui.separator();
            ui.small("File: ex.rs | Rust + egui");
        });
    }
}
