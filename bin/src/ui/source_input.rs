use crate::{app::App, View};
use eframe::egui::*;

pub struct SourceInput;

impl View for SourceInput {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.vertical_centered(|ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().visuals.extreme_bg_color = Color32::default();
                ui.vertical_centered(|ui| {
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut app.source_input)
                            .code_editor()
                            .font(FontId::new(15., FontFamily::Monospace))
                            .text_color(Color32::WHITE),
                    );
                });
            });
        });
    }
}
