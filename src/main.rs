use eframe::egui;

struct PluteoApp;

impl eframe::App for PluteoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Pluteo");
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(400.0, 300.0))
            .with_title("Pluteo"),
        ..Default::default()
    };

    eframe::run_native(
        "Pluteo",
        options,
        Box::new(|_cc| Ok(Box::new(PluteoApp))),
    );
}
