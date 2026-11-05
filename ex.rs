use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui Example",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([420.0, 300.0])
                .with_min_inner_size([320.0, 240.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App {
    counter: i32,
    name: String,
}

impl App {
    fn greeting(&self) -> Option<String> {
        let name = self.name.trim();
        (!name.is_empty()).then(|| format!("Hello, {name} 👋"))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui Example");
            ui.separator();

            ui.label("Your name:");
            ui.add(
                egui::TextEdit::singleline(&mut self.name)
                    .hint_text("Type here...")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("−").clicked() {
                    self.counter = self.counter.saturating_sub(1);
                }
                if ui.button("+").clicked() {
                    self.counter = self.counter.saturating_add(1);
                }
                if ui
                    .add_enabled(self.counter != 0, egui::Button::new("Reset"))
                    .clicked()
                {
                    self.counter = 0;
                }
            });
            ui.add_space(10.0);

            ui.label(egui::RichText::new(format!("Counter: {}", self.counter)).size(24.0).strong());

            if let Some(greeting) = self.greeting() {
                ui.label(greeting);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.small("ex.rs | Rust + egui");
                ui.separator();
            });
        });
    }
}
