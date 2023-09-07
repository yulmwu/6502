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

            if ui.button("halt").clicked() {
                app.is_running = false;
                app.emulator.debug("Halted");
            }

            if ui.button("load").clicked() {
                let src = match Assembler::new(app.source_input.clone()).assemble() {
                    Ok(src) => src,
                    Err(e) => {
                        app.error = Some(e.to_string());
                        return;
                    }
                };

                app.emulator.reset();
                app.emulator.load(&src);
            }

            if ui.button("reset").clicked() {
                app.emulator.reset();
            }
        });
    }
}
