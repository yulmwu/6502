use crate::{app::App, View, IS_RUNNING};
use assembler::{disassemble, Assembler};
use eframe::egui::{menu::menu_button, *};
use std::{fs, sync::atomic::Ordering};

pub struct MenuBar;

impl View for MenuBar {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.horizontal(|ui| {
            let style = ui.style_mut();
            style.spacing.button_padding = vec2(2.0, 0.0);
            style.visuals.widgets.active.bg_stroke = Stroke::NONE;
            style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
            style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;

            menu_button(ui, "File", |ui| {
                if ui.button("save as binary").clicked() {
                    let dialog = tinyfiledialogs::save_file_dialog("Save as binary", "binary.bin");

                    if let Some(path) = dialog {
                        let binary = Assembler::new(&app.source_input).assemble();
                        match binary {
                            Ok(binary) => {
                                if let Err(e) = fs::write(path, binary) {
                                    app.error = Some(e.to_string());
                                }
                            }
                            Err(e) => app.error = Some(e.to_string()),
                        }
                    }
                }
                if ui.button("save as source code").clicked() {
                    let source = app.source_input.clone();

                    let dialog = tinyfiledialogs::save_file_dialog_with_filter(
                        "Save as source code",
                        "source.asm",
                        &["*.asm"],
                        "Assembly source code (*.asm)",
                    );

                    if let Some(path) = dialog {
                        if let Err(e) = fs::write(path, source) {
                            app.error = Some(e.to_string());
                        }
                    }
                }
                ui.separator();
                if ui.button("load binary").clicked() {
                    let dialog = tinyfiledialogs::open_file_dialog("Load binary", "", None);

                    if let Some(path) = dialog {
                        let binary = fs::read(path);
                        match binary {
                            Ok(binary) => {
                                app.emulator.reset();
                                app.emulator.load(&binary);

                                app.source_input = match disassemble(&binary) {
                                    Ok(disassembled) => disassembled
                                        .iter()
                                        .map(|(_, i)| i.clone())
                                        .collect::<Vec<_>>()
                                        .join("\n"),
                                    Err(e) => {
                                        app.error = Some(e.to_string());
                                        return;
                                    }
                                };
                            }
                            Err(e) => app.error = Some(e.to_string()),
                        }
                    }
                }
                if ui.button("load source code").clicked() {
                    let dialog = tinyfiledialogs::open_file_dialog(
                        "Load source code",
                        "",
                        Some((&["*.asm"], "Assembly source code (*.asm)")),
                    );

                    if let Some(path) = dialog {
                        let source = fs::read_to_string(path);
                        match source {
                            Ok(source) => {
                                app.source_input = source;
                                app.emulator.reset();
                            }
                            Err(e) => app.error = Some(e.to_string()),
                        }
                    }
                }
            });

            menu_button(ui, "Window", |ui| {
                if ui
                    .button(if app.is_open_disassembler_window {
                        "Close Disassembler"
                    } else {
                        "Open Disassembler"
                    })
                    .clicked()
                {
                    app.is_open_disassembler_window = !app.is_open_disassembler_window;
                }
            });

            ui.separator();

            if ui.button("Run").clicked() {
                IS_RUNNING.store(true, Ordering::Relaxed);
                app.error = None;
            }

            if ui.button("Halt").clicked() {
                IS_RUNNING.store(false, Ordering::Relaxed);
                app.emulator.debug("Halted");
            }

            if ui.button("Load").clicked() {
                let src = match Assembler::new(&app.source_input).assemble() {
                    Ok(src) => src,
                    Err(e) => {
                        app.error = Some(e.to_string());
                        return;
                    }
                };

                app.emulator.reset();
                app.emulator.load(&src);
                app.error = None;
            }

            if ui.button("Reset").clicked() {
                app.emulator.reset();
                app.error = None;
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                menu_button(ui, "About", |ui| {
                    ui.heading("6502 Emulator & Assembler");
                    ui.horizontal(|ui| {
                        ui.add(Label::new("by ky0422"));
                        ui.separator();
                        ui.add(Hyperlink::from_label_and_url(
                            "Source Code",
                            "https://github.com/ky0422/6502",
                        ));
                    });
                });
            });
        });
    }
}
