use eframe::egui;

pub struct VideoGrid;

impl VideoGrid {
    pub fn show(ctx: &egui::Context, participants: &[String]) {
        egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::BLACK))
        .show(ctx, |ui| {
            
            let panel_rect = ui.available_rect_before_wrap();
            let inner_rect = panel_rect.shrink(16.0);
            
            // Draw the background panel
            /*ui.painter().rect_filled(
                inner_rect,
                egui::Rounding::same(12.0),
                egui::Color32::from_rgb(20, 20, 20),
            );*/

            let content_rect = inner_rect;//.shrink(24.0);
            
            ui.allocate_ui_at_rect(content_rect, |ui| {
                match participants.len() {
                    _ => {
                        Self::render_grid_participants(ui, participants);
                    },
                }
            });
        });
    }

    fn render_grid_participants(ui: &mut egui::Ui, participants: &[String]) {
        let available_width = ui.available_width() - 32.0;
        let available_height = ui.available_height() - 40.0;
        let spacing = 16.0;
        
        // Calculate optimal grid layout
        let participant_count = participants.len();
        let rows = (participant_count as f32).sqrt().ceil() as usize;
        let cols = (participant_count + rows - 1) / rows;
        
        // Calculate card size to fill available space
        let card_width = (available_width - (spacing * (cols - 1) as f32)) / cols as f32;
        let card_height = (available_height - (spacing * (rows - 1) as f32)) / rows as f32;
        let size = egui::vec2(card_width, card_height);
        
        ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
        
        egui::Grid::new("video_grid")
            .spacing(egui::vec2(spacing, spacing))
            .show(ui, |ui| {
                for (i, participant) in participants.iter().enumerate() {
                    // Update render_participant_card to use the new size
                    Self::render_participant_card(ui, participant, size);
                    
                    if (i + 1) % cols == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_participant_card(ui: &mut egui::Ui, participant: &str, size: egui::Vec2) {
        egui::Frame::none()
            .rounding(egui::Rounding::same(8.0))
            .fill(egui::Color32::from_rgb(20, 20, 20))
            .inner_margin(egui::style::Margin::same(8.0))
            .show(ui, |ui| {
                ui.set_min_size(size);
                ui.vertical_centered(|ui| {
                    Self::render_camera_view(ui, size);
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(participant)
                            .size(14.0)
                            .color(egui::Color32::WHITE)
                    );
                });
            });
    }

    fn render_camera_view(ui: &mut egui::Ui, size: egui::Vec2) {
        let camera_size = egui::vec2(size.x - 16.0, size.y - 32.0);
        ui.add_sized(
            camera_size,
            egui::Label::new(
                egui::RichText::new("📹")
                    .size(32.0)
                    .color(egui::Color32::WHITE)
            )
        );
    }
}