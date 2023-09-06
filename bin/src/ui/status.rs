use crate::{app::App, View};
use eframe::egui::*;
use emulator::registers::Registers;

pub struct StatusBar;

impl View for StatusBar {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.horizontal(|ui| {
                let r @ Registers {
                    a, x, y, p, sp, pc, ..
                } = &app.emulator.registers;

                ui.add(Label::new(
                    RichText::new(format!("A: {a:02X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!("X: {x:02X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!("Y: {y:02X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!("P: {p:02X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!("SP: {sp:02X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!("PC: {pc:04X}")).strong().monospace(),
                ));
                ui.separator();
                ui.add(Label::new(
                    RichText::new(format!(
                        "{} {} - {}  {} {} {} {} (NV-B DIZC)",
                        r.get_flag_negative() as u8,
                        r.get_flag_overflow() as u8,
                        r.get_flag_break() as u8,
                        r.get_flag_decimal() as u8,
                        r.get_flag_interrupt_disable() as u8,
                        r.get_flag_zero() as u8,
                        r.get_flag_carry() as u8
                    ))
                    .strong()
                    .monospace(),
                ));
            });
        });
    }
}
