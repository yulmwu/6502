use crate::{app::App, View};
use eframe::egui::*;

pub struct DiassemblerUi;

impl View for DiassemblerUi {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for (pointer, instruction) in app.disassembled.iter() {
                    // ui.label(format!("0x{:04X}: {}", i + 0x8000, line));
                    let pointer = Label::new(
                        RichText::new(format!("0x{:04X}", pointer + 0x8000))
                            .monospace()
                            .color(Color32::from_rgb(50, 180, 80)),
                    );
                    let instruction = Label::new(RichText::new(instruction).monospace());

                    ui.horizontal(|ui| {
                        ui.add(pointer);
                        ui.separator();
                        ui.add(instruction);
                    });

                    ui.separator();
                }
            });
    }
}
