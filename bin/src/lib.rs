use eframe::egui;

pub mod app;
pub mod ui;

/// (time, message)
pub static mut DEBUG_OUTPUT: (String, String) = (String::new(), String::new());
pub static mut DEBUG_UPDATE: bool = false;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, app: &mut app::App);
}
