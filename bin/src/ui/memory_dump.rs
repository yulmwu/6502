use crate::{app::App, View};
use eframe::egui::*;
use emulator::memory::memory_hexdump;

pub struct MemoryDumpOptions;

impl View for MemoryDumpOptions {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.extreme_bg_color = Color32::default();
            ui.add(
                TextEdit::singleline(&mut app.memory_dump_range_input.0)
                    .desired_width(32.)
                    .font(FontId::new(10., FontFamily::Monospace))
                    .text_color(Color32::WHITE),
            );
            ui.add(Label::new(
                RichText::new(" ~ ").monospace().color(Color32::WHITE),
            ));
            ui.add(
                TextEdit::singleline(&mut app.memory_dump_range_input.1)
                    .desired_width(32.)
                    .font(FontId::new(10., FontFamily::Monospace))
                    .text_color(Color32::WHITE),
            );
            ui.separator();

            if ui.button("update").clicked() {
                let start = u16::from_str_radix(&app.memory_dump_range_input.0, 16);
                let end = u16::from_str_radix(&app.memory_dump_range_input.1, 16);

                if let (Ok(start), Ok(end)) = (start, end) {
                    app.memory_dump_range = (start, end);
                } else {
                    app.memory_dump_range_input = (
                        format!("{:04X}", app.memory_dump_range.0),
                        format!("{:04X}", app.memory_dump_range.1),
                    );
                }
            }
        });
    }
}

pub struct MemoryDump;

impl View for MemoryDump {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, app: &mut App) {
        let (start, end) = app.memory_dump_range;
        let dump = memory_hexdump(app.emulator.memory.mem, start, end);

        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for i in dump {
                    ui.horizontal(|ui| {
                        ui.add(Label::new(
                            RichText::new(format!("0x{:04X}", i.0))
                                .strong()
                                .color(Color32::from_rgb(50, 180, 80))
                                .monospace(),
                        ));
                        ui.separator();
                        for j in i.1 {
                            ui.add(Label::new(RichText::new(format!(" {:02X}", j)).monospace()));
                        }
                        ui.separator();
                        for j in i.2 {
                            ui.add(Label::new(
                                RichText::new(j)
                                    .color(Color32::from_rgb(50, 180, 80))
                                    .monospace(),
                            ));
                        }
                        ui.separator();
                    });
                    ui.separator();
                }
            });
    }
}
