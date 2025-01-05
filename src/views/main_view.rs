use eframe::egui;
use super::components::{friends_panel::FriendsPanel, video_grid::VideoGrid};

pub struct PlaygroundApp {
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
            participants: vec!["Jass".to_string(), "Participant 1".to_string(), "Participant 2".to_string(), "Participant 2".to_string()],
        }
    }
}

impl eframe::App for PlaygroundApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Configure the style for the side panel
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::BLACK;
        ctx.set_style(style);

        FriendsPanel::show(ctx, &self.friends);
        VideoGrid::show(ctx, &self.participants);
    }
}
