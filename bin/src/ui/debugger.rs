use crate::{app::App, View, DEBUG_OUTPUT, DEBUG_UPDATE};
use eframe::egui::*;
use emulator::{CpuDebugger, DebugKind};

pub struct DebuggerUi;

impl View for DebuggerUi {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.horizontal(|ui| {
            if ui.button("step").clicked() {
                app.emulator.step();
            }

            if ui.button("clear").clicked() {
                unsafe {
                    DEBUG_OUTPUT.clear();
                }
            }
        });
    }
}

pub struct DebuggerOutput;

impl View for DebuggerOutput {
    fn ui(&mut self, ui: &mut Ui, _: &mut App) {
        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let messages = unsafe { DEBUG_OUTPUT.clone() };
                    for debug in messages {
                        ui.horizontal(|ui| {
                            let time = Label::new(
                                RichText::new(debug.0)
                                    .monospace()
                                    .color(Color32::LIGHT_BLUE),
                            );
                            let color = match debug.2 {
                                DebugKind::Info => Color32::WHITE,
                                DebugKind::Warn => Color32::YELLOW,
                                DebugKind::Error => Color32::RED,
                            };
                            let msg = Label::new(RichText::new(debug.1).monospace().color(color));

                            if unsafe { DEBUG_UPDATE } {
                                ui.add(time);
                                ui.add(msg);
                                ui.scroll_to_cursor(Some(Align::Center));
                            } else {
                                ui.add(time);
                                ui.add(msg);
                            }
                        });
                    }
                });
            });

        if unsafe { DEBUG_UPDATE } {
            unsafe {
                DEBUG_UPDATE = false;
            }
        }
    }
}
