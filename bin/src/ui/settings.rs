use crate::{app::App, View};
use eframe::egui::*;

pub struct SettingsUi;

impl View for SettingsUi {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                let settings = &mut app.settings;
                let visibility = &mut app.window_visibility;
                ui.heading("Settings");
                ui.checkbox(&mut settings.reactive_mode, "Reactive Mode")
                    .on_hover_text("If unchecked (Continuous mode), CPU usage may increase.");
                ui.checkbox(&mut settings.panel_ui, "Panel UI");
                ui.separator();
                ui.checkbox(&mut visibility.display, "Show Display");
                ui.checkbox(&mut visibility.source, "Show Source Input");
                ui.checkbox(&mut visibility.memory_dump, "Show Memory Dump");
                ui.checkbox(&mut visibility.debugger, "Show Debugger");
            });
    }
}
