use crate::{app::App, View};
use assembler::disassemble;
use eframe::egui::*;
use emulator::memory::MemoryBus;

pub struct DisassemblerUi;

impl View for DisassemblerUi {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        if ui.button("disassemble").clicked() {
            let sliced = app.emulator.memory.slice(0x8000..0xFFFF);
            app.disassembled = match disassemble(sliced) {
                Ok(disassembled) => disassembled,
                Err(e) => {
                    app.error = Some(e.to_string());
                    return;
                }
            };
        }

        ui.separator();

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
