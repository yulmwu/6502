use crate::{app::App, View};
use assembler::Assembler;
use eframe::egui::{menu::menu_button, *};

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

            menu_button(
                ui,
                "File",
                |ui| {
                    if ui.button("load rom (TODO)").clicked() {}
                },
            );

            ui.separator();

            if ui.button("Run").clicked() {
                app.is_running = true;
            }

            if ui.button("Halt").clicked() {
                app.is_running = false;
                app.emulator.debug("Halted");
            }

            if ui.button("Load").clicked() {
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

            if ui.button("Reset").clicked() {
                app.emulator.reset();
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
