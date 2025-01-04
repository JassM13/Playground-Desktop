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
        // Set white background for the whole app
        ctx.set_visuals(egui::Visuals::light());
        
        // Configure the style for the side panel
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::from_rgb(240, 240, 240);
        ctx.set_style(style);

        egui::SidePanel::left("friends_panel")
            .default_width(400.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(egui::Frame::none()
                .fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                ui.add_space(16.0);  // Top margin
                
                // Get available rect and shrink it for margins
                let mut panel_rect = ui.available_rect_before_wrap();
                panel_rect.min.x += 16.0;  // Left margin
                panel_rect.max.x -= 16.0;  // Right margin
                panel_rect.max.y -= 16.0;  // Bottom margin
                
                // Draw the black rounded rectangle background
                ui.painter().rect_filled(
                    panel_rect,
                    egui::Rounding::same(12.0),
                    egui::Color32::BLACK,
                );

                // Content container with padding
                egui::Frame::none()
                    .inner_margin(egui::style::Margin {
                        left: 32.0,
                        right: 96.0,  // Increased right padding
                        top: 16.0,
                        bottom: 16.0,
                    })
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 8.0;
                        ui.heading(
                            egui::RichText::new("Friends")
                                .color(egui::Color32::WHITE)
                        );
                        ui.add_space(8.0);
                        
                        for friend in &self.friends {
                            ui.horizontal(|ui| {
                                // Profile picture
                                let size = 32.0;
                                egui::Frame::none()
                                    .rounding(egui::Rounding::same(16.0))
                                    .fill(egui::Color32::from_gray(64))
                                    .show(ui, |ui| {
                                        ui.set_min_size(egui::vec2(size, size));
                                        ui.set_max_size(egui::vec2(size, size));
                                        ui.centered_and_justified(|ui| {
                                            ui.label("👤");
                                        });
                                    });
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(friend)
                                        .color(egui::Color32::WHITE)
                                );
                            });
                            ui.add_space(4.0);
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Video Grid");
            ui.add_space(16.0);
            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("video_grid")
                    .spacing([16.0, 16.0])
                    .min_col_width(240.0)
                    .max_col_width(240.0)
                    .show(ui, |ui| {
                        for (i, participant) in self.participants.iter().enumerate() {
                            if i > 0 && i % 2 == 0 {
                                ui.end_row();
                            }
                            
                            let response = egui::Frame::none()
                                .rounding(egui::Rounding::same(12.0))
                                .fill(egui::Color32::from_gray(20))
                                .shadow(egui::epaint::Shadow {
                                    extrusion: 2.0,
                                    color: egui::Color32::from_black_alpha(100),
                                })
                                .inner_margin(egui::style::Margin::same(12.0))
                                .show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(240.0, 180.0));
                                    ui.vertical_centered(|ui| {
                                        // Camera view with rounded corners
                                        egui::Frame::none()
                                            .rounding(egui::Rounding::same(8.0))
                                            .fill(egui::Color32::from_gray(30))
                                            .show(ui, |ui| {
                                                ui.add_sized(
                                                    [220.0, 140.0],
                                                    egui::Label::new(
                                                        egui::RichText::new("📹")
                                                            .size(32.0)
                                                    )
                                                );
                                            });
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new(participant)
                                                .size(16.0)
                                        );
                                    });
                                }).response;

                            // Store rect before moving response
                            let rect = response.rect;
                            response.on_hover_ui(|ui| {
                                ui.painter().rect_filled(
                                    rect,  // Use stored rect instead of response.rect
                                    egui::Rounding::same(12.0),
                                    egui::Color32::from_gray(25),
                                );
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
            .with_title("Playground"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Playground",
        native_options,
        Box::new(|_cc| Box::new(PlaygroundApp::default())),
    )
}