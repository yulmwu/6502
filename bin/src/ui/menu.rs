use crate::{app::App, View};
use assembler::Assembler;
use eframe::egui::*;

pub struct MenuBar;

impl View for MenuBar {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.horizontal(|ui| {
            if ui.button("run").clicked() {
                app.is_running = true;
            }

            if ui.button("load").clicked() {
                let src = Assembler::new(app.source_input.clone()).assemble().unwrap();

                app.emulator.reset();
                app.emulator.load(&src);
            }

            if ui.button("reset").clicked() {
                app.emulator.reset();
            }

            if ui.button("display").clicked() {
                app.is_display_open = true;
            }
        });
    }
}
