use eframe::egui;

fn main () {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "PentTracker",
        native_options,
        Box::new(|cc| Box::new(PTEguiApp::new(cc))))
}

#[derive(Default)]
struct PTEguiApp {}

impl PTEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PTEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PentTracker");
        });
    }
}