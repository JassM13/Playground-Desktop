use eframe::egui;

pub struct FriendsPanel;

impl FriendsPanel {
    pub fn show(ctx: &egui::Context, friends: &[String]) {
        egui::SidePanel::left("friends_panel")
            .min_width(256.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(egui::Frame::none().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                let panel_rect = ui.available_rect_before_wrap();
                let inner_rect = panel_rect.shrink(16.0);
                
                ui.painter().rect_filled(
                    inner_rect,
                    egui::Rounding::same(12.0),
                    egui::Color32::from_rgb(20, 20, 20),
                );

                let content_rect = inner_rect.shrink(24.0);
            
                ui.allocate_ui_at_rect(content_rect, |ui| {
                    match friends.len() {
                        _ => {
                            Self::render_content(ui, friends);
                        },
                    }
                });
            });
    }

    fn render_content(ui: &mut egui::Ui, friends: &[String]) {
        ui.spacing_mut().item_spacing.y = 8.0;
        ui.heading(
            egui::RichText::new("Friends")
                .color(egui::Color32::WHITE)
        );
        ui.add_space(8.0);
        
        for friend in friends {
            ui.horizontal(|ui| {
                Self::render_profile_picture(ui);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(friend)
                        .color(egui::Color32::WHITE)
                );
            });
            ui.add_space(4.0);
        }
    }

    fn render_profile_picture(ui: &mut egui::Ui) {
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
    }
}
