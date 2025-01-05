use eframe::egui;
mod views;
use views::main_view::PlaygroundApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Playground"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Playground",
        native_options,
        Box::new(|_cc| Box::new(PlaygroundApp::default())),
    )
}

