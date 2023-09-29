use eframe::egui;
use emulator::DebugKind;
use std::sync::atomic::AtomicBool;

pub mod app;
pub mod ui;

/// (time, message, kind)
pub static mut DEBUG_OUTPUT: Vec<(String, String, DebugKind)> = Vec::new();
pub static mut DEBUG_UPDATE: bool = false;
pub static IS_RUNNING: AtomicBool = AtomicBool::new(false);

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, app: &mut app::App);
}
