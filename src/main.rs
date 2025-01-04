use eframe::egui;

struct PlaygroundApp {
    friends: Vec<String>,
    participants: Vec<String>,
}

impl Default for PlaygroundApp {
    fn default() -> Self {
        Self {
            friends: vec![
                "Friend 1".to_string(),
                "Friend 2".to_string(),
                "Friend 3".to_string(),
                "Friend 4".to_string(),
            ],
            participants: vec!["You".to_string(), "Participant 1".to_string(), "Participant 2".to_string()],
        }
    }
}

impl eframe::App for PlaygroundApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set dark theme
        ctx.set_visuals(egui::Visuals::dark());
        
        // Configure the dark theme with jet black background
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.window_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);
        ctx.set_style(style);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading(
                egui::RichText::new("Playground")
                    .strong()
                    .color(egui::Color32::WHITE)
                    .family(egui::FontFamily::Proportional)
            );
        });

        egui::SidePanel::left("friends_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Friends");
                ui.separator();
                for friend in &self.friends {
                    ui.horizontal(|ui| {
                        // Create circular frame for profile picture
                        let size = 32.0;
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
                        egui::Frame::none()
                            .rounding(egui::Rounding::same(4.0))
                            .fill(egui::Color32::from_gray(64))
                            .show(ui, |ui| {
                                ui.set_min_size(egui::vec2(size, size));
                                ui.set_max_size(egui::vec2(size, size));
                                ui.centered_and_justified(|ui| {
                                    ui.label("👤");
                                });
                            });
                        //ui.add_space(8.0);
                        ui.label(friend);
                    });
                    //ui.add_space(4.0);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Video Grid");
            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("video_grid")
                    .spacing([10.0, 10.0])
                    .min_col_width(240.0)
                    .max_col_width(240.0)
                    .show(ui, |ui| {
                        for (i, participant) in self.participants.iter().enumerate() {
                            if i > 0 && i % 2 == 0 {
                                ui.end_row();
                            }
                            
                            egui::Frame::dark_canvas(ui.style())
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(240.0, 180.0));
                                    ui.set_max_size(egui::vec2(240.0, 180.0));
                                    ui.vertical_centered(|ui| {
                                        ui.add_sized(
                                            [220.0, 140.0],
                                            egui::Label::new("📹")
                                        );
                                        ui.label(participant);
                                    });
                                });
                        }
                    });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_icon(load_icon())
            .with_title("Playground"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Playground",
        native_options,
        Box::new(|_cc| Box::new(PlaygroundApp::default())),
    )
}

fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/logo.png");
        let mut image = image::load_from_memory(icon)
            .expect("Failed to load icon")
            .into_rgba8();
        
        // macOS-style rounded corners
        for y in 0..image.height() {
            for x in 0..image.width() {
                let px = x as f32;
                let py = y as f32;
                let width = image.width() as f32;
                let height = image.height() as f32;
                
                // Convert to coordinate system with origin at center
                let dx = (px - width / 2.0) / (width / 2.0);
                let dy = (py - height / 2.0) / (height / 2.0);
                
                // macOS-style squircle formula (n=5 for similar curvature to macOS)
                let n = 5.0;
                let radius = (dx.abs().powf(n) + dy.abs().powf(n)).powf(1.0/n);
                
                if radius > 1.0 {
                    // Outside the squircle, make transparent
                    let pixel = image.get_pixel_mut(x, y);
                    pixel[3] = 0;
                } else if radius > 0.95 {  // Adjust this value to control edge softness
                    // Add anti-aliasing at the edges
                    let alpha = ((1.0 - radius) / 0.05 * 255.0) as u8;
                    let pixel = image.get_pixel_mut(x, y);
                    pixel[3] = alpha.min(pixel[3]);
                }
            }
        }
        
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
